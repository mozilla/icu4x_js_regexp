// This file is licensed under the same terms as ICU4X.
// For details, please see the LICENSE file.

mod blob_provider;
mod ffi;
mod property;
mod uniset;

pub use property::get_unicode_set;
pub use uniset::ICU4XUniset;
