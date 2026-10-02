# IDE support and limitations

## Expected rust-analyzer behavior

With procedural-macro expansion enabled, rust-analyzer analyzes the output of
standalone `#[compose]` and module-level `#[struct_fragments]` as ordinary Rust
syntax. The generated destination structs therefore participate in type
inference and should provide:

- record-literal field completion for composed structs;
- field-access completion such as `user.id` and `user.name`;
- type checking and unknown-field diagnostics;
- macro expansion through **Expand macro recursively at caret**.

Current rust-analyzer configuration exposes `procMacro.enable` and
`procMacro.attributes.enable`; both default to `true`. A workspace or editor can
override them. If generated fields are missing from completion, check those
settings and confirm the proc-macro crate builds successfully.

## Navigation, rename, and hover

The macro preserves useful source spans:

- flattened fields retain the spans of their fragment fields;
- destination-local fields remain unchanged;
- nested field names and types retain spans from the compose attribute;
- the generated nested-field `pub` token uses the field-name span.

This gives rust-analyzer source locations to work with, but macro expansion adds
an unavoidable ambiguity. A flattened destination field is generated while its
tokens originated in a fragment. Depending on the rust-analyzer and editor
version, go-to-definition may lead to the fragment field, the invocation, or a
virtual macro-expansion view. Rename across a generated copy and all source-level
uses is likewise not guaranteed and should be reviewed before applying edits.

Hover text is owned by rust-analyzer. Stable procedural macros return tokens and
diagnostics; they have no API for providing editor hover markup. Hovering over a
composed type may therefore show its name or documentation without rendering its
entire expanded field list. Use macro expansion or `cargo expand` when the exact
shape matters.

## What is automated in this repository

The `examples/standalone.rs` and `examples/basic.rs` downstream-style targets
verify both public contexts, normal construction, field access, and type
checking. Integration and UI tests verify valid generated fields and diagnostics
for invalid or conflicting fields. `cargo expand --example standalone` and
`cargo expand --example basic` verify the final emitted struct syntax.

The current development environment does not have a rust-analyzer binary, so
LSP requests for completion, hover, go-to-definition, and rename cannot be
automated here. Before release, open `examples/basic.rs` in VS Code with the
rust-analyzer extension and manually check:

1. completion inside `models::User { ... }`;
2. completion after `user.`;
3. an unknown-field diagnostic after adding `user.missing`;
4. go-to-definition from `user.id` and `user.profile`;
5. a preview of rename changes before accepting a fragment-field rename;
6. **Expand macro recursively at caret** on `#[struct_fragments]`;
7. hover over `User`, noting that the complete field list is not guaranteed.

Official references:

- [rust-analyzer procedural-macro configuration](https://rust-analyzer.github.io/book/configuration)
- [rust-analyzer features and recursive macro expansion](https://rust-analyzer.github.io/book/features.html)
- [Rust Reference: procedural macros and spans](https://doc.rust-lang.org/stable/reference/procedural-macros.html)
