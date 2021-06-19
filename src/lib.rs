//! Provides the [Dig] trait for recursively digging through
//! a recursive data structure using a JSON-path-like selector
//! syntax.
//!
//! Also provides implementations for common types like [`serde_json::Value`].
//!
//! [`serde_json::Value`]: https://docs.serde.rs/serde_json/value/enum.Value.html
//!
//! # Features
//!
//! Provides the following features:
//!
//! - `serde_json`: Include a `Dig` implementation for `serde_json::Value`

#[cfg(feature = "serde_json")]
use serde_json::Value;

/// Used to "dig through" recursive data structures to extract
/// named values using a selector string.  Selectors are sequential
/// names separated by an ASCII '.' character, optionally prefixed
/// with a '$' root segment.
///
#[cfg_attr(
    feature = "serde_json",
    doc = r##"
# Examples

When the `serde_json` feature is enabled, [`Value`] implements `Dig`:

```
# use digger::Dig;
# use serde_json::{json, Value};
let value = json!({
    "foo": {
        "bar": {
            "baz": "hello there"
        }
    }
});

let expected = Value::String(String::from("hello there"));

assert_eq!(value.dig("foo.bar.baz"), Some(&expected));
```

[`Value`]: https://docs.serde.rs/serde_json/value/enum.Value.html
"##
)]
pub trait Dig {
    /// Retrieves a datum identified by the given name segment,
    /// or none.
    fn value_for_name(&self, name: &str) -> Option<&Self>;

    /// Fetches the data within [self] identified by the given
    /// `selector`.
    ///
    /// Selector strings have a lightweight syntax resembling basic
    /// JSON-Path selectors - chains of name segments, separated by
    /// ASCII period characters (`.`).  As in JSON Path, selectors
    /// can be absolute (i.e. prefixed with a sigil, like `$.`) or
    /// relative.
    ///
    /// Returns an optional result, containing a reference to the named
    /// data if found, and none if not.
    fn dig(&self, selector: impl AsRef<str>) -> Option<&Self> {
        selector
            .as_ref()
            .split('.')
            .skip_while(|&s| s == "$")
            .filter(|&s| !s.is_empty())
            .fold(Some(self), |res, name| match res {
                Some(d) => d.value_for_name(name),
                None => None,
            })
    }
}

#[cfg(feature = "serde_json")]
impl Dig for Value {
    fn value_for_name(&self, name: &str) -> Option<&Self> {
        match self {
            Value::Object(o) => o.get(name),
            _ => None,
        }
    }
}

#[cfg(test)]
#[cfg(feature = "serde_json")]
mod json_tests {
    use serde_json::{json, Value};

    use super::Dig;

    #[test]
    fn not_found_at_end() {
        let value = json!({
            "foo": {
                "bar": {
                    "baz": true
                }
            }
        });

        let result = value.dig("foo.bar.quux");

        assert_eq!(None, result);
    }

    #[test]
    fn not_found_at_start() {
        let value = json!({
            "foo": {
                "bar": {
                    "baz": true
                }
            }
        });

        let result = value.dig("rust.bar.baz");

        assert_eq!(None, result);
    }

    #[test]
    fn empty_selector_is_identity() {
        let value = json!({
            "foo": {
                "bar": {
                    "baz": true
                }
            }
        });

        let result = value.dig("");

        assert_eq!(Some(&value), result);
    }

    #[test]
    fn sigil_alone_is_identity() {
        let value = json!({
            "foo": {
                "bar": {
                    "baz": true
                }
            }
        });

        let result = value.dig("$");

        assert_eq!(Some(&value), result);
    }

    #[test]
    fn sigil_prefix_is_ignored() {
        let value = json!({
            "foo": {
                "bar": {
                    "baz": true
                }
            }
        });

        let result = value.dig("$.foo.bar.baz");
        let expected = Value::Bool(true);

        assert_eq!(Some(&expected), result);
    }
}
