use bevy::asset::Assets;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

pub fn render_cell(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>,color_material: Handle<ColorMaterial>,position: Vec3){
    commands.spawn(MaterialMesh2dBundle{
        mesh: Mesh2dHandle(meshes.add(Rectangle::new(15.0,15.0))),
        material: color_material,
        transform: Transform::from_xyz(position.x, position.y, position.z),
        ..Default::default()
    });
}
pub fn despawn_all(
    mut commands: Commands,
    query: Query<Entity, With<Mesh2dHandle>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}