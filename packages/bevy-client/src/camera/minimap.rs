use bevy::{
    camera::{RenderTarget, Viewport, visibility::RenderLayers},
    prelude::*,
    render::view::Hdr,
};
use bevy_magic_light_2d::{gi::render_layer::{ALL_LAYERS, CAMERA_LAYER_POST_PROCESSING}, prelude::*};

use crate::{components::MinimapCamera, constants};

pub fn spawn(
    mut commands: Commands,
    camera_targets: Res<CameraTargets>,
) {
    let projection = || {
        Projection::Orthographic(OrthographicProjection {
            scale: constants::camera::MIN_SCALE * 20.,
            // near: -2000.0,
            // far: 2000.0,
            near: -1000.0,
            far: 1000.0,
            ..OrthographicProjection::default_2d()
        })
    };

    let viewport = || {
        Some(Viewport {
            physical_position: UVec2::new(0, 0),
            physical_size: UVec2::new(256, 256),
            ..default()
        })
    };

    commands.spawn((
        Camera2d,
        Camera {
            order: 4,
            viewport: viewport(),
            ..Default::default()
        },
        Hdr,
        RenderTarget::Image(camera_targets.objects_target.clone().into()),
        projection(),
        /* BloomSettings::NATURAL, */
        Name::new("minimap_camera"),
        /* ObjectsCamera, */
        SpriteCamera,
        MinimapCamera,
        RenderLayers::from_layers(CAMERA_LAYER_OBJECTS),
    ));

    commands.spawn((
        Camera2d,
        Camera {
            order: 4,
            viewport: viewport(),
            ..Default::default()
        },
        Hdr,
        RenderTarget::Image(camera_targets.walls_target.clone().into()),
        projection(),
        /* BloomSettings::NATURAL, */
        Name::new("minimap_camera"),
        /* ObjectsCamera, */
        SpriteCamera,
        MinimapCamera,
        RenderLayers::from_layers(CAMERA_LAYER_WALLS),
    ));

    commands.spawn((
        Camera2d,
        Camera {
            order: 4,
            viewport: viewport(),
            ..Default::default()
        },
        Hdr,
        RenderTarget::Image(camera_targets.floor_target.clone().into()),
        projection(),
        /* BloomSettings::NATURAL, */
        Name::new("minimap_camera"),
        /* ObjectsCamera, */
        SpriteCamera,
        MinimapCamera,
        RenderLayers::from_layers(CAMERA_LAYER_FLOOR),
    ));
}

pub fn follow() {

}