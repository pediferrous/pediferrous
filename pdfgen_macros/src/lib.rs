use proc_macro::{Span, TokenStream};
use syn::{
    parse::Parse, parse_macro_input, punctuated::Punctuated, Attribute, Ident, LitByteStr, Token,
    Visibility,
};

/// Represents the input to the `const_names` macro.
struct ConstName {
    docs: Vec<Attribute>,
    visibility: Visibility,
    name: Ident,
}

impl Parse for ConstName {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let docs: Vec<Attribute> = input.call(Attribute::parse_outer)?;
        let visibility: Visibility = input.parse()?;
        let name: Ident = input.parse()?;

        Ok(ConstName {
            docs,
            visibility,
            name,
        })
    }
}

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
    let const_name =
        parse_macro_input!(token_stream with Punctuated<ConstName, Token![,]>::parse_terminated);

    let mut ts = TokenStream::new();
    for cn in const_name {
        let docs = cn.docs;
        let visibility = cn.visibility;
        let name = cn.name;

        let name_byte_str = create_pdf_style_byte_literal(&name);

        let expanded = quote::quote! {
            #(#docs)*
            #visibility const #name: Name<&'static [u8]> = Name::from_static(#name_byte_str);
        };
        ts.extend(TokenStream::from(expanded));
    }

    ts
}

/// Helper function converting uppercase literals to LitByteStr in PascalCase format
fn create_pdf_style_byte_literal(name: &Ident) -> LitByteStr {
    let name = name.to_string();

    let mut prev_was_underline = false;
    let mut literal = String::with_capacity(name.len());
    for (i, ch) in name.chars().enumerate() {
        match (i, prev_was_underline) {
            (0, _) | (_, true) => {
                for ch in ch.to_uppercase() {
                    literal.push(ch);
                }
                prev_was_underline = false;
            }
            _ if ch == '_' => {
                prev_was_underline = true;
                continue;
            }
            _ => {
                for ch in ch.to_lowercase() {
                    literal.push(ch);
                }
            }
        }
    }

    syn::LitByteStr::new(literal.as_bytes(), Span::call_site().into())
}
