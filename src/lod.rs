use bevy::prelude::*;

use crate::{camera::MainCamera, resources::Lod};

pub struct LodPlugin {
    lod: Lod,
}

impl Default for LodPlugin {
    fn default() -> Self {
        Self { lod: Lod::MIN }
    }
}

impl Plugin for LodPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.lod)
            .add_systems(Update, update_lod);
    }
}

#[allow(clippy::needless_pass_by_value)]
fn update_lod(
    mut current_lod: ResMut<Lod>,
    camera: Query<&OrthographicProjection, With<MainCamera>>,
) {
    let Ok(projection) = camera.get_single() else {
        return;
    };
    let new_lod = Lod::from_scale(projection.scale);
    if new_lod != *current_lod {
        *current_lod = new_lod;
    }
}
