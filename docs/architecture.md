# Architecture

## Stable-Rust boundary

`#[struct_fragments]` receives an entire inline module. This is necessary
because a stable procedural macro cannot ask rustc for the fields of a type name
provided to a separate macro invocation. The module-level design discovers
fragment definitions and expands destinations in one deterministic syntax-tree
transformation. It uses no global registry, filesystem discovery, or macro
invocation ordering.

Fragment definitions remain ordinary structs in the output. Their helper
attributes are removed before emitting the transformed module.

The exported standalone `#[compose(...)]` attribute receives one `ItemStruct`.
It can append nested fields because their names and complete Rust types are in
the attribute input. It cannot flatten a type name because that type's fields
are not available to the invocation. Standalone flattening and `self` are
therefore rejected during validation rather than approximated through external
state.

## Compose grammar

The parser produces one `ComposeEntry` per comma-separated entry:

```rust
enum ComposeEntry {
    Flatten(Ident),
    Nested {
        field_name: Ident,
        ty: Type,
    },
    SelfFields,
}
```

The source forms are:

```text
Fragment          flatten a fragment's fields
field_name: Type  generate one public nested field
self              insert the destination's local fields
```

`Type` is `syn::Type`, so nested fields can use ordinary Rust type syntax and
do not need to reference fragments. `self` is a reserved placement entry and
may occur at most once.

Both public contexts share this parser. Module-level validation accepts all
three variants. Standalone validation accepts only `Nested`.

## Expansion order

After parsing and structural validation, expansion walks compose entries from
left to right:

1. `Flatten` resolves a same-module fragment and appends clones of all its
   fields in declaration order.
2. `Nested` appends one newly generated `pub field_name: Type` field.
3. `SelfFields` appends all unchanged destination-local fields in their
   declaration order.

If there is no `SelfFields` entry, local fields are appended after all compose
entries. An explicit `self` remains valid when there are no local fields.

Standalone expansion uses a deliberately different fixed order: unchanged
local fields first, followed by nested fields in attribute order. It preserves
the rest of the parsed `ItemStruct`, including attributes, derives, visibility,
generics, and where clauses.

Every appended field is registered by name before final output is assigned.
Conflicts between any flattened, nested, or local sources produce a diagnostic
at the new field and, when possible, a second diagnostic at the earlier field.
There is no overwrite or rename behavior.

## Source spans and IDEs

The destination struct itself is retained rather than regenerated. Flattened
fragment fields and local fields are complete `syn::Field` clones, so their
tokens retain their original source spans. A generated nested field reuses the
field identifier, colon, and type tokens from its compose entry; its generated
`pub` token uses the field identifier's span.

These spans improve diagnostics and give rust-analyzer useful source mappings,
but cannot guarantee a particular navigation destination. A flattened field is
both a generated destination field and a copy originating in a fragment, so an
editor may navigate to the fragment declaration, the macro invocation, or a
virtual expansion depending on rust-analyzer and client behavior. The proc-macro
API cannot supply custom hover content.

## Deliberate limits

Generic fragments, cross-module fragment lookup, omit/pick/rename operations,
field or type transformations, conditional composition, generated conversions,
derive propagation, and framework-specific integrations are outside the current
design.
