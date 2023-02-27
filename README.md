# proc-macro-type-name

Convert Rust field names (snake case) to type names (pascal case) using proc_macro2 identifiers.

## Example

```rust
use proc_macro_type_name::ToTypeName;

let ident: proc_macro2::Indent = Ident::new("foo_bar", Span::call_site());
let type_ident = (&ident).to_type_ident(ident.span());

assert_eq!(type_ident.to_string(), "FooBar".to_owned());

quote! {
    enum #type_ident {}
}
```
