use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod utils;

use utils::*;

/// Derive macro for the Builder Design Pattern.
///
/// # Examples
///
/// ```rust
/// # use derive_builder::Builder;
///
/// # fn main() {
/// #[derive(Builder)]
/// pub struct Command {
///   executable: String,
///   // #[builder(each = "arg")]
///   args: Vec<String>,
///   // #[builder(each = "env")]
///   env: Vec<String>,
///   current_dir: Option<String>,
/// }
///
/// let command = Command::builder()
///     .executable("cargo".to_owned())
///     .args(vec!["build".to_owned(), "--release".to_owned()])
///     .env(vec![])
///     .build()
///     .unwrap();
///
/// // let command = Command::builder()
/// //   .executable("cargo".to_owned())
/// //   .arg("build".to_owned())
/// //   .arg("--release".to_owned())
/// //   .build()
/// //   .unwrap();
///
/// assert_eq!(command.executable, "cargo");
/// assert_eq!(command.args, &["build", "--release"]);
/// assert!(command.env.is_empty());
/// assert!(command.current_dir.is_none());
/// # }
/// ```
#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(input as DeriveInput);
  // println!("{:#?}", ast);

  // Get Command & CommandBuilder.
  let (name, bident) = get_struct_and_builder_ident(&ast);

  // `#bident` fileds.
  let fields = get_struct_fields(&ast);

  // `#bident` fields wrapped in `Options`.
  let optioned_fields = fields.iter().map(|f| {
    let name = &f.ident;
    let ty = &f.ty;

    if inner_option_t(ty).is_some() {
      quote! { #name: #ty }
    } else {
      quote! { #name: std::option::Option<#ty> }
    }
  });

  // let extend_methods = fields.iter().map(|f: &syn::Field| -> proc_macro2::TokenStream {
  //   let name = &f.ident;
  //   if !f.attrs.is_empty() {
  //     eprintln!("{:#?}", f.attrs);
  //   }
  //   quote! { #name: None }
  // });

  // `#bident` builder struct setter methods.
  let setter_methods = fields.iter().map(|f| {
    let name = &f.ident;
    let ty = &f.ty;

    if let Some(inner_ty) = inner_option_t(ty) {
      quote! {
        pub fn #name(&mut self, #name: #inner_ty) -> &mut Self {
          self.#name = Some(#name);
          self
        }
      }
    } else {
      quote! {
        pub fn #name(&mut self, #name: #ty) -> &mut Self {
          self.#name = Some(#name);
          self
        }
      }
    }
  });

  // `#bident::build()` return statement.
  let build_ret = fields.iter().map(|f| {
        let name = &f.ident;

        if inner_option_t(&f.ty).is_some() {
            quote! { #name: self.#name.clone() }
        } else {
            quote! { #name: self.#name.clone().ok_or(concat!(stringify!(#name), " is not set"))? }
        }
    });

  // Initialize fields with None.
  let builder_empty = fields.iter().map(|f| {
    let name = &f.ident;
    quote! { #name: None }
  });

  // Expanded TokenStream.
  let ts = quote! {
    /// The Builder struct has fields identical to the original struct excpet
    /// the field types are wrapped with `std::option::Option` where necessary.
    ///
    /// # Examples:
    ///
    /// ```rust
    /// # use derive_builder::Builder;
    /// #[derive(Builder)]
    /// struct Command {
    ///   executable: String,
    ///   args: Vec<String>,
    ///   env: Vec<String>,
    ///   current_dir: Option<String>,
    /// }
    ///
    /// // The above code will generate a `CommandBuilder` struct with a
    /// // signature similar to:
    /// struct CommandBuilder {
    ///   executable: std::option::Option<String>,
    ///   args: std::option::Option<Vec<String>>,
    ///   env: std::option::Option<Vec<String>>,
    ///   current_dir: Option<String>,
    /// }
    /// ```
    struct #bident {
      #(#optioned_fields,)*
    }

    // impl `CommandBuilder`.
    impl #bident {

      // List of setter methods.
      #(#setter_methods)*

      // #(#extend_methods)*

      /// `build` method on the builder struct runs the arguments and returns
      /// an object of original struct wrapped with `#[derive(Builder)]`.
      ///
      /// # Examples:
      ///
      /// ```rust
      /// # use derive_builder::Builder;
      /// pub struct Command {
      ///   executable: String,
      ///   args: Vec<String>,
      ///   env: Vec<String>,
      ///   current_dir: Option<String>,
      /// }
      ///
      /// let command = Command::builder()
      ///     .executable("cargo".to_owned())
      ///     .args(vec!["build".to_owned(), "--release".to_owned())
      ///     .env(vec![])
      ///     .build()
      ///     .unwrap();
      ///
      /// assert_eq!(command.executable, "cargo");
      /// assert_eq!(command.args, &["build", "release"]);
      /// assert!(command.current_dir.is_none());
      /// ```
      pub fn build(&self) -> Result<#name, Box<dyn std::error::Error>> {
        Ok(#name {
          #(#build_ret,)*
        })
      }
    }

    impl #name {
      /// Builder is called on the original struct wrapped with the derive-macro
      /// `#[derive(Builder)]` and it returns a type with the original struct's
      /// name combined with "Builder" e.g. struct Command; => CommandBuilder.
      ///
      /// The builder method initializes the structs value with a default value
      /// of `None` since the builder struct wraps the fields with `Option` where
      /// necessary.
      fn builder() -> #bident {
        #bident {
          #(#builder_empty,)*
        }
      }
    }
  };

  ts.into()
}
