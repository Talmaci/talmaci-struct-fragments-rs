# talmaci-struct-fragments-rs

Compile-time reusable struct field composition for Rust.

Rust backend applications often repeat groups of fields across API responses,
persistence records, internal models, list/detail representations, and audit or
timestamp structures. This crate lets those field groups be declared once and
flattened into related structs at compile time.

This is deliberately **not inheritance** and does not model TypeScript-style
`extends`. The generated types are ordinary Rust structs, with no runtime
abstraction and no runtime overhead.

## Composition forms

`compose` has three explicit entry forms.

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

Destination-local fields normally follow all compose entries. A standalone
`self` places them explicitly instead:

```rust
#[compose(Identity, self, Timestamps)]
```

Local fields retain their original order, and `self` may appear at most once.

## Complete example

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

## Why composition is module-level

A stable procedural macro cannot receive only a type name and ask rustc for
that type's fields. Separate macro invocations also have no reliable ordering
or shared semantic registry. `#[struct_fragments]` therefore receives one
inline module syntax tree, discovers all fragments in it, and expands every
composition deterministically in a single invocation. It uses no global state,
filesystem cache, or invocation-order assumptions.

## Diagnostics

Composition reports compile-time errors for unknown flattened fragments,
duplicate field names across every field source, duplicate `self` entries,
malformed entry syntax, tuple structs, and generic fragment definitions.
Duplicate diagnostics point to both the conflicting and original fields when
possible. Fields are never silently renamed or overwritten.

## Current limitations

- Only inline modules and named-field structs are supported.
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
