# meta-signal-mind

Meta signal contract for PersonaMind policy and configuration.

Ordinary mind graph, work graph, and subscription traffic lives in
`signal-mind`. Runtime behavior lives in `mind`.

`schema/lib.schema` owns the wire vocabulary. `schema-rust-next` emits the
checked-in Rust contract under `src/schema/`; crate root keeps only small
compatibility aliases and accessors.
