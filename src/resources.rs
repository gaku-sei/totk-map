use std::{cmp::Ordering, fmt::Display, ops::Mul, path::PathBuf};

use bevy::{prelude::Resource, utils::HashSet};

use crate::types::{Location, Material};

pub const MAP_SIZE_PX: f32 = 12_000.0;

#[derive(Debug, Resource)]
pub struct Options {
    pub debug_display: bool,
    /// Forwarded to Bevy's window plugin, change canvas selector in web/wasm mode
    pub canvas: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Resource)]
pub struct Lod(u32);

impl PartialEq<u32> for Lod {
    fn eq(&self, other: &u32) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<u32> for Lod {
    fn partial_cmp(&self, other: &u32) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl Mul<f32> for Lod {
    type Output = f32;

    #[allow(clippy::cast_precision_loss)]
    fn mul(self, rhs: f32) -> Self::Output {
        self.0 as f32 * rhs
    }
}

impl Lod {
    pub const MIN_VALUE: u32 = 0;
    pub const MAX_VALUE: u32 = 6;
    pub const MIN: Lod = Lod(Self::MIN_VALUE);

    const SCALING_MAGIC_NUMBER: f32 = 2.0;

    #[allow(
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation,
        clippy::cast_possible_wrap
    )]
    #[must_use]
    pub fn from_scale(scale: f32) -> Self {
        Self(
            Self::MAX_VALUE
                - ((scale.ln() * Self::SCALING_MAGIC_NUMBER).round() as i32)
                    .max(Self::MIN_VALUE as i32)
                    .min(Self::MAX_VALUE as i32) as u32,
        )
    }

    /// For one dimension, total is `tiles_nb.pow(2)`
    #[must_use]
    pub fn tiles_nb(self) -> u32 {
        2_u32.pow(self.0)
    }

    #[allow(clippy::cast_precision_loss)]
    #[must_use]
    pub fn tile_px_size(self) -> f32 {
        MAP_SIZE_PX / self.tiles_nb() as f32
    }

    #[allow(
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation,
        clippy::cast_precision_loss,
        clippy::cast_possible_wrap
    )]
    #[must_use]
    pub fn index(self, pos: f32) -> u32 {
        let index =
            ((pos + MAP_SIZE_PX / 2.0) / MAP_SIZE_PX * self.tiles_nb() as f32).floor() as u32;
        index.max(0).min(self.tiles_nb() - 1)
    }
}

impl Display for Lod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Resource)]
pub enum MapType {
    Sky,
    Surface,
    Depths,
}

impl MapType {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sky => "sky",
            Self::Surface => "surface",
            Self::Depths => "depths",
        }
    }

    #[must_use]
    pub fn iter() -> &'static [Self] {
        &[MapType::Sky, MapType::Surface, MapType::Depths]
    }

    fn load_locations(self) -> anyhow::Result<Vec<Location>> {
        #[cfg(not(target_arch = "wasm32"))]
        let locations = {
            let file = std::fs::File::open(format!("./assets/markers/{self}/locations.json"))?;
            serde_json::from_reader(file)?
        };
        #[cfg(target_arch = "wasm32")]
        let locations = match self {
            Self::Sky => {
                serde_json::from_str(include_str!("../assets/markers/sky/locations.json"))?
            }
            Self::Surface => {
                serde_json::from_str(include_str!("../assets/markers/surface/locations.json"))?
            }
            Self::Depths => {
                serde_json::from_str(include_str!("../assets/markers/depths/locations.json"))?
            }
        };

        Ok(locations)
    }

    fn load_materials(self) -> anyhow::Result<Vec<Material>> {
        #[cfg(not(target_arch = "wasm32"))]
        let materials = {
            let file = std::fs::File::open(format!("./assets/markers/{self}/materials.json"))?;
            serde_json::from_reader(file)?
        };
        #[cfg(target_arch = "wasm32")]
        let materials = match self {
            Self::Sky => {
                serde_json::from_str(include_str!("../assets/markers/sky/materials.json"))?
            }
            Self::Surface => {
                serde_json::from_str(include_str!("../assets/markers/surface/materials.json"))?
            }
            Self::Depths => {
                serde_json::from_str(include_str!("../assets/markers/depths/materials.json"))?
            }
        };

        Ok(materials)
    }

    #[must_use]
    pub fn tile_path(self, lod: Lod, x_idx: u32, y_idx: u32) -> PathBuf {
        format!("tiles/{self}/{lod}/{x_idx}_{y_idx}.jpg").into()
    }
}

