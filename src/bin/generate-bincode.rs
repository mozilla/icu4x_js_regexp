use std::error::Error;
use std::path::PathBuf;

use icu_provider::export::DataExporter;
use icu_provider::ResourceKey;
use icu_provider_blob::export::BlobExporter;
use icu_provider_uprops::PropertiesDataProvider;

fn raw_data_dir() -> PathBuf {
    PathBuf::from(std::env!("CARGO_MANIFEST_DIR")).join("data/raw")
}

fn output_path() -> PathBuf {
    PathBuf::from(std::env!("CARGO_MANIFEST_DIR")).join("data/uprops.bincode")
}

fn get_all_uprops_keys() -> Vec<ResourceKey> {
    let mut keys = vec![];

    // Keys are supported if a corresponding .toml file exists in `/data`.
    for key in &icu_uniset::provider::key::ALL_KEYS {
        let name = key.sub_category.split("=").collect::<Vec<_>>()[0];
        let mut path = raw_data_dir().clone().join(&*name);
        path.set_extension("toml");
        if path.exists() {
            keys.push(*key);
        }
    }

    keys
}

fn main() -> Result<(), Box<dyn Error>> {
    let fs_provider = PropertiesDataProvider::new(raw_data_dir());

    let sink = Box::new(std::fs::File::create(output_path())?);
    let mut exporter = BlobExporter::new_with_sink(sink);

    let keys = get_all_uprops_keys();
    for key in keys.iter() {
        icu_provider::export::export_from_iterable(key, &fs_provider, &mut exporter)?
    }

    exporter.close()?;

    Ok(())
}
