use icu_provider_blob::StaticDataProvider;

const STATIC_STR_DATA: &[u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/uprops.bincode"));

/// Get a `DataProvider`, loading from the statically initialized bincode blob.
/// Panics if unable to load the data.
pub fn get_static_provider() -> StaticDataProvider {
    // TODO: use once_cell?
    StaticDataProvider::new_from_static_blob(STATIC_STR_DATA)
        .expect("Deserialization should succeed")
}
