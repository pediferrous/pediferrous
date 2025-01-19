use proc_macro::TokenStream;

mod name;

/// Generate one or more `const Name<&'static [u8]>` values from the given identifiers. Identifiers
/// should be specified in upper snake case and will be converted to pascal-cased PDF names. All
/// leters will be replaced with their lowercase equivalents, except the first letter and any
/// letter preceded by an underscore. For example:
///
/// * `IDENT` -> `Ident`
/// * `SECOND_IDENT` -> `SecondIdent`
///
/// # Example
///
/// ```ignore
/// use pdfgen_macros::const_names;
///
/// pub struct SomeStruct;
///
/// impl SomeStruct {
///     const_names!(TYPE, SUBTYPE, MEDIA_BOX);
///
///     // expands to
///     const TYPE: Name<&'static [u8]> = Name::from_static(b"Type");
///     const SUBTYPE: Name<&'static [u8]> = Name::from_static(b"Subtype");
///     const MEDIA_BOX: Name<&'static [u8]> = Name::from_static(b"MediaBox");
/// }
/// ```
#[proc_macro]
pub fn const_names(token_stream: TokenStream) -> TokenStream {
    name::const_names(token_stream)
}
