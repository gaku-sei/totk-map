use bevy::prelude::{Vec2, Vec3};
use serde::Deserialize;

use crate::resources::Lod;

#[derive(Debug, Deserialize)]
pub struct LocationLayerIcon {
    pub url: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Deserialize)]
pub struct LocationLayerMarker {
    #[serde(rename = "coords")]
    pub pos: Vec2,
    pub elv: f32,
    pub id: String,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LocationLayer {
    pub icon: Option<LocationLayerIcon>,
    pub markers: Vec<LocationLayerMarker>,
    #[serde(rename = "minZoom", default)]
    pub min_lod: u32,
    #[serde(rename = "maxZoom", default = "max_lod")]
    pub max_lod: u32,
}

#[derive(Debug, Deserialize)]
pub struct Location {
    pub name: String,
    pub source: Option<String>,
    pub layers: Vec<LocationLayer>,
}

#[derive(Debug, Deserialize)]
pub struct Material {
    pub name: String,
    #[serde(rename = "markerCoords")]
    pub pos: Vec<Vec3>,
}

fn max_lod() -> u32 {
    Lod::MAX_VALUE
}
