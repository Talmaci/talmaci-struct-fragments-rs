use proc_macro2::Span;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Attribute, Meta, Result, Token, Type};

use crate::model::ComposeEntry;

pub(crate) fn is_helper(attribute: &Attribute, name: &str) -> bool {
    attribute.path().is_ident(name)
}

pub(crate) fn validate_fragment(attribute: &Attribute) -> Result<()> {
    match &attribute.meta {
        Meta::Path(_) => Ok(()),
        _ => Err(syn::Error::new_spanned(attribute, "`fragment` does not accept arguments")),
    }
}

pub(crate) fn parse_compose(attribute: &Attribute) -> Result<Vec<ComposeEntry>> {
    let entries = attribute
        .parse_args_with(Punctuated::<ComposeEntry, Token![,]>::parse_terminated)?
        .into_iter()
        .collect::<Vec<_>>();

    if entries.is_empty() {
        return Err(syn::Error::new_spanned(attribute, "`compose` requires at least one entry"));
    }

    Ok(entries)
}

pub(crate) fn validate_compose(entries: &[ComposeEntry]) -> Result<()> {
    let mut first_self: Option<Span> = None;

    for entry in entries {
        let ComposeEntry::SelfFields { self_token } = entry else {
            continue;
        };

        if let Some(first_span) = first_self {
            let mut error = syn::Error::new(self_token.span, "`self` may appear at most once in `compose`");
            error.combine(syn::Error::new(first_span, "first `self` entry is here"));
            return Err(error);
        }

        first_self = Some(self_token.span);
    }

    Ok(())
}

impl Parse for ComposeEntry {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        if input.peek(Token![self]) {
            let self_token = input.parse()?;
            if !input.is_empty() && !input.peek(Token![,]) {
                return Err(input.error("`self` must be a standalone `compose` entry"));
            }
            return Ok(Self::SelfFields { self_token });
        }

        let field_or_fragment = input.parse()?;
        if input.peek(Token![:]) {
            return Ok(Self::Nested {
                field_name: field_or_fragment,
                colon_token: input.parse()?,
                ty: Box::new(input.parse::<Type>()?),
            });
        }

        if input.peek(Token![=]) {
            return Err(syn::Error::new(
                input.span(),
                "nested composition uses `field_name: Type`, not `field_name = Type`",
            ));
        }

        Ok(Self::Flatten(field_or_fragment))
    }
}
