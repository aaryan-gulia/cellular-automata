mod automata;
mod rendering;
mod menu;

use std::io;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::{prelude::*, winit::WinitSettings};
use bevy::render::camera::ScalingMode;
use bevy::ui::*;
use bevy::sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle};
use bevy::text::scale_value;
use bevy::ui;
use bevy::window::*;
use bevy::winit::WinitWindows;

use automata::rule::Rule;
use automata::state::*;
use rendering::render::*;
use rendering::diagnostics::*;
use crate::automata::rule::Rule2D;
use crate::automata::SimState;
use crate::menu::ControlScreen;

fn update_cells(
    mut automata: ResMut<AutomataState>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    rule: Res<Rule>,
    mut query: Query<(&mut Transform, &mut Handle<ColorMaterial>)>,
    sim_state: Res<SimState>,
) {
    if sim_state.is_running == false {
        return;
    }
    automata.move_next_gen_parallel(&(*rule));

    for (index, mut cell) in automata.state_vec.iter_mut().enumerate() {
        let (mut transform, mut material) = query.get_mut(cell.entity.unwrap()).unwrap();
        *material = create_cell_material(cell.state, &mut materials);
    }
}

fn calculate_cell_position(grid_width: usize, index: usize, window_width:f32) -> Vec2 {
    let x = (index) as f32 * 15.0 + 10.0;
    Vec2::new(x as f32, 0.0)
}

fn create_cell_material(current_state: bool, materials: &mut ResMut<Assets<ColorMaterial>>) -> Handle<ColorMaterial> {
    if current_state {
        materials.add(<bevy::prelude::Color as Into<ColorMaterial>>::into(Color::WHITE))
    } else {
        materials.add(<bevy::prelude::Color as Into<ColorMaterial>>::into(Color::BLACK))
    }
}


pub struct CellularAutomata;
impl Plugin for CellularAutomata{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_simulation)
            .add_systems(Update,update_cells);
    }
}

fn setup_simulation(mut commands: Commands,
                    mut meshes: ResMut<Assets<Mesh>>,
                    mut materials: ResMut<Assets<ColorMaterial>>,
                    asset_server: Res<AssetServer>,
                    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let rule = format!("{:08b}", 2);
    let rule = Rule::new(rule);
    let window_width = windows.single().width() as f32/2.0;
    println!("Width of Simulation Window is: {}", window_width);
    let cell_size = 15.0;
    let num_cells = (window_width / cell_size) as usize;
    println!("Number of Cell is: {}", num_cells);
    let mut initial_state_vec = vec![false; num_cells];
    initial_state_vec[num_cells/2 + 1] = true;
    let mut automata = AutomataState::new(initial_state_vec);
    let mut index: usize = 0;
    for mut cell in &mut automata.state_vec{
        cell.position = calculate_cell_position(num_cells, index as usize, window_width as f32);
        index += 1;
        cell.material = Some(create_cell_material(cell.state,&mut materials));
        cell.entity = Some(commands.spawn(
            MaterialMesh2dBundle{
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(15.0,15.0))),
                material: create_cell_material(cell.state,&mut materials),
                transform: Transform::from_xyz(cell.position.x,0.,0.),
                ..Default::default()
            }
        ).id());
    }

    commands.insert_resource(automata);
    commands.insert_resource(rule);
    commands.insert_resource(SimState{is_running: true});
}

fn setup_camera(mut commands: Commands){
    let mut camera_bundle = Camera2dBundle::default();
    let projection = OrthographicProjection {
        scaling_mode: ScalingMode::WindowSize(1.0),
        ..OrthographicProjection::default()
    };
// change the settings we want to change:
    camera_bundle.camera.clear_color = ClearColorConfig::from(Color::GRAY);
    commands.spawn(camera_bundle);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins((bevy_framepace::FramepacePlugin, ControlScreen))
        .add_systems(Startup, (setup_camera,setup_fps_counter))
        .add_systems(Update, (
        fps_text_update_system,
    ))
        .add_plugins(CellularAutomata2D)
        .run();
}

pub struct CellularAutomata2D;
impl Plugin for crate::CellularAutomata2D {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_simulation_2d)
            .add_systems(Update,update_cells_2d);
    }
}

fn setup_simulation_2d(mut commands: Commands,
                       mut meshes: ResMut<Assets<Mesh>>,
                       mut materials: ResMut<Assets<ColorMaterial>>,
                       asset_server: Res<AssetServer>,
                       mut windows: Query<&mut Window, With<PrimaryWindow>>,
){
let rule = Rule2D{under_population: 2, over_population: 3, birth_min: 3, birth_max: 3};
    let window_width = windows.single().width() as f32/2.0;
    let cell_size = 15.0;
    let mut grid_width = (window_width / cell_size) as usize;
    let num_cells = grid_width * grid_width;
    let mut automata = AutomataState2D::new(vec![false; num_cells]);

    automata.start_with_lwss(grid_width);

    let mut index: usize = 0;
    for mut cell in &mut automata.state_vec{
        cell.position = calculate_cell_position_2d(grid_width, index);
        index += 1;
        cell.material = Some(create_cell_material(cell.state,&mut materials));
        cell.entity = Some(commands.spawn(
            MaterialMesh2dBundle{
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(15.0,15.0))),
                material: create_cell_material(cell.state,&mut materials),
                transform: Transform::from_xyz(cell.position.x,cell.position.y,0.),
                ..Default::default()
            }
        ).id());
    }

    commands.insert_resource(automata);
    commands.insert_resource(rule);
    commands.insert_resource(SimState{is_running: false});
}

fn calculate_cell_position_2d(grid_width: usize, index: usize) -> Vec2 {
    let x = (index % grid_width) as f32 * 15.0 + 10.0;
    let y = ((index / grid_width) as f32 * 15.0 + 10.0) -315.;
    Vec2::new(x as f32, y as f32)
}

fn update_cells_2d(
    mut automata: ResMut<AutomataState2D>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    rule: Res<Rule2D>,
    mut query: Query<(&mut Handle<ColorMaterial>)>,
    sim_state: Res<SimState>,
) {
    if sim_state.is_running == false {
        return;
    }

    for index in &*automata.active_indices.lock().unwrap()
    {
        let(mut material) = query.get_mut(automata.state_vec[*index].entity.unwrap()).unwrap();
        *material = create_cell_material(automata.state_vec[*index].state, &mut materials);
    }

    let grid_width = (automata.state_vec.len() as f32).sqrt() as usize;

    automata.move_next_gen_parallel(&(*rule), grid_width);

}
