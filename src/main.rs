mod automata;
mod rendering;

use std::io;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::window::*;
use bevy::prelude::*;
use bevy::prelude::Entity;
use bevy::render::mesh::shape;
use bevy::sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle};
use bevy::winit::WinitSettings;

use automata::rule::Rule;
use automata::state::*;
use rendering::render::*;
use rendering::diagnostics::*;

fn update_cells(
    mut automata: ResMut<AutomataState>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    rule: Res<Rule>,
    mut query: Query<(&mut Transform, &mut Handle<ColorMaterial>)>,
) {
    automata.move_next_gen(&(*rule));

    for (index, mut cell) in automata.state_vec.iter_mut().enumerate() {
        let (mut transform, mut material) = query.get_mut(cell.entity.unwrap()).unwrap();
        *material = create_cell_material(cell.state, &mut materials);
        transform.translation = cell.position;
    }
}

fn calculate_cell_position(grid_width: usize, index: usize) -> Vec3 {
    let (x, y) = (index % grid_width, index / grid_width);
    Vec3::new(x as f32 * 10.0, y as f32 * 10.0, 0.0)
}

fn create_cell_material(current_state: bool, materials: &mut ResMut<Assets<ColorMaterial>>) -> Handle<ColorMaterial> {
    if current_state {
        materials.add(<bevy::prelude::Color as Into<ColorMaterial>>::into(Color::WHITE))
    } else {
        materials.add(<bevy::prelude::Color as Into<ColorMaterial>>::into(Color::BLACK))
    }
}

fn setup_simulation(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let rule = format!("{:b}", 2);
    let rule = Rule::new(rule);
    let mut initial_state_vec = vec![false; 51];
    initial_state_vec[26] = true;
    let mut automata = AutomataState::new(initial_state_vec);

    for cell in &mut automata.state_vec{
        cell.material = Some(create_cell_material(cell.state,&mut materials));
        cell.entity = Some(commands.spawn(
            MaterialMesh2dBundle{
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(15.0,15.0))),
                material: create_cell_material(cell.state,&mut materials),
                transform: Transform::from_xyz(cell.position.x, cell.position.y,cell.position.z),
                ..Default::default()
            }
        ).id());
    }

    commands.insert_resource(automata);
    commands.insert_resource(rule);
    commands.spawn(Camera2dBundle::default());
}
pub struct CellularAutomata;
impl Plugin for CellularAutomata{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_simulation)
            .add_systems(Update,update_cells);
    }
}

#[derive(Component)]
struct FpsText;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(bevy_framepace::FramepacePlugin)
        .add_systems(Startup, setup_fps_counter)
        .add_systems(Update, (
        fps_text_update_system,
    ))
        .add_plugins(CellularAutomata)
        .run();
}

fn render_cells(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>, state: ResMut<AutomataState>) {
    for cell in &*state.state_vec{
        commands.spawn(
            MaterialMesh2dBundle{
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(15.0,15.0))),
                material: create_cell_material(cell.state,&mut materials),
                transform: Transform::from_xyz(cell.position.x, cell.position.y,cell.position.z),
                ..Default::default()
            }
        );
    }
}
fn despawn_all(
    mut commands: Commands,
    query: Query<Entity, With<Mesh2dHandle>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}