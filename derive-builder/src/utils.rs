use syn::{punctuated::Punctuated, token::Comma, DeriveInput};

/// Get the inner option type `T` from `Option<T>`.
pub fn inner_option_t(ty: &syn::Type) -> Option<&syn::Type> {
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
///
/// ```rust
/// # use derive_builder::Builder;
/// #[derive(Builder)]
/// struct Command {
///   executable: String,
///   args: Vec<String>,
///   env: Vec<String>,
///   current_dir: Option<String>
/// }
/// ```
/// Note: The above struct will return the following fields:
///
/// - `executable: String,`
/// - `args: Vec<String>,`
/// - `env: Vec<String>,`
/// - `current_dir: Option<String>,`
///
/// ...as a `syn::punctuated::Punctuated<syn::Field, syn::token::Comma>`
/// type.
pub fn get_struct_fields(ast: &DeriveInput) -> &Punctuated<syn::Field, Comma> {
  if let syn::Data::Struct(syn::DataStruct {
    fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
    ..
  }) = ast.data
  {
    named
  } else {
    unimplemented!()
  }
}

/// Constructs a struct builder inform of `MyStructBuilder`.
///
/// Note: If the caller's struct is `Command`, the builder identifier
///       will be `CommandBuilder`.
///
/// Return value in this case will be:
///   (`Command`, `CommandBuilder`)
pub fn get_struct_and_builder_ident(
  ast: &DeriveInput,
) -> (&syn::Ident, syn::Ident) {
  // Command - Identifier.
  let name = &ast.ident;

  // Construct - CommandBuilder identifier.
  let bname = format!("{}Builder", name);
  let bident = syn::Ident::new(&bname, name.span());

  (name, bident)
}
