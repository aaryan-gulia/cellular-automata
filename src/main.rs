mod automata;

use std::io;
use std::process::Command;
use bevy::prelude::*;
use bevy::render::mesh::shape;
use bevy::sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle};

use automata::rule::Rule;
use automata::state::AutomataState;

fn update_cells(rule: Res<Rule>, mut query: Query<&mut AutomataState>){
    if true{
        for mut state_vec in &mut query{
            state_vec.move_next_gen(&(*rule));
        }
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

fn show_automata(query: Query<&AutomataState>,mut commands: Commands,
                     mut meshes: ResMut<Assets<Mesh>>,
                     mut materials: ResMut<Assets<ColorMaterial>>,) {
    if true{
        for state_vec in &query{
            let grid_width = 51; // Change this to the width of your grid
            let cell_size = Vec3::new(500.0, 50.0, 0.0);
            const X_EXTENT:f32= 1000.;
            for (index, &cell_state) in state_vec.state_vec.iter().enumerate() {
                let material = if cell_state {
                    materials.add(<bevy::prelude::Color as Into<ColorMaterial>>::into(Color::WHITE))
                } else {
                    materials.add(<bevy::prelude::Color as Into<ColorMaterial>>::into(Color::BLACK))
                };

                let x = (index as i32 - grid_width/2) as f32 * 10.0;
                let y = 0 as f32;
                let shape = Mesh2dHandle(meshes.add(Rectangle::new(15.0,15.0)));

                commands.spawn(MaterialMesh2dBundle{
                    mesh: shape,
                    material: material,
                    transform: Transform::from_xyz(-X_EXTENT / 2. + index as f32 / (grid_width - 1) as f32 * X_EXTENT,0.,0.),
                    ..Default::default()
                });
            }
        }
    }
}

fn setup_simulation(mut commands: Commands){
    let rule = format!("{:b}", 18);
    let rule = Rule::new(rule);
    let mut initial_state_vec = vec![false; 51];
    initial_state_vec[26] = true;
    let mut automata = AutomataState::new(initial_state_vec);
    commands.spawn(automata);
    commands.insert_resource(rule);
    commands.spawn(Camera2dBundle::default());
}

#[derive(Resource)]
struct GreetTimer(Timer);
pub struct CellularAutomata;
impl Plugin for CellularAutomata{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_simulation)
            .add_systems(Update,(update_cells,despawn_all,show_automata).chain());
    }
}



fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CellularAutomata)
        .run();
}


fn cmd_line_elem_automata(){
    println!("Welcome to the elementary cellular automata!");
    println!("Please enter the rule number (0-255) you'd like to see:");
    let mut rule = String::new();
    io::stdin().read_line(&mut rule).expect("NOT ABLE TO READ RULE");
    let rule:u32 = match rule.trim().parse(){
        Ok(r) => r,
        Err(_) => panic!("INVALID RULE: {}",rule),
    };
    let rule = format!("{:b}", rule);
    let rule = Rule::new(rule);

    // Hardcoding the initial state vector
    let mut initial_state_vec = vec![false; 101];
    initial_state_vec[51] = true;
    let mut automata = AutomataState::new(initial_state_vec);

    println!("Please enter the number of generations to play:");
    let mut generations = String::new();
    io::stdin().read_line(&mut generations).expect("NOT ABLE TO READ GENERATIONS");
    let generations:u32 = match generations.trim().parse(){
        Ok(r) => r,
        Err(_) => panic!("INVALID GENERATIONS: {}",generations),
    };
    automata.print_automata();
    for _gen in 1..=generations{
        automata.move_next_gen(&rule);
        automata.print_automata();
    }
}
