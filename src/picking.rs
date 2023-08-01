use bevy::prelude::*;
use bevy_mod_raycast::prelude::*;

use crate::ui::egui_is_hovered;

pub struct PickingPlugin;

impl Plugin for PickingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultRaycastingPlugin::<RaycastSet>::default())
            .add_systems(
                First,
                update_raycast_with_cursor
                    .before(RaycastSystem::BuildRays::<RaycastSet>)
                    .run_if(not(egui_is_hovered)),
            );
    }
}

#[derive(Clone, Reflect)]
pub struct RaycastSet;

fn update_raycast_with_cursor(
    mut cursor: EventReader<CursorMoved>,
    mut query: Query<&mut RaycastSource<RaycastSet>>,
) {
    let Some(cursor_moved) = cursor.iter().last() else {
        return;
    };
    for mut pick_source in &mut query {
        pick_source.cast_method = RaycastMethod::Screenspace(cursor_moved.position);
    }
}
