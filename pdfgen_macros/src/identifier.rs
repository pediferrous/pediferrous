use proc_macro2::{Span, TokenStream};
use syn::{
    Attribute, Ident, LitByteStr, Token, Visibility, parse::Parse, parse_macro_input,
    punctuated::Punctuated,
};

/// Represents the input to the `const_identifiers` macro.
struct ConstIdentifier {
    docs: Vec<Attribute>,
    visibility: Visibility,
    identifier: Ident,
    custom_lit: Option<LitByteStr>,
}

impl Parse for ConstIdentifier {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let docs: Vec<Attribute> = input.call(Attribute::parse_outer)?;
        let visibility: Visibility = input.parse()?;
        let identifier: Ident = input.parse()?;

        let mut custom_lit = None;

        if input.parse::<Token![:]>().is_ok() {
            custom_lit = input.parse()?;
        }

        Ok(ConstIdentifier {
            docs,
            visibility,
            identifier,
            custom_lit,
        })
    }
}

pub(crate) fn const_identifiers(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let const_name = parse_macro_input!(token_stream with Punctuated<ConstIdentifier, Token![,]>::parse_terminated);

    let mut ts = TokenStream::new();
    for cn in const_name {
        let docs = cn.docs;
        let visibility = cn.visibility;
        let identifier = cn.identifier;

        let name_byte_str = cn
            .custom_lit
            .unwrap_or_else(|| create_pdf_style_byte_literal(&identifier));

        let expanded = quote::quote! {
            #(#docs)*
            #visibility const #identifier: Identifier<&'static [u8]> = Identifier::from_static(#name_byte_str);
        };

        ts.extend(expanded);
    }

    ts.into()
}

/// Helper function converting uppercase literals to LitByteStr in PascalCase format
fn create_pdf_style_byte_literal(identifier: &Ident) -> LitByteStr {
    let identifier = identifier.to_string();

    let mut prev_was_underline = false;
    let mut literal = String::with_capacity(identifier.len());
    for (i, ch) in identifier.chars().enumerate() {
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

    syn::LitByteStr::new(literal.as_bytes(), Span::call_site())
}

#[cfg(test)]
mod tests {
    use proc_macro2::{Ident, Span};
    use syn::LitByteStr;

    use super::create_pdf_style_byte_literal;

    #[test]
    fn single_word_name() {
        let span = Span::call_site();
        let ident = Ident::new("NAME", span);

        let identifier = create_pdf_style_byte_literal(&ident);

        assert_eq!(identifier, LitByteStr::new(b"Name", span));
    }

    #[test]
    fn double_word_name() {
        let span = Span::call_site();
        let ident = Ident::new("NAME_TWO", span);

        let identifier = create_pdf_style_byte_literal(&ident);

        assert_eq!(identifier, LitByteStr::new(b"NameTwo", span));
    }
}
