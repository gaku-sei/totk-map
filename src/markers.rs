use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_mod_raycast::RaycastMesh;

use crate::{
    camera::MainCamera,
    picking::RaycastSet,
    resources::{DisplayedMarkers, FocusedMarkers, Lod, MapType, Markers, SpawnedMarkers},
};

const LOCATION_ICON_PATH: &str = "icons/mainquest.png";
const MATERIAL_ICON_PATH: &str = "icons/star.png";

pub struct MarkersPlugin;

impl Plugin for MarkersPlugin {
    fn build(&self, app: &mut App) {
        let markers = Markers::load().expect("markers to load");
        app.insert_resource(markers)
            .init_resource::<SpawnedMarkers>()
            .init_resource::<DisplayedMarkers>()
            .init_resource::<FocusedMarkers>()
            .add_systems(Startup, draw_markers)
            .add_systems(
                Update,
                (change_markers_visibility, focus_markers, update_scale),
            );
    }
}

#[derive(Component)]
pub struct MarkerSprite {
    pub name: String,
    pub layer_name: Option<String>,
    pub min_lod: u32,
    pub max_lod: u32,
}

impl MarkerSprite {
    fn new(name: String, layer_name: Option<String>, min_lod: u32, max_lod: u32) -> Self {
        Self {
            name,
            layer_name,
            min_lod,
            max_lod,
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn spawn_markers_for_map(
    commands: &mut Commands,
    assets_server: &AssetServer,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    markers: &Markers,
    displayed_markers: &mut DisplayedMarkers,
    map_type: MapType,
    spawned_markers: &mut SpawnedMarkers,
) {
    if spawned_markers.is_spawned(map_type) {
        return;
    }

    displayed_markers.markers_mut().clear();
    for location in markers.locations(map_type) {
        displayed_markers
            .markers_mut()
            .insert(location.name.clone());
        for layer in &location.layers {
            let icon_path = layer.icon.as_ref().map_or_else(
                || LOCATION_ICON_PATH.to_string(),
                |icon| format!("icons/{}", icon.url),
            );

            for layer_marker in &layer.markers {
                commands
                    .spawn(MaterialMesh2dBundle {
                        mesh: meshes
                            .add(Mesh::from(shape::Quad::new(Vec2::splat(2.0))))
                            .into(),
                        material: materials
                            .add(ColorMaterial::from(Color::rgba(0.0, 0.0, 0.0, 0.0))),
                        transform: Transform::from_xyz(
                            layer_marker.pos.y,
                            layer_marker.pos.x,
                            100.0,
                        ),
                        ..default()
                    })
                    .with_children(|commands| {
                        commands.spawn(SpriteBundle {
                            texture: assets_server.load(&icon_path),
                            transform: Transform::from_scale(Vec3::splat(0.1)),
                            ..default()
                        });
                    })
                    .insert(MarkerSprite::new(
                        location.name.to_string(),
                        layer_marker.name.clone(),
                        layer.min_lod,
                        layer.max_lod,
                    ))
                    .insert(RaycastMesh::<RaycastSet>::default());
            }
        }
    }

    for material in markers.materials(map_type) {
        for pos in &material.pos {
            commands
                .spawn(MaterialMesh2dBundle {
                    mesh: meshes
                        .add(Mesh::from(shape::Quad::new(Vec2::splat(2.0))))
                        .into(),
                    material: materials.add(ColorMaterial::from(Color::rgba(0.0, 0.0, 0.0, 0.0))),
                    transform: Transform::from_xyz(pos.y, pos.x, 100.0),
                    ..default()
                })
                .with_children(|commands| {
                    commands.spawn(SpriteBundle {
                        texture: assets_server.load(MATERIAL_ICON_PATH),
                        transform: Transform::from_scale(Vec3::splat(0.1)),
                        ..default()
                    });
                })
                .insert(MarkerSprite::new(
                    material.name.to_string(),
                    None,
                    Lod::MIN_VALUE,
                    Lod::MAX_VALUE,
                ))
                .insert(RaycastMesh::<RaycastSet>::default());
        }
    }

    spawned_markers.mark_spawned(map_type);
}

#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)]
fn draw_markers(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    markers: Res<Markers>,
    mut displayed_markers: ResMut<DisplayedMarkers>,
    map_type: Res<MapType>,
    mut spawned_markers: ResMut<SpawnedMarkers>,
) {
    spawn_markers_for_map(
        &mut commands,
        &assets_server,
        &mut meshes,
        &mut materials,
        &markers,
        &mut displayed_markers,
        *map_type,
        &mut spawned_markers,
    );
}

#[allow(clippy::needless_pass_by_value)]
fn change_markers_visibility(
    displayed_markers: Res<DisplayedMarkers>,
    lod: Res<Lod>,
    mut marker_sprites: Query<(&mut Visibility, &MarkerSprite)>,
) {
    for (mut marker_sprite_visibility, marker_sprite) in &mut marker_sprites {
        *marker_sprite_visibility = if displayed_markers.markers().contains(&marker_sprite.name)
            && *lod >= marker_sprite.min_lod
            && *lod <= marker_sprite.max_lod
        {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

#[allow(clippy::needless_pass_by_value)]
fn focus_markers(
    mut focused_markers: ResMut<FocusedMarkers>,
    query: Query<(&RaycastMesh<RaycastSet>, &MarkerSprite)>,
) {
    *focused_markers = FocusedMarkers::default();
    for (mesh, marker_sprite) in &query {
        let mut name: String = marker_sprite.name.clone();
        if let Some(layer_marker_name) = &marker_sprite.layer_name {
            if name != layer_marker_name.as_str() {
                name.push_str(" - ");
                name.push_str(layer_marker_name);
            }
        }
        for (_, intersection) in mesh.intersections() {
            debug!(
                target: "intersection",
                "name={} distance={:?} position={:?}",
                name,
                intersection.distance(),
                intersection.position(),
            );
            focused_markers.markers_mut().push(name.clone());
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
fn update_scale(
    camera: Query<&OrthographicProjection, With<MainCamera>>,
    mut marker_sprites: Query<&mut Transform, With<MarkerSprite>>,
) {
    let Ok(projection) = camera.get_single() else {
        return;
    };
    debug!(target: "camera", "scale={}", projection.scale);
    let Some(marker_sprite) = marker_sprites.iter().last() else {
        return;
    };
    let marker_scale = (10.0 * projection.scale).max(2.0).min(200.0);
    if (marker_scale - marker_sprite.scale.x).abs() < f32::EPSILON {
        return;
    }
    for mut marker_sprite in &mut marker_sprites {
        marker_sprite.scale = Vec3::splat(marker_scale);
    }
}
