#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_egui::EguiPlugin;
use bevy_svg::prelude::*;

use crate::{
    camera::CameraPlugin, lod::LodPlugin, maps::MapsPlugin, markers::MarkersPlugin,
    picking::PickingPlugin, resources::Options, ui::UiPlugin,
};

pub mod camera;
pub mod lod;
pub mod maps;
pub mod markers;
pub mod picking;
pub mod resources;
pub mod types;
pub mod ui;

pub fn run(options: Options) {
    let canvas = options.canvas.clone();
    App::new()
        .insert_resource(options)
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Tears of the Kingdom - Map".to_string(),
                    fit_canvas_to_parent: true,
                    canvas,
                    ..default()
                }),
                ..default()
            }),
            FrameTimeDiagnosticsPlugin,
            EguiPlugin,
            UiPlugin,
            PickingPlugin,
            CameraPlugin,
            SvgPlugin,
            LodPlugin::default(),
            MapsPlugin,
            MarkersPlugin,
        ))
        .run();
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(js_name = "run")]
pub fn run_wasm(canvas: String, debug_display: bool) {
    run(Options {
        canvas: Some(canvas),
        debug_display,
    });
}
