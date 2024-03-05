use crate::automata::traits::NextGenApplicable;
use bevy::prelude::*;
use rayon::prelude::*;

const X_EXTENT:f32= 1000.;
#[derive(Component,Clone)]
pub struct Cell {
    pub state: bool,
    pub material: Option<Handle<ColorMaterial>>,
    pub position: Vec2,
    pub entity: Option<Entity>,
}
impl Cell {
    pub fn calculate_cell_position(grid_width: f32, index: f32) -> Vec2 {
        let (x, y) = (0 as f32/ 2. + index as f32 / (grid_width - 1.0) as f32 * X_EXTENT, 0);
        Vec2::new(x as f32, y as f32)
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

use std::sync::Arc;

impl AutomataState {
    pub fn move_next_gen_parallel(&mut self, rule: &dyn NextGenApplicable) {
        let temp_state_vec = Arc::new(self.state_vec.clone());
        let chunk_size = 10; // Adjust this value as needed
        self.state_vec.par_chunks_mut(chunk_size).enumerate().for_each(|(chunk_index, chunk)| {
            let temp_state_vec = Arc::clone(&temp_state_vec);
            for (index, cell) in chunk.iter_mut().enumerate() {
                let global_index = chunk_index * chunk_size + index;
                let prev = if global_index == 0 {
                    temp_state_vec.get(temp_state_vec.len()-1).expect("SIZE - 1 IS ALWAYS BE A VALID INDEX")
                } else{
                    temp_state_vec.get(global_index-1).expect("pointer - 1 will never be invalid")
                };
                let next = match temp_state_vec.get(global_index+1){
                    Some(t) => t,
                    None => {
                        temp_state_vec.get(0).expect("0 IS ALWAYS A VALID INDEX")
                    }
                };
                let curr = match temp_state_vec.get(global_index){
                    Some(t) => t,
                    None => {
                        temp_state_vec.get(0).expect("0 IS ALWAYS A VALID INDEX")
                    }
                };
                cell.state = rule.get_next_state(prev.state, curr.state, next.state);
            }
        });
        self.generation += 1;
    }
}