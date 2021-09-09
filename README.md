# icu4x_js_regexp

Regular expressions in JavaScript have various features to support internationalization. In some cases, these features are defined with reference to the [`Unicode Standard`]. For example, the [`Canonicalize`] algorithm used to define case-insensitive (`/i`) RegExps refers to the CaseFolding.txt file of the Unicode Character Database.

The [`ICU4X`] project provides components for internationalization written in Rust. This crate builds on ICU4X to provide access to the Unicode data necessary to implement a regular expression engine compatible with the [`ECMA262 standard`].

[`Unicode Standard`]: https://unicode.org/standard/standard.html
[`Canonicalize`]: https://tc39.es/ecma262/multipage/text-processing.html#sec-runtime-semantics-canonicalize-ch
[`ICU4X`]: https://github.com/unicode-org/icu4x
[`ECMA262 standard`]: https://tc39.es/ecma262/multipage/

