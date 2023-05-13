// // Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::{prelude::*, ecs::system::EntityCommands, utils::HashMap};
use serde::{Serialize, Deserialize, de::DeserializeOwned};
use shrinkwraprs::Shrinkwrap;

use crate::util::{Json, PathBufLongExt, get_bevy_asset_dir};

pub trait TileAttribName {
    const TILE_ATTRIB_NAME: &'static str;
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy, PartialEq, Eq, Component)]
#[serde(rename_all = "lowercase")] 
pub enum MovementGround {
    Developed,
    #[default]
    Regular,
    Rough,
    Difficult,
    Treacherous,
}

impl TileAttribName for MovementGround {
    const TILE_ATTRIB_NAME: &'static str = "movement_ground";
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy, PartialEq, Eq, Component)]
#[serde(rename_all = "lowercase")] 
pub enum MovementSky {
    #[default]
    Regular,
    Turbulent,
}

impl TileAttribName for MovementSky {
    const TILE_ATTRIB_NAME: &'static str = "movement_sky";
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy, PartialEq, Eq, Component)]
#[serde(rename_all = "lowercase")] 
pub enum MovementWater {
    Shallow,
    #[default]
    Regular,
    Rough,
    Treacherous,
}

impl TileAttribName for MovementWater {
    const TILE_ATTRIB_NAME: &'static str = "movement_water";
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy, PartialEq, Eq, Component)]
pub struct CombatModifiers {
    pub base_defence:  u8,
    pub has_coverage:  bool,
    pub has_elevation: bool,
}

impl TileAttribName for CombatModifiers {
    const TILE_ATTRIB_NAME: &'static str = "combat_modifiers";
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Component, Shrinkwrap)]
pub struct TileName(String);

impl TileAttribName for TileName {
    const TILE_ATTRIB_NAME: &'static str = "name";
}


pub type AttribParser = fn(&mut EntityCommands, &serde_json::Value);

#[derive(Debug, Clone, Component, Shrinkwrap)]
pub struct TileAttribConfig(serde_json::Value);

#[derive(Debug, Clone, Resource, Shrinkwrap)]
struct TileAttribEntityMap(HashMap<Handle<Json>, Entity>);

fn load_tile_attrib_configs(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut entity_map: ResMut<TileAttribEntityMap>,
) {
    let root = get_bevy_asset_dir();
    let paths = std::fs::read_dir(root.clone() + "tiles/");
    if paths.is_err() { return; }

    for path in paths.unwrap() {
        let path = path.unwrap().path();
        if path.long_ext() == Some("tile.json") {
            let path = path.to_str().unwrap();
            let path = path.split_at(root.len()).1; // TODO BUG this isn't very neat, plus, if there's a double slash this will cause a bug

            let handle = asset_server.load(path);
            let id = commands.spawn_empty().id();
            entity_map.0.insert(handle, id);
        }
    }
}

fn modify_tile_attrib_configs(
    mut query: Query<&mut TileAttribConfig>,
    mut ev_asset: EventReader<AssetEvent<Json>>,
    assets: Res<Assets<Json>>,
    entity_map: Res<TileAttribEntityMap>,
    mut commands: Commands,
) {
    for ev in ev_asset.iter() {
        match ev {
            AssetEvent::Created { handle } => {
                if let Some(&entity) = entity_map.get(handle) {
                    let json = assets.get(handle).unwrap();
                    commands.entity(entity).insert(TileAttribConfig(json.0.clone()));
                }
            },
            AssetEvent::Modified { handle } => {
                if let Some(entity) = entity_map.get(handle) {
                    if let Ok(mut entity) = query.get_mut(*entity) {
                        info!("Updated");
                        let json = assets.get(handle).unwrap();
                        entity.0 = json.0.clone();
                    }
                }
            },
            _ => {}
        }
    }

}

pub struct AttribLoaderPlugin;

impl Plugin for AttribLoaderPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .insert_resource(TileAttribEntityMap(Default::default()))
            .add_startup_system(load_tile_attrib_configs)
            .add_system(modify_tile_attrib_configs)
            .add_system(tile_attrib_deserialize::<TileName       >)
            .add_system(tile_attrib_deserialize::<MovementGround >)
            .add_system(tile_attrib_deserialize::<MovementWater  >)
            .add_system(tile_attrib_deserialize::<MovementSky    >)
            .add_system(tile_attrib_deserialize::<CombatModifiers>);
    }
}

pub fn tile_attrib_deserialize<T: DeserializeOwned + Component + TileAttribName>(
    query: Query<(Entity, &TileAttribConfig), Changed<TileAttribConfig>>,
    mut commands: Commands,
) {
    for (entity, config) in query.iter() {
        if let Some(val) = config.get(T::TILE_ATTRIB_NAME) {
            info!("Deserialize {}", T::TILE_ATTRIB_NAME);
            commands.entity(entity).insert(T::deserialize(val).unwrap());
        }
    }
}