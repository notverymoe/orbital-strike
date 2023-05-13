// // Copyright 2023 Natalie Baker // AGPLv3 //

mod container;
use bevy::{prelude::PluginGroup, app::PluginGroupBuilder};
pub use container::*;

pub mod map;
pub mod util;

use map::tile::AttribLoaderPlugin;
use util::JsonAssetPlugin;

pub struct OSBaseGamePlugin;

impl PluginGroup for OSBaseGamePlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(JsonAssetPlugin)
            .add(AttribLoaderPlugin)
    }
}