use ashscript_types::constants::map::{CHUNK_SIZE, HEX_LAYOUT};
use bevy::{
    asset::RenderAssetUsages,
    camera::visibility::RenderLayers,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};
use bevy_magic_light_2d::prelude::CAMERA_LAYER_FLOOR;
use hexx::{hex, shapes, Hex, HexLayout, HexOrientation, PlaneMeshBuilder};
use rand::random;

use crate::components::{LoadChunks, State, TickEvent, UnloadedChunks};

pub const HEX_SIZE: Vec2 = Vec2::splat(64.0);
const COLORS: [Color; 3] = [
    /* Color::BLUE, Color::WHITE, Color::RED, */
    Color::srgba(60. / 255., 60. / 255., 60. / 255., 1.),
    Color::srgba(65. / 255., 65. / 255., 65. / 255., 1.),
    Color::srgba(55. / 255., 55. / 255., 55. / 255., 1.),
];

pub fn generate_tiles(
    unloaded_chunks: Res<UnloadedChunks>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    state: Res<State>,
) {
    println!("generating tiles");

    let mesh = hexagonal_plane(&HEX_LAYOUT);
    let mesh_handle = meshes.add(mesh);

    let material_handles = [
        materials.add(ColorMaterial::from(COLORS[0])),
        materials.add(ColorMaterial::from(COLORS[1])),
        materials.add(ColorMaterial::from(COLORS[2])),
    ];

    for chunk_hex in unloaded_chunks.0.iter() {
        for hex in shapes::hexagon(chunk_hex.to_higher_res(CHUNK_SIZE), CHUNK_SIZE) {
            generate_chunk(
                chunk_hex,
                hex,
                &mut commands,
                /* &material_handles, */
                &mesh_handle,
                &mut materials,
            );
        }
    }
}

fn generate_chunk(
    chunk_hex: &Hex,
    hex: Hex,
    commands: &mut Commands,
    /* material_handles: &[Handle<ColorMaterial>], */
    mesh_handle: &Handle<Mesh>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let pos = HEX_LAYOUT.hex_to_world_pos(hex);
    /*     let color_index = (chunk_hex.x - chunk_hex.y).rem_euclid(3);
    let material_handle = material_handles[color_index as usize].clone(); */
    let offset = /* (50. + random::<f32>() * 3.) */55. / 255.;
    let material_handle = materials.add(ColorMaterial::from(Color::srgba(
        offset, offset, offset, 1.,
    )));

    // let handle = materials.add(ColorMaterial::from(COLORS[0]));

    commands.spawn((
        Mesh2d(mesh_handle.clone()),
        MeshMaterial2d(material_handle),
        Transform::from_xyz(pos.x, pos.y, 0.0),
        RenderLayers::from_layers(CAMERA_LAYER_FLOOR),
    ));
}

pub fn hexagonal_plane(hex_layout: &HexLayout) -> Mesh {
    let mesh_info = PlaneMeshBuilder::new(hex_layout)
        // < 1 creates borders around hexes
        .with_scale(hexx::Vec3::splat(1. /* 0.95 */))
        .facing(hexx::Vec3::Z)
        .center_aligned()
        .build();
    let positions = mesh_info
        .vertices
        .iter()
        .map(|v| [v.x, v.y, v.z])
        .collect::<Vec<[f32; 3]>>();
    let normals = mesh_info
        .normals
        .iter()
        .map(|v| [v.x, v.y, v.z])
        .collect::<Vec<[f32; 3]>>();
    let uvs = mesh_info
        .uvs
        .iter()
        .map(|v| [v.x, v.y])
        .collect::<Vec<[f32; 2]>>();

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_indices(Indices::U16(mesh_info.indices))
}
