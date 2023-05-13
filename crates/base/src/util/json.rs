// // Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::{reflect::TypeUuid, prelude::{Plugin, AddAsset}, asset::{AssetLoader, LoadedAsset}};
use serde::{Deserialize, Serialize};
use shrinkwraprs::Shrinkwrap;

#[derive(Serialize, Deserialize, TypeUuid, Clone, Eq, PartialEq, Shrinkwrap)]
#[uuid = "3712f823-1105-40ad-bf19-0289e090aaf1"]
pub struct Json(pub serde_json::Value);

pub struct JsonAssetPlugin;

impl Plugin for JsonAssetPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_asset::<Json>().add_asset_loader(JsonAssetLoader);
    }
}

pub struct JsonAssetLoader;

impl AssetLoader for JsonAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let asset = serde_json::from_slice::<Json>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["json"]
    }
}