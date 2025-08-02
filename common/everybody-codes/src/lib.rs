pub mod client;
pub mod parts_data;

/// # Errors
pub fn fetch_parts(data_dir: &str) -> Result<bool, parts_data::Error> {
    parts_data::PartsData::new_from_cargo(data_dir)?
        .store_if_necessary()
}
