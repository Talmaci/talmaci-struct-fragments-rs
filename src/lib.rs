//! Compile-time reusable struct field composition for Rust.
//!
//! Use [`compose`] directly on a named-field struct to append public nested
//! fields. Use [`struct_fragments`] on an inline module when fields must be
//! flattened from reusable fragment definitions.

use proc_macro::TokenStream;

mod expand;
mod model;
mod parse;

/// Appends public nested fields to a standalone named-field struct.
///
/// Every entry has the form `field_name: Type`. Local fields stay first and
/// generated fields follow in entry order. Nested types are ordinary Rust types
/// and do not need to be fragments.
///
/// Standalone `compose` deliberately does not accept flattened fragment names or
/// `self`. Flattening requires the module-level [`struct_fragments`] macro so the
/// fragment syntax tree is available in the same invocation.
///
/// The original struct's attributes, derives, visibility, generics, where
/// clauses, documentation, and local fields are preserved.
///
/// # Example
///
/// ```
/// use talmaci_struct_fragments_rs::compose;
///
/// struct Metadata {
///     label: String,
/// }
///
/// #[compose(metadata: Option<Metadata>, tags: Vec<String>)]
/// struct User {
///     id: i32,
/// }
///
/// let user = User {
///     id: 1,
///     metadata: None,
///     tags: vec!["rust".to_owned()],
/// };
/// assert_eq!(user.tags, ["rust"]);
/// ```
#[proc_macro_attribute]
pub fn compose(args: TokenStream, input: TokenStream) -> TokenStream {
    let result = (|| {
        let args = syn::parse::<parse::ComposeArgs>(args)?;
        parse::validate_standalone_compose(&args.entries)?;
        let destination = syn::parse::<syn::ItemStruct>(input)?;
        expand::expand_standalone(destination, &args.entries)
    })();

    match result {
        Ok(output) => output.into(),
        Err(error) => error.into_compile_error().into(),
    }
}

/// Composes reusable field fragments within an inline module.
///
/// Fragment structs remain ordinary Rust structs in the expanded output. The
/// helper attributes `fragment` and `compose` are consumed by this macro.
///
/// Compose entries have three forms:
///
/// - `Fragment` flattens a same-module fragment's fields.
/// - `field_name: Type` creates one public nested field. `Type` can be any
///   ordinary Rust type and does not need to be a fragment.
/// - `self` inserts the destination's locally declared fields at that position.
///   Without `self`, local fields are appended after the compose entries.
///
/// Duplicate names across flattened, nested, and local fields are compile-time
/// errors. Only named-field structs are supported, and generic fragments are
/// intentionally rejected.
///
/// # Example
///
/// ```
/// use talmaci_struct_fragments_rs::struct_fragments;
///
/// #[struct_fragments]
/// mod models {
///     #[fragment]
///     pub struct UserCore {
///         pub id: i32,
///         pub name: String,
///     }
///
///     pub struct Profile {
///         pub bio: String,
///     }
///
///     #[compose(UserCore, self, profile: Profile)]
///     pub struct User {
///         pub active: bool,
///     }
/// }
///
/// let user = models::User {
///     id: 1,
///     name: "Ada".to_owned(),
///     active: true,
///     profile: models::Profile { bio: "Engineer".to_owned() },
/// };
/// assert_eq!(user.id, 1);
/// ```
#[proc_macro_attribute]
pub fn struct_fragments(args: TokenStream, input: TokenStream) -> TokenStream {
    if !args.is_empty() {
        return syn::Error::new(
            proc_macro2::Span::call_site(),
            "`struct_fragments` does not accept arguments",
        )
        .into_compile_error()
        .into();
    }

    match syn::parse(input).and_then(expand::expand) {
        Ok(output) => output.into(),
        Err(error) => error.into_compile_error().into(),
    }
}
