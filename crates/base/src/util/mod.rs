// // Copyright 2023 Natalie Baker // AGPLv3 //

mod json;
pub use json::*;

mod path_buf;
pub use path_buf::*;

pub fn get_bevy_asset_dir() -> String {
    std::env::var("BEVY_ASSET_ROOT").or_else(|_| std::env::var("CARGO_MANIFEST_DIR")).map_or_else(|_| "./assets/".to_owned(), |v| v + "/assets/")
}