// This file is licensed under the same terms as ICU4X.
// For details, please see the LICENSE file.

use icu_provider_blob::StaticDataProvider;
use once_cell::sync::OnceCell;

const STATIC_STR_DATA: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/uprops.bincode"));

/// Get a `DataProvider`, loading from the statically initialized bincode blob.
/// Panics if unable to load the data.
pub fn get_static_provider() -> &'static StaticDataProvider {
    static PROVIDER: OnceCell<StaticDataProvider> = OnceCell::new();
    PROVIDER.get_or_init(|| {
        StaticDataProvider::new_from_static_blob(STATIC_STR_DATA)
            .expect("Deserialization should succeed")
    })
}
