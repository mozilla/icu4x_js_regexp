use icu_provider::prelude::DataPayload;
use icu_uniset::provider::{UnicodePropertyV1, UnicodePropertyV1Marker};
use icu_uniset::UnicodeSet;

/// A set of unicode characters.
pub struct ICU4XUniset(pub(crate) DataPayload<'static, UnicodePropertyV1Marker>);

impl From<UnicodeSet<'static>> for ICU4XUniset {
    fn from(uniset: UnicodeSet<'static>) -> Self {
        use std::borrow::Cow;
        ICU4XUniset(DataPayload::from_owned(
            UnicodePropertyV1::from_owned_uniset(uniset, Cow::Borrowed("")),
        ))
    }
}

impl ICU4XUniset {
    pub fn get(&self) -> &UnicodeSet<'_> {
        &self.0.get().inv_list
    }
}
