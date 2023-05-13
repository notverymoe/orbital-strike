// // Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::*;
use shrinkwraprs::Shrinkwrap;

use crate::Array2D;

pub mod tile;

#[derive(Shrinkwrap, Resource)]
pub struct TileMap(Array2D<Entity>);

