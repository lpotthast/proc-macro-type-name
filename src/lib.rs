//! # proc-macro-type-name
//!
//! Convert Rust field names (snake case) to type names (pascal case) using proc_macro2 identifiers.
//!
//! ## Example
//!
//! ```rust
//! use proc_macro_type_name::ToTypeName;
//!
//! let ident: proc_macro2::Indent = Ident::new("foo_bar", Span::call_site());
//! let type_ident = (&ident).to_type_ident(ident.span());
//!
//! assert_eq!(type_ident.to_string(), "FooBar".to_owned());
//!
//! quote! {
//!     enum #type_ident {}
//! }
//! ```

#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used)]

use proc_macro2::{Ident, Span};

/// Helper trait for generating a (pascal case) Rust type name/identifier.
/// Particularly helpful in derive macros when generating enum variants for struct fields.
pub trait ToTypeName {
    fn to_type_name(&self) -> String;

    fn to_type_ident(&self, span: Span) -> Ident {
        Ident::new(self.to_type_name().as_str(), span)
    }
}

impl ToTypeName for &str {
    fn to_type_name(&self) -> String {
        to_pascal_case(self)
    }
}

impl ToTypeName for String {
    fn to_type_name(&self) -> String {
        to_pascal_case(self.as_str())
    }
}

impl ToTypeName for &Ident {
    fn to_type_name(&self) -> String {
        to_pascal_case(self.to_string().as_str())
    }
}

fn to_pascal_case(snake_case: &str) -> String {
    assert!(!snake_case
        .as_bytes()
        .iter()
        .any(|c| c.is_ascii_whitespace()));
    let mut pascal_case = String::new();
    for part in snake_case.split(&['_', '-']) {
        pascal_case.push_str(capitalize_first_letter(part).as_str());
    }
    pascal_case
}

fn capitalize_first_letter(s: &str) -> String {
    s[0..1].to_uppercase() + &s[1..]
}

#[cfg(test)]
mod test {
    use proc_macro2::Ident;
    use proc_macro2::Span;

    use crate::capitalize_first_letter;
    use crate::to_pascal_case;
    use crate::ToTypeName;

    #[test]
    fn capitalize_first_letter_ignores_whitespace() {
        assert_eq!(capitalize_first_letter(" foo"), " foo".to_owned());
    }

    #[test]
    fn capitalize_first_letter_only_modifies_first_character() {
        assert_eq!(capitalize_first_letter("foo"), "Foo".to_owned());
        assert_eq!(capitalize_first_letter("foo-bar"), "Foo-bar".to_owned());
        assert_eq!(capitalize_first_letter("foo_bar"), "Foo_bar".to_owned());
        assert_eq!(capitalize_first_letter("foo bar"), "Foo bar".to_owned());
    }

    #[test]
    #[should_panic]
    fn to_pascal_case_should_panic_on_leading_whitespace() {
        std::panic::set_hook(Box::new(|_| {})); // Hide stacktrace form console.
        to_pascal_case(" foo");
    }

    #[test]
    #[should_panic]
    fn to_pascal_case_should_panic_on_trailing_whitespace() {
        std::panic::set_hook(Box::new(|_| {})); // Hide stacktrace form console.
        to_pascal_case("bar ");
    }

    #[test]
    fn to_pascal_case_properly_converts_casing() {
        assert_eq!(to_pascal_case("foo_bar-bazBrr"), "FooBarBazBrr".to_owned());
    }

    #[test]
    fn to_type_ident_for_ident() {
        let ident = Ident::new("foo_bar", Span::call_site());
        let type_ident = (&ident).to_type_ident(ident.span());
        assert_eq!(type_ident.to_string(), "FooBar".to_owned());
    }

    #[test]
    fn to_type_ident_for_string() {
        let ident = "foo_bar".to_owned();
        let type_ident = (&ident).to_type_ident(Span::call_site());
        assert_eq!(type_ident.to_string(), "FooBar".to_owned());
    }

    #[test]
    fn to_type_ident_for_str() {
        let ident = "foo_bar";
        let type_ident = (&ident).to_type_ident(Span::call_site());
        assert_eq!(type_ident.to_string(), "FooBar".to_owned());
    }
}
