# talmaci-struct-fragments-rs

Compile-time reusable struct field composition for Rust.

Rust backend applications often repeat groups of fields across API responses,
persistence records, internal models, list/detail representations, and audit or
timestamp structures. This crate supports both reusable flattened field groups
and concise nested fields at compile time.

This is deliberately **not inheritance** and does not model TypeScript-style
`extends`. The generated types are ordinary Rust structs, with no runtime
abstraction and no runtime overhead.

## Standalone nested composition

Use the exported `compose` attribute directly when every added field is nested:

```rust
use talmaci_struct_fragments_rs::compose;

pub struct Timestamps {
    pub created_at: u64,
    pub updated_at: u64,
}

pub struct Metadata {
    pub label: String,
}

#[compose(
    timestamps: Timestamps,
    metadata: Option<Metadata>,
)]
pub struct User {
    pub id: i32,
    pub name: String,
}
```

This expands conceptually to:

```rust
pub struct User {
    pub id: i32,
    pub name: String,
    pub timestamps: Timestamps,
    pub metadata: Option<Metadata>,
}
```

Local fields stay first. Generated nested fields are public and follow in the
order listed. Their types are parsed as ordinary Rust types and do not need to
be fragments. The original struct's attributes, derives, visibility, generics,
where clauses, documentation, and local fields are preserved.

Standalone `compose` accepts only `field_name: Type` entries. Flatten entries
such as `#[compose(Timestamps)]` and `self` require module-level composition and
produce explicit diagnostics in standalone context.

## Flattened composition

Flattening requires `struct_fragments` around an inline module because the outer
macro must receive each fragment's syntax tree in the same invocation. A stable
attribute macro cannot inspect the fields of an arbitrary type name.

Inside `#[struct_fragments]`, the helper `compose` attribute has three entry
forms.

### Flatten

```rust
#[compose(Timestamps)]
```

The fragment's fields are cloned into the destination:

```rust
pub created_at: u64,
pub updated_at: u64,
```

Only flattened identifiers are looked up in the module's fragment definitions.

### Nested

```rust
#[compose(timestamps: Timestamps)]
```

This generates one ordinary public field:

```rust
pub timestamps: Timestamps,
```

The right-hand side is parsed as a Rust type, so types such as
`Option<Metadata>` work too. A nested type does not need to be a fragment.

### Local placement

Destination-local fields normally follow all module-level compose entries. The
`self` entry places them explicitly instead:

```rust
#[compose(Identity, self, Timestamps)]
```

Local fields retain their original order, and `self` may appear at most once.

## Complete module-level example

```rust
use talmaci_struct_fragments_rs::struct_fragments;

#[struct_fragments]
mod models {
    #[fragment]
    pub struct Identity {
        pub id: i32,
    }

    #[fragment]
    pub struct Timestamps {
        pub created_at: u64,
        pub updated_at: u64,
    }

    pub struct Profile {
        pub bio: String,
    }

    #[compose(
        Identity,
        self,
        Timestamps,
        profile: Profile,
    )]
    pub struct User {
        pub name: String,
    }
}
```

Conceptually, `User` expands to:

```rust
pub struct User {
    pub id: i32,
    pub name: String,
    pub created_at: u64,
    pub updated_at: u64,
    pub profile: Profile,
}
```

Expansion walks compose entries from left to right. A flattened fragment appends
its fields, a nested entry appends one public field, and `self` appends the local
fields. If `self` is absent, local fields are appended last. Each flattened field
is cloned as a complete syntax tree, preserving visibility, documentation,
attributes, name, and type.

Fragment definitions remain ordinary Rust structs. The same fragment can
intentionally be flattened with `Timestamps` or nested with
`timestamps: Timestamps`.

## IDE support

rust-analyzer can expand procedural macros and understand the resulting ordinary
Rust structs when procedural-macro and attribute-macro support are enabled. This
allows type checking, generated-field completion, field-access completion, and
invalid-field diagnostics. Current rust-analyzer releases enable this support by
default, but editor or workspace configuration can disable it.

To inspect the complete generated struct, put the caret on the macro use and run
**rust-analyzer: Expand macro recursively at caret**. The command-line
alternative is:

```text
cargo expand
```

Normal hover is rendered by rust-analyzer, not by the procedural macro. Depending
on the editor and rust-analyzer version, hovering over a composed type may show
the type name and documentation without listing every generated field. The macro
has no stable API for supplying custom hover text. Navigation and rename behavior
for macro-generated copies is also editor-dependent; see
[IDE support and limitations](docs/ide-support.md) for the detailed expectations
and manual verification checklist.

## Why flattening is module-level

A stable procedural macro cannot receive only a type name and ask rustc for
that type's fields. Separate macro invocations also have no reliable ordering
or shared semantic registry. `#[struct_fragments]` therefore receives one
inline module syntax tree, discovers all fragments in it, and expands every
composition deterministically in a single invocation. Standalone nested
composition needs no lookup and can operate directly on one struct. Neither
path uses global state, filesystem caches, or invocation-order assumptions.

## Diagnostics

Composition reports compile-time errors for unknown flattened fragments,
standalone flatten or `self` attempts, duplicate field names across every field
source, duplicate module-level `self` entries, malformed entry syntax,
unsupported struct shapes, and generic fragment definitions. Duplicate
diagnostics point to both the conflicting and original fields when possible.
Fields are never silently renamed or overwritten.

## Current limitations

- Standalone `compose` supports only named-field structs and nested entries.
- Fragment flattening supports only inline `struct_fragments` modules and
  named-field structs.
- Fragment names are unqualified identifiers in the same annotated module.
- Generic fragments are rejected. Generic destination structs are supported.
- Field omission, renaming, transformations, conditional mappings, generated
  conversions, and framework-specific behavior are out of scope.

## Development

```text
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

Use `cargo expand` in a consuming crate to inspect the straightforward generated
structs.

## License

Licensed under the MIT License.