impl Default for MapType {
    fn default() -> Self {
        Self::Surface
    }
}

impl Display for MapType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Default, Resource)]
pub struct SpawnedMarkers {
    pub sky: bool,
    pub surface: bool,
    pub depths: bool,
}

impl SpawnedMarkers {
    #[must_use]
    pub fn is_spawned(&self, map_type: MapType) -> bool {
        match map_type {
            MapType::Sky => self.sky,
            MapType::Surface => self.surface,
            MapType::Depths => self.depths,
        }
    }

    pub fn mark_spawned(&mut self, map_type: MapType) {
        match map_type {
            MapType::Sky => self.sky = true,
            MapType::Surface => self.surface = true,
            MapType::Depths => self.depths = true,
        }
    }
}

#[derive(Debug, Resource)]
pub struct Markers {
    pub sky_locations: Vec<Location>,
    pub surface_locations: Vec<Location>,
    pub depths_locations: Vec<Location>,
    pub sky_materials: Vec<Material>,
    pub surface_materials: Vec<Material>,
    pub depths_materials: Vec<Material>,
}

impl Markers {
    #[must_use]
    pub fn locations(&self, map_type: MapType) -> &[Location] {
        match map_type {
            MapType::Sky => &self.sky_locations,
            MapType::Surface => &self.surface_locations,
            MapType::Depths => &self.depths_locations,
        }
    }

    #[must_use]
    pub fn materials(&self, map_type: MapType) -> &[Material] {
        match map_type {
            MapType::Sky => &self.sky_materials,
            MapType::Surface => &self.surface_materials,
            MapType::Depths => &self.depths_materials,
        }
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn load() -> anyhow::Result<Self> {
        Ok(Self {
            sky_locations: MapType::load_locations(MapType::Sky)?,
            surface_locations: MapType::load_locations(MapType::Surface)?,
            depths_locations: MapType::load_locations(MapType::Depths)?,
            sky_materials: MapType::load_materials(MapType::Sky)?,
            surface_materials: MapType::load_materials(MapType::Surface)?,
            depths_materials: MapType::load_materials(MapType::Depths)?,
        })
    }
}

#[derive(Debug, Default, Resource)]
pub struct DisplayedMarkers(HashSet<String>);

impl DisplayedMarkers {
    #[must_use]
    pub fn markers(&self) -> &HashSet<String> {
        &self.0
    }

    pub fn markers_mut(&mut self) -> &mut HashSet<String> {
        &mut self.0
    }

    pub fn add_missing_from(&mut self, src: impl IntoIterator<Item = String>) {
        self.0.extend(src);
    }

    pub fn remove_from<'a>(&mut self, src: impl IntoIterator<Item = &'a String>) {
        for marker in src {
            self.0.remove(marker);
        }
    }

    pub fn toggle(&mut self, marker: String) {
        if self.0.contains(&marker) {
            self.0.remove(&marker);
        } else {
            self.0.insert(marker);
        }
    }
}

#[derive(Debug, Default, Resource)]
pub struct FocusedMarkers(Vec<String>);

impl FocusedMarkers {
    #[must_use]
    pub fn markers(&self) -> &[String] {
        &self.0
    }

    pub fn markers_mut(&mut self) -> &mut Vec<String> {
        &mut self.0
    }
}

#[derive(Debug, Default, Resource)]
pub struct LoadedTiles(Vec<PathBuf>);

impl LoadedTiles {
    #[must_use]
    pub fn tiles(&self) -> &[PathBuf] {
        &self.0
    }

    pub fn tiles_mut(&mut self) -> &mut Vec<PathBuf> {
        &mut self.0
    }
}
