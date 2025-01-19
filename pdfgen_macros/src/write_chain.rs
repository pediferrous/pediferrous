use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Expr, Pat, Token,
};

/// Represents a sequence of expressions inside of a `for` loop in `write_chain` macro.
///
/// For example:
///
/// ```rust
/// for entry in &self.entries {
///     entry.write(writer),
///     writer.write(b",\n"),
///     /// ...
/// }
/// ```
// Fields are mapped as following:
// for $pattern in $expr {
//     comma delimited expressions
// }
struct LoopWrite {
    pattern: Pat,
    loop_expr: Expr,
    expressions: Vec<Expr>,
}

impl Parse for LoopWrite {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _ = input.parse::<Token![for]>()?;
        let pattern = Pat::parse_multi_with_leading_vert(input)?;

        let _ = input.parse::<Token![in]>()?;
        let loop_expr = input.parse()?;

        let body;
        syn::braced!(body in input);

        let expressions = Punctuated::<Expr, Token![,]>::parse_terminated(&body)?
            .into_iter()
            .collect();

        Ok(LoopWrite {
            pattern,
            loop_expr,
            expressions,
        })
    }
}

impl ToTokens for LoopWrite {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self {
            pattern,
            loop_expr,
            expressions,
        } = self;

        let output = quote! {
            for #pattern in #loop_expr {
                #(
                written += #expressions?;
                )*
            }
        };

        tokens.extend(output);
    }
}

/// Represents a conditional expression rendered in `write_chain` macro. Both pattern matching and
/// regular if expressions are supported:
///
/// ```rust
/// // Regular if expression syntax.
/// if x == 42 {
///     writer.write(x),
///     writer.wrtite(/* ... */),
///     /// ...
/// }
///
/// // Pattern match syntax
/// if let Some(value) = optional {
///     writer.write(value),
///     writer.write(/* ... */),
///     /// ...
/// }
/// ```
// Fields in case of a normal if expression:
// if $if_expr {
//     comma delimited expressions
// }
//
// Fields in case of a pattern match:
// if let $pattern = $if_expr {
//     comma delimited expressions
// }
struct IfWrite {
    pattern: Option<Pat>,
    if_expr: Expr,
    expressions: Vec<Expr>,
}

impl Parse for IfWrite {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _ = input.parse::<Token![if]>()?;
        let mut pattern = None;

        if input.parse::<Token![let]>().is_ok() {
            pattern = Some(Pat::parse_multi_with_leading_vert(input)?);
            let _ = input.parse::<Token![=]>()?;
        }

        let if_expr = input.parse()?;

        let body;
        syn::braced!(body in input);

        let expressions = Punctuated::<Expr, Token![,]>::parse_terminated(&body)?
            .into_iter()
            .collect();

        Ok(IfWrite {
            pattern,
            if_expr,
            expressions,
        })
    }
}

impl ToTokens for IfWrite {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self {
            pattern,
            if_expr,
            expressions,
        } = self;

        let header = match pattern {
            Some(pattern) => quote!(if let #pattern = #if_expr),
            None => quote!(if #if_expr),
        };

        let body = quote! {
            {
                #(
                written += #expressions?;
                )*
            }
        };

        tokens.extend(header);
        tokens.extend(body);
    }
}

enum WriteStatement {
    Single(Expr),
    Loop(LoopWrite),
    If(IfWrite),
}

impl quote::ToTokens for WriteStatement {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            WriteStatement::Single(single) => {
                tokens.extend(quote!(written += #single?;));
            }
            WriteStatement::Loop(loop_expr) => loop_expr.to_tokens(tokens),
            WriteStatement::If(if_expr) => if_expr.to_tokens(tokens),
        }
    }
}

impl Parse for WriteStatement {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Token![if]) {
            let if_write = input.parse()?;
            Ok(Self::If(if_write))
        } else if input.peek(Token![for]) {
            let loop_write = input.parse()?;
            Ok(Self::Loop(loop_write))
        } else {
            let single = input.parse()?;
            Ok(Self::Single(single))
        }
    }
}

pub fn write_chain(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let write_statements = parse_macro_input!(token_stream with Punctuated<WriteStatement, Token![,]>::parse_terminated);

    generate_write_chain(write_statements.into_iter()).into()
}

fn generate_write_chain(writes: impl Iterator<Item = WriteStatement>) -> proc_macro2::TokenStream {
    quote! {
        {
        let mut written = 0;

        #(#writes)*

        written
        }
    }
}
