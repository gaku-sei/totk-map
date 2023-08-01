use std::ops::RangeBounds;

use bevy::prelude::*;

use crate::{
    camera::MainCamera,
    resources::{LoadedTiles, Lod, MapType},
};

pub struct MapsPlugin;

#[derive(Component)]
pub struct Tile {
    map_type: MapType,
    lod: Lod,
    x_idx: u32,
    y_idx: u32,
}

impl Tile {
    fn is_in_ranges(
        &self,
        x_range: &impl RangeBounds<u32>,
        y_range: &impl RangeBounds<u32>,
    ) -> bool {
        x_range.contains(&self.x_idx) && y_range.contains(&self.y_idx)
    }
}

impl Plugin for MapsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MapType>()
            .init_resource::<LoadedTiles>()
            .add_systems(Startup, maps)
            .add_systems(Update, load_tiles);
    }
}

#[allow(clippy::missing_panics_doc)]
pub fn load_tile(
    commands: &mut Commands,
    assets_server: &AssetServer,
    loaded_tiles: &mut LoadedTiles,
    map_type: MapType,
    lod: Lod,
    x_idx: u32,
    y_idx: u32,
) {
    let path = map_type.tile_path(lod, x_idx, y_idx);
    if loaded_tiles.tiles().contains(&path) {
        return;
    }
    loaded_tiles.tiles_mut().push(path);
    let path = loaded_tiles.tiles().last().unwrap().as_path();

    let tiles_nb = lod.tiles_nb();
    let tile_px_size = lod.tile_px_size();
    let translation = tile_px_size / 2.0;

    #[allow(clippy::cast_precision_loss)]
    commands
        .spawn(SpriteBundle {
            texture: assets_server.load(path),
            sprite: Sprite {
                custom_size: Some(Vec2::new(tile_px_size, tile_px_size)),
                ..default()
            },
            transform: Transform::from_xyz(
                -(tiles_nb as f32 / 2.0 - x_idx as f32) * tile_px_size + translation,
                (tiles_nb as f32 / 2.0 - y_idx as f32) * tile_px_size - translation,
                lod * 10.0,
            ),
            ..default()
        })
        .insert(Tile {
            map_type,
            lod,
            x_idx,
            y_idx,
        });
}

#[allow(clippy::needless_pass_by_value)]
fn maps(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    mut loaded_tiles: ResMut<LoadedTiles>,
    map_type: Res<MapType>,
) {
    let lod = Lod::MIN;
    let tiles_nb = lod.tiles_nb();

    for x_idx in 0..tiles_nb {
        for y_idx in 0..tiles_nb {
            load_tile(
                &mut commands,
                &assets_server,
                &mut loaded_tiles,
                *map_type,
                lod,
                x_idx,
                y_idx,
            );
        }
    }
}

#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)]
fn load_tiles(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    mut loaded_tiles: ResMut<LoadedTiles>,
    camera: Query<&OrthographicProjection, With<MainCamera>>,
    position: Query<&Transform, With<MainCamera>>,
    lod: Res<Lod>,
    mut tiles: Query<(&mut Visibility, &Tile)>,
    map_type: Res<MapType>,
) {
    let Ok(projection) = camera.get_single() else {
        return;
    };
    let Ok(camera) = position.get_single() else {
        return;
    };

    let xs = lod.index(camera.translation.x + projection.area.min.x)
        ..=lod.index(camera.translation.x + projection.area.max.x);
    let ys = lod.index(-camera.translation.y + projection.area.min.y)
        ..=lod.index(-camera.translation.y + projection.area.max.y);

    for (mut tile_visibility, tile) in &mut tiles {
        // TODO: We should only display the tiles that are contained in the ranges for the inferior lods as well
        *tile_visibility = if tile.map_type == *map_type
            && (tile.lod == 0 || tile.lod < *lod || tile.is_in_ranges(&xs, &ys))
        {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }

    for x_idx in xs {
        for y_idx in ys.clone() {
            load_tile(
                &mut commands,
                &assets_server,
                &mut loaded_tiles,
                *map_type,
                *lod,
                x_idx,
                y_idx,
            );
        }
    }
}
