use syn::{Field, Ident, Type};

#[derive(Clone)]
pub(crate) struct Fragment {
    pub(crate) ident: Ident,
    pub(crate) fields: Vec<Field>,
}

#[derive(Clone)]
pub(crate) struct FieldOrigin {
    pub(crate) span: proc_macro2::Span,
    pub(crate) description: String,
}

pub(crate) enum ComposeEntry {
    Flatten(Ident),
    Nested {
        field_name: Ident,
        colon_token: syn::Token![:],
        ty: Box<Type>,
    },
    SelfFields {
        self_token: syn::Token![self],
    },
}
