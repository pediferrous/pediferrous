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

/// Comment
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

/// Comment
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
