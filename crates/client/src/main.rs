// // Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::{prelude::{App, Component, AssetPlugin, PluginGroup}, DefaultPlugins};
use os_base::map::tile::{TileAttribName, tile_attrib_deserialize};
use serde::{Serialize, Deserialize};
use shrinkwraprs::Shrinkwrap;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin{
            watch_for_changes: true,
            ..Default::default()
        }))
        .add_plugins(os_base::OSBaseGamePlugin)
        .add_system(tile_attrib_deserialize::<TileAttribResource>)
        .run();
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Component, Shrinkwrap)]
pub struct TileAttribResource(String);

impl TileAttribName for TileAttribResource {
    const TILE_ATTRIB_NAME: &'static str = "sprite";
}
