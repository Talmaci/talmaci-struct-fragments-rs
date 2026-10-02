use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Field, FieldMutability, Fields, Ident, Item, ItemMod, ItemStruct, Result, Visibility};

use crate::model::{ComposeEntry, FieldOrigin, Fragment};
use crate::parse::{is_helper, parse_compose, validate_compose, validate_fragment};

pub(crate) fn expand(mut module: ItemMod) -> Result<TokenStream> {
    let Some((_, items)) = &mut module.content else {
        return Err(syn::Error::new_spanned(&module, "`struct_fragments` requires an inline module"));
    };

    reject_helpers_on_non_structs(items)?;
    let fragments = collect_fragments(items)?;
    compose_destinations(items, &fragments)?;

    Ok(quote!(#module))
}

fn reject_helpers_on_non_structs(items: &[Item]) -> Result<()> {
    let mut errors = None;

    for item in items {
        if matches!(item, Item::Struct(_)) {
            continue;
        }

        for attribute in item_attrs(item) {
            if is_helper(attribute, "fragment") || is_helper(attribute, "compose") {
                combine_error(
                    &mut errors,
                    syn::Error::new_spanned(
                        attribute,
                        "struct fragment helper attributes may only be used on structs",
                    ),
                );
            }
        }
    }

    match errors {
        Some(error) => Err(error),
        None => Ok(()),
    }
}

fn collect_fragments(items: &[Item]) -> Result<HashMap<String, Fragment>> {
    let mut fragments: HashMap<String, Fragment> = HashMap::new();
    let mut errors = None;

    for item in items {
        let Item::Struct(item_struct) = item else {
            continue;
        };

        let fragment_attributes =
            item_struct.attrs.iter().filter(|attribute| is_helper(attribute, "fragment")).collect::<Vec<_>>();

        if fragment_attributes.is_empty() {
            continue;
        }

        if fragment_attributes.len() > 1 {
            combine_error(
                &mut errors,
                syn::Error::new_spanned(fragment_attributes[1], "duplicate `fragment` attribute"),
            );
        }

        for attribute in &fragment_attributes {
            if let Err(error) = validate_fragment(attribute) {
                combine_error(&mut errors, error);
            }
        }

        if item_struct.attrs.iter().any(|attribute| is_helper(attribute, "compose")) {
            combine_error(
                &mut errors,
                syn::Error::new_spanned(
                    &item_struct.ident,
                    "a struct cannot be both a fragment and a composition destination",
                ),
            );
        }

        if !item_struct.generics.params.is_empty() || item_struct.generics.where_clause.is_some() {
            combine_error(
                &mut errors,
                syn::Error::new_spanned(
                    &item_struct.generics,
                    "generic fragments are not supported in this version",
                ),
            );
            continue;
        }

        let Fields::Named(fields) = &item_struct.fields else {
            combine_error(
                &mut errors,
                syn::Error::new_spanned(&item_struct.fields, "fragments must be structs with named fields"),
            );
            continue;
        };

        let name = item_struct.ident.to_string();
        if let Some(previous) = fragments.get(&name) {
            let mut error =
                syn::Error::new_spanned(&item_struct.ident, format!("duplicate fragment definition `{name}`"));
            error.combine(syn::Error::new_spanned(&previous.ident, "first fragment definition is here"));
            combine_error(&mut errors, error);
            continue;
        }

        fragments.insert(
            name,
            Fragment {
                ident: item_struct.ident.clone(),
                fields: fields.named.iter().cloned().collect(),
            },
        );
    }

    match errors {
        Some(error) => Err(error),
        None => Ok(fragments),
    }
}

fn compose_destinations(items: &mut [Item], fragments: &HashMap<String, Fragment>) -> Result<()> {
    let mut errors = None;

    for item in items {
        let Item::Struct(item_struct) = item else {
            continue;
        };

        let fragment_marker_count =
            item_struct.attrs.iter().filter(|attribute| is_helper(attribute, "fragment")).count();
        let compose_attributes = item_struct
            .attrs
            .iter()
            .filter(|attribute| is_helper(attribute, "compose"))
            .cloned()
            .collect::<Vec<_>>();

        item_struct
            .attrs
            .retain(|attribute| !is_helper(attribute, "fragment") && !is_helper(attribute, "compose"));

        if fragment_marker_count > 0 || compose_attributes.is_empty() {
            continue;
        }

        if compose_attributes.len() > 1 {
            combine_error(
                &mut errors,
                syn::Error::new_spanned(&compose_attributes[1], "duplicate `compose` attribute"),
            );
            continue;
        }

        let requested = match parse_compose(&compose_attributes[0]) {
            Ok(requested) => requested,
            Err(error) => {
                combine_error(&mut errors, error);
                continue;
            }
        };

        if let Err(error) = validate_compose(&requested) {
            combine_error(&mut errors, error);
            continue;
        }

        if let Err(error) = compose_struct(item_struct, &requested, fragments) {
            combine_error(&mut errors, error);
        }
    }

    match errors {
        Some(error) => Err(error),
        None => Ok(()),
    }
}

fn compose_struct(
    destination: &mut ItemStruct,
    requested: &[ComposeEntry],
    fragments: &HashMap<String, Fragment>,
) -> Result<()> {
    let Fields::Named(destination_fields) = &destination.fields else {
        return Err(syn::Error::new_spanned(
            &destination.fields,
            "composition destinations must be structs with named fields",
        ));
    };

    let mut errors = None;
    let local_fields = destination_fields.named.iter().cloned().collect::<Vec<_>>();
    let mut output_fields = Vec::new();
    let mut seen_fields: HashMap<String, FieldOrigin> = HashMap::new();
    let mut placed_local_fields = false;

    for entry in requested {
        match entry {
            ComposeEntry::Flatten(fragment_name) => {
                append_fragment_fields(
                    fragment_name,
                    &destination.ident,
                    fragments,
                    &mut output_fields,
                    &mut seen_fields,
                    &mut errors,
                );
            }
            ComposeEntry::Nested {
                field_name,
                colon_token,
                ty,
            } => {
                let field = nested_field(field_name, colon_token, ty);
                record_field(
                    &field,
                    &destination.ident,
                    format!("nested field `{field_name}`"),
                    &mut seen_fields,
                    &mut errors,
                );
                output_fields.push(field);
            }
            ComposeEntry::SelfFields { .. } => {
                append_local_fields(
                    &local_fields,
                    &destination.ident,
                    &mut output_fields,
                    &mut seen_fields,
                    &mut errors,
                );
                placed_local_fields = true;
            }
        }
    }

    if !placed_local_fields {
        append_local_fields(
            &local_fields,
            &destination.ident,
            &mut output_fields,
            &mut seen_fields,
            &mut errors,
        );
    }

    if let Some(error) = errors {
        return Err(error);
    }

    let Fields::Named(destination_fields) = &mut destination.fields else {
        unreachable!("destination field shape was checked above");
    };
    destination_fields.named = output_fields.into_iter().collect();
    Ok(())
}

fn append_fragment_fields(
    fragment_name: &Ident,
    destination: &Ident,
    fragments: &HashMap<String, Fragment>,
    output_fields: &mut Vec<Field>,
    seen_fields: &mut HashMap<String, FieldOrigin>,
    errors: &mut Option<syn::Error>,
) {
    let name = fragment_name.to_string();
    let Some(fragment) = fragments.get(&name) else {
        combine_error(errors, syn::Error::new_spanned(fragment_name, format!("unknown fragment `{name}`")));
        return;
    };

    for field in &fragment.fields {
        record_field(field, destination, format!("fragment `{}`", fragment.ident), seen_fields, errors);
        output_fields.push(field.clone());
    }
}

fn append_local_fields(
    local_fields: &[Field],
    destination: &Ident,
    output_fields: &mut Vec<Field>,
    seen_fields: &mut HashMap<String, FieldOrigin>,
    errors: &mut Option<syn::Error>,
) {
    for field in local_fields {
        record_field(field, destination, "local fields".to_owned(), seen_fields, errors);
        output_fields.push(field.clone());
    }
}

fn nested_field(field_name: &Ident, colon_token: &syn::Token![:], ty: &syn::Type) -> Field {
    Field {
        attrs: Vec::new(),
        vis: Visibility::Public(syn::token::Pub {
            span: field_name.span(),
        }),
        mutability: FieldMutability::None,
        ident: Some(field_name.clone()),
        colon_token: Some(*colon_token),
        ty: ty.clone(),
    }
}

fn record_field(
    field: &syn::Field,
    destination: &Ident,
    origin_description: String,
    seen_fields: &mut HashMap<String, FieldOrigin>,
    errors: &mut Option<syn::Error>,
) {
    let Some(ident) = &field.ident else {
        return;
    };
    let name = ident.to_string();
    let origin = FieldOrigin {
        span: ident.span(),
        description: origin_description,
    };

    if let Some(previous) = seen_fields.get(&name) {
        let mut error = syn::Error::new(
            ident.span(),
            format!("duplicate field `{name}` in composed struct `{destination}`"),
        );
        error.combine(syn::Error::new(
            previous.span,
            format!("first defined here (from {})", previous.description),
        ));
        combine_error(errors, error);
    } else {
        seen_fields.insert(name, origin);
    }
}

fn item_attrs(item: &Item) -> &[syn::Attribute] {
    match item {
        Item::Const(item) => &item.attrs,
        Item::Enum(item) => &item.attrs,
        Item::ExternCrate(item) => &item.attrs,
        Item::Fn(item) => &item.attrs,
        Item::ForeignMod(item) => &item.attrs,
        Item::Impl(item) => &item.attrs,
        Item::Macro(item) => &item.attrs,
        Item::Mod(item) => &item.attrs,
        Item::Static(item) => &item.attrs,
        Item::Struct(item) => &item.attrs,
        Item::Trait(item) => &item.attrs,
        Item::TraitAlias(item) => &item.attrs,
        Item::Type(item) => &item.attrs,
        Item::Union(item) => &item.attrs,
        Item::Use(item) => &item.attrs,
        _ => &[],
    }
}

fn combine_error(errors: &mut Option<syn::Error>, error: syn::Error) {
    if let Some(errors) = errors {
        errors.combine(error);
    } else {
        *errors = Some(error);
    }
}

#[cfg(test)]
mod tests {
    use quote::ToTokens;
    use syn::{Fields, Item, ItemMod, parse_quote, parse2};

    use super::expand;

    #[test]
    fn copies_complete_fields_in_fragment_then_local_order() {
        let input: ItemMod = parse_quote! {
            mod models {
                #[fragment]
                struct Core {
                    /// Identifier documentation.
                    pub id: u64,
                }

                #[fragment]
                struct Audit {
                    #[cfg(any())]
                    pub created_at: u64,
                }

                #[compose(Core, Audit)]
                struct Model {
                    local: bool,
                }
            }
        };

        let expanded: ItemMod = parse2(expand(input).expect("module should expand"))
            .expect("expanded tokens should remain a module");
        let (_, items) = expanded.content.as_ref().expect("module should remain inline");
        let destination = items
            .iter()
            .find_map(|item| match item {
                Item::Struct(item) if item.ident == "Model" => Some(item),
                _ => None,
            })
            .expect("destination should remain in output");
        let Fields::Named(fields) = &destination.fields else {
            panic!("destination should have named fields");
        };
        let names = fields
            .named
            .iter()
            .map(|field| field.ident.as_ref().expect("named field").to_string())
            .collect::<Vec<_>>();

        assert_eq!(names, ["id", "created_at", "local"]);
        assert!(matches!(fields.named[0].vis, syn::Visibility::Public(_)));
        assert!(fields.named[0].attrs.iter().any(|attribute| attribute.path().is_ident("doc")));
        assert!(fields.named[1].attrs.iter().any(|attribute| attribute.path().is_ident("cfg")));

        let tokens = expanded.into_token_stream().to_string();
        assert!(!tokens.contains("fragment"));
        assert!(!tokens.contains("compose"));
    }

    #[test]
    fn walks_entries_left_to_right_and_places_locals_at_self() {
        let input: ItemMod = parse_quote! {
            mod models {
                #[fragment]
                struct A { a: u8 }

                #[fragment]
                struct B { b: u8 }

                #[compose(A, self, metadata: Option<String>, B)]
                struct Model {
                    local_one: u8,
                    local_two: u8,
                }
            }
        };

        let expanded: ItemMod = parse2(expand(input).expect("module should expand"))
            .expect("expanded tokens should remain a module");
        assert_eq!(field_names(&expanded, "Model"), ["a", "local_one", "local_two", "metadata", "b"]);

        let (_, items) = expanded.content.as_ref().expect("module should remain inline");
        let destination = items
            .iter()
            .find_map(|item| match item {
                Item::Struct(item) if item.ident == "Model" => Some(item),
                _ => None,
            })
            .expect("destination should remain in output");
        let Fields::Named(fields) = &destination.fields else {
            panic!("destination should have named fields");
        };
        assert!(matches!(fields.named[3].vis, syn::Visibility::Public(_)));
        assert_eq!(fields.named[3].ty.to_token_stream().to_string(), "Option < String >");
    }

    #[test]
    fn appends_locals_when_self_is_omitted() {
        let input: ItemMod = parse_quote! {
            mod models {
                #[fragment]
                struct A { a: u8 }

                #[fragment]
                struct B { b: u8 }

                #[compose(A, nested: String, B)]
                struct Model { local: u8 }
            }
        };

        let expanded: ItemMod = parse2(expand(input).expect("module should expand"))
            .expect("expanded tokens should remain a module");
        assert_eq!(field_names(&expanded, "Model"), ["a", "nested", "b", "local"]);
    }

    fn field_names(module: &ItemMod, struct_name: &str) -> Vec<String> {
        let (_, items) = module.content.as_ref().expect("module should remain inline");
        let destination = items
            .iter()
            .find_map(|item| match item {
                Item::Struct(item) if item.ident == struct_name => Some(item),
                _ => None,
            })
            .expect("requested struct should remain in output");
        let Fields::Named(fields) = &destination.fields else {
            panic!("destination should have named fields");
        };

        fields.named.iter().map(|field| field.ident.as_ref().expect("named field").to_string()).collect()
    }
}
