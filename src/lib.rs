//! Compile-time reusable struct field composition for Rust.
//!
//! The [`struct_fragments`] attribute operates on an inline module. Within that
//! module, `#[fragment]` marks ordinary named-field structs whose fields may be
//! copied into structs marked with `#[compose(...)]`. Compose entries can also
//! generate nested fields or place a destination's local fields with `self`.

use proc_macro::TokenStream;

mod expand;
mod model;
mod parse;

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
        return syn::Error::new(proc_macro2::Span::call_site(), "`struct_fragments` does not accept arguments")
            .into_compile_error()
            .into();
    }

    match syn::parse(input).and_then(expand::expand) {
        Ok(output) => output.into(),
        Err(error) => error.into_compile_error().into(),
    }
}
