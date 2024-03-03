use crate::automata::traits::NextGenApplicable;
use bevy::prelude::*;
use bevy::render::mesh::shape;
use bevy::sprite::MaterialMesh2dBundle;

const X_EXTENT:f32= 1000.;
#[derive(Component,Clone)]
pub struct Cell {
    pub state: bool,
    pub material: Option<Handle<ColorMaterial>>,
    pub position: Vec3,
    pub entity: Option<Entity>,
}
impl Cell {
    pub fn calculate_cell_position(grid_width: f32, index: f32) -> Vec3 {
        let (x, y) = (-X_EXTENT / 2. + index as f32 / (grid_width - 1.0) as f32 * X_EXTENT, 0);
        Vec3::new(x as f32, y as f32, 0.0)
    }
}
#[derive(Resource)]
pub struct AutomataState{
    pub state_vec: Vec<Cell>,
    pub generation: u32,
}
impl AutomataState {
    pub fn new(state_vec: Vec<bool>) -> AutomataState {
        let grid_width = state_vec.len();
        let mut new_state = AutomataState { state_vec: Vec::new(), generation: 0 };
        for (index, state) in state_vec.iter().enumerate() {
            new_state.state_vec.push(Cell {
                state: *state,
                material: None,
                position: Cell::calculate_cell_position(grid_width as f32, index as f32),
                entity: None
            });
        }
        new_state
    }
    pub fn add_cell(&mut self, state: bool, grid_width: usize, index: usize) {
        self.state_vec.push(Cell {
            state: state,
            material: None,
            position: Cell::calculate_cell_position(grid_width as f32, index as f32),
            entity: None
        });
    }

    pub fn move_next_gen(&mut self, rule: &dyn NextGenApplicable) {
        let temp_state_vec = self.state_vec.clone();
        for (index, cell) in temp_state_vec.iter().enumerate() {
            let prev = if index == 0 {
                self.state_vec.get(self.state_vec.len()-1).expect("SIZE - 1 IS ALWAYS BE A VALID INDEX")
            } else{
                self.state_vec.get(index-1).expect("pointer - 1 will never be invalid")
            };
            let next = match self.state_vec.get(index+1){
                Some(t) => t,
                None => {
                    self.state_vec.get(0).expect("0 IS ALWAYS A VALID INDEX")
                }
            };
            let curr = match self.state_vec.get(index){
                Some(t) => t,
                None => {
                    self.state_vec.get(0).expect("0 IS ALWAYS A VALID INDEX")
                }
            };
            self.state_vec[index].state = rule.get_next_state(prev.state, curr.state, next.state);
        }
        self.generation += 1;
    }
}