use proc_macro2::TokenTree;
use quote::format_ident;
use syn::{
  punctuated::Punctuated, token::Comma, DeriveInput, GenericArgument, Ident,
  PathArguments, Type,
};

/// Get the inner option type `T` from `Option<T>`.
pub fn inner_type_t<'a>(ty_ident: &str, ty: &'a Type) -> Option<&'a Type> {
  if let Type::Path(ref p) = ty {
    if p.path.segments.len() != 1 || p.path.segments[0].ident != ty_ident {
      return None;
    }

    if let PathArguments::AngleBracketed(ref inner_ty) =
      p.path.segments[0].arguments
    {
      if inner_ty.args.len() != 1 {
        return None;
      }

      let inner_ty = inner_ty.args.first().unwrap();
      if let GenericArgument::Type(ref t) = inner_ty {
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
#[inline]
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
#[inline]
pub fn get_struct_and_builder_ident(ast: &DeriveInput) -> (&Ident, Ident) {
  // Command - Identifier.
  let name = &ast.ident;

  // Construct - CommandBuilder identifier.
  let bident = format_ident!("{}Builder", name);

  (name, bident)
}

/// Extract the value of an attribute macro `#[attribute(key = "...")]`.
///
/// #[builder(each = "...")] - extract_attrs_value(f, "builder", "each")
pub fn extract_attrs_value(
  f: &syn::Field,
  attribute: &str,
  key: &str,
) -> Option<Ident> {
  for attr in &f.attrs {
    if attr.path.segments.len() == 1 && attr.path.segments[0].ident == attribute
    {
      if let Some(TokenTree::Group(g)) = attr.tokens.clone().into_iter().next()
      {
        // #[builder(each = "...")]
        let mut tokens = g.stream().into_iter();
        // Check if the first token is `key`.
        match tokens.next().unwrap() {
          TokenTree::Ident(ident) if ident == key => {}
          tt => panic!("Expected `each` but got {:?}", tt),
        }
        // Check if the next token is `=`.
        match tokens.next().unwrap() {
          TokenTree::Punct(punct) if punct.as_char() == '=' => {}
          tt => panic!("Expected `=` but got {:?}", tt),
        }
        // ...and the next token is the value to be extracted.
        let arg = match tokens.next().unwrap() {
          TokenTree::Literal(lit) => lit,
          tt => panic!("Expected string but got {:?}", tt),
        };
        // Crate a new `Ident` from the value.
        match syn::Lit::new(arg) {
          syn::Lit::Str(s) => {
            let arg = Ident::new(&s.value(), s.span());
            return Some(arg);
            // let inner_ty = inner_type_t("Vec", &f.ty);
          }
          str_lit => panic!("Expected string literal but got {:?}", str_lit),
        }
      }
    }
  }
  None
}
