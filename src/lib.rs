mod attribute;
mod refine;

use attribute::Attribute;
use proc_macro::TokenStream;
use quote::quote;
use refine::{into_optional, omit_or_pick};
use syn::{parse_macro_input, ItemStruct};

/// Create new Struct that omit the specified fields
/// # Examples
///
/// ```
/// use types_rs::omit;
///
/// // Create a new struct `NewS` with omitted field `b`
/// #[omit(NewS, [b])]
/// struct S {
///     a: i32,
///     b: &str,
/// }
/// // `NewS` will only have field `a`
/// let _ = NewS { a: 1 };
/// ```
///
/// When the fields dosn't specified, it will be same as the original struct
///
/// ```
/// use types_rs::omit;
///
/// #[omit(NewS)]
/// struct S<'a> {
///     a: i32,
///     b: &'a str,
/// }
/// // `NewS` is same as the original struct `S`
/// let _ = NewS { a: 1, b: "hello" };
/// ```
#[proc_macro_attribute]
pub fn omit(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as Attribute);
    let item = parse_macro_input!(item as ItemStruct);

    let item = omit_or_pick(attr, item, attribute::AttributeType::Omit);

    quote! {
        #item
    }
    .into()
}

/// Create new struct that pick the specified fields
/// # Examples
/// ```
/// use types_rs::pick;
///
/// // Create a new struct `NewS` with picked field `b`
/// #[pick(NewS, [b])]
/// struct S<'a> {
///    a: i32,
///    b: &'a str,
///    c: f64,
/// }
///
/// // `NewS` will only have field `b`
/// let _ = NewS { b: "hello" };
/// ```
///
/// When the fields dosn't specified, it will be empty struct
/// ```
/// use types_rs::pick;
///
/// #[pick(NewS)]
/// struct S {
///   a: i32,
///   b: f64,
/// }
///
/// // `NewS` will be empty struct
/// let _ = NewS {};
/// ```
#[proc_macro_attribute]
pub fn pick(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as Attribute);
    let item = parse_macro_input!(item as ItemStruct);

    let item = omit_or_pick(attr, item, attribute::AttributeType::Pick);

    quote! {
        #item
    }
    .into()
}

/// Change all fields to [`Option`] type
/// # Examples
/// ```
/// use types_rs::partial;
///
/// // Create a new struct `NewS` with all fields optional
/// #[partial(NewS)]
/// struct S<'a> {
///   a: i32,
///   b: &'a str,
///   c: f64,
/// }
///
/// // `NewS` will have all fields optional
/// let _ = NewS { a: Some(1), b: Some("hello"), c: Some(1.5) };
/// ```
///
/// When the field is already is [`Option`], it's no effect
///
/// ```
/// use types_rs::partial;
///
/// #[partial(NewS)]
/// struct S {
///  a: i32,
///  b: Option<f64>,
/// }
///
/// let _ = NewS { a: Some(1), b: Some(1.1) };
/// ```
#[proc_macro_attribute]
pub fn partial(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as Attribute);
    let mut item = parse_macro_input!(item as ItemStruct);

    item.ident = syn::Ident::new(&attr.name.to_string(), item.ident.span());

    let fields = item.fields.into_iter().map(into_optional).collect();
    item.fields = syn::Fields::Named(syn::FieldsNamed {
        brace_token: syn::token::Brace::default(),
        named: fields,
    });

    quote! {
        #item
    }
    .into()
}
