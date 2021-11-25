use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Get the inner option type `T` from `Option<T>`.
fn inner_option_t(ty: &syn::Type) -> Option<&syn::Type> {
  if let syn::Type::Path(ref p) = ty {
    if p.path.segments.len() != 1 || p.path.segments[0].ident != "Option" {
      return None;
    }

    if let syn::PathArguments::AngleBracketed(ref inner_ty) =
      p.path.segments[0].arguments
    {
      if inner_ty.args.len() != 1 {
        return None;
      }

      let inner_ty = inner_ty.args.first().unwrap();
      if let syn::GenericArgument::Type(ref t) = inner_ty {
        return Some(t);
      }
    }
  }
  None
}

/// Get struct fields from a parsed syntax tree (`syn::DeriveInput`).
fn get_struct_fields(
  ast: &DeriveInput,
) -> &syn::punctuated::Punctuated<syn::Field, syn::token::Comma> {
  let fields = if let syn::Data::Struct(syn::DataStruct {
    fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
    ..
  }) = ast.data
  {
    named
  } else {
    unimplemented!()
  };

  fields
}

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(input as DeriveInput);
  // println!("{:#?}", ast);

  // Command - Identifier.
  let name = &ast.ident;

  // Construct - CommandBuilder identifier.
  let bname = format!("{}Builder", name);
  let bident = syn::Ident::new(&bname, name.span());

  // `#bident` fileds.
  let fields = get_struct_fields(&ast);

  // `#bident` fields wrapped in `Options`.
  let optioned = fields.iter().map(|f| {
    let name = &f.ident;
    let ty = &f.ty;

    if inner_option_t(&ty).is_some() {
      quote! { #name: #ty }
    } else {
      quote! { #name: std::option::Option<#ty> }
    }
  });

  // `#bident` (CommandBuilder) methods.
  let methods = fields.iter().map(|f| {
    let name = &f.ident;
    let ty = &f.ty;

    if let Some(inner_ty) = inner_option_t(&ty) {
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
    use std::error::Error;

    struct #bident {
      #(#optioned,)*
    }

    impl #bident {

      #(#methods)*

      pub fn build(&self) -> Result<#name, Box<dyn Error>> {
        Ok(#name {
          #(#build_ret,)*
        })
      }
    }

    impl #name {
      fn builder() -> #bident {
        #bident {
          #(#builder_empty,)*
        }
      }
    }
  };

  ts.into()
}
