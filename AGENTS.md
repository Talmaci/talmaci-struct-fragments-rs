# Repository guidance

## Architecture

- This is a Rust 2024 procedural macro crate.
- `#[struct_fragments]` must operate on a complete inline module. Stable proc
  macros cannot query rustc for a named type's fields, so do not replace this
  with independent attribute invocations or introduce global registries,
  filesystem caches, or ordering assumptions.
- `#[fragment]` and `#[compose(...)]` are helper attributes consumed and removed
  by the outer macro. Fragment structs remain in generated output.
- Flattening (`Fragment`) and nesting (`field: Type`) are distinct operations.
  Only flattened fragment names are resolved against the module registry.
- Expansion clones each flattened `syn::Field` so all field metadata is
  preserved, including source spans. Nested fields are newly generated public
  fields whose identifier, colon, type, and synthetic `pub` span trace to their
  compose entry.
- Expansion walks compose entries left to right. `self` inserts destination
  fields at its position; when omitted, destination fields are appended last.
- `self` may occur at most once, including when the destination has no fields.
- Duplicate names across flattened, nested, and local fields are errors. Never
  introduce silent conflict resolution.
- Fragment structs remain ordinary structs and may be flattened or used as the
  type of a nested field.
- Generic fragments are intentionally rejected in the MVP. Generic destination
  structs work because fragment fields themselves are non-generic.

## Scope

Keep the API explicit and small. Do not add omit/pick/rename/optional or
mapped-type-style transformations, generated conversions, conditional
composition, derive propagation, cross-module lookup, or framework-specific
behavior without explicit future design work.

Do not add editor-specific generated files, caches, extensions, language-server
plugins, or build-script workarounds. rust-analyzer owns completion, navigation,
rename, macro expansion, and hover rendering for the emitted token stream.

## Verification

Run all of these after changes:

```text
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

Add `trybuild` pass/fail coverage for changes to parsing or diagnostics and a
normal integration test for observable generated behavior.
