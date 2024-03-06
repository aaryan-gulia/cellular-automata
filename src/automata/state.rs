use std::collections::HashSet;
use crate::automata::traits::{NextGenApplicable, NextGenApplicable2D};
use bevy::prelude::*;
use rayon::prelude::*;
use std::sync::Mutex;

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

#[derive(Resource)]
pub struct AutomataState2D{
    pub state_vec: Vec<Cell>,
    pub generation: u32,
    pub active_indices: Mutex<HashSet<usize>>,
}

impl AutomataState2D{
    pub fn new(state_vec: Vec<bool>) -> AutomataState2D {
        let mut new_state = AutomataState2D { state_vec: Vec::new(), generation: 0 , active_indices: Mutex::new(HashSet::new())};
        for (index, state) in state_vec.iter().enumerate() {
            new_state.state_vec.push(Cell {
                state: *state,
                material: None,
                position: Vec2::new(0.,0.),
                entity: None
            });
        }
        new_state
    }
    pub fn add_cell(&mut self, state: bool, width: usize, index: usize) {
        self.state_vec.push(Cell {
            state: state,
            material: None,
            position: Cell::calculate_cell_position(width as f32, index as f32),
            entity: None
        });
    }

    pub fn move_next_gen(&mut self, rule: &dyn NextGenApplicable2D, width: usize) {
        let temp_state_vec = Arc::new(self.state_vec.clone());
        let temp_active_indices = self.active_indices.lock().unwrap().clone();
        for (index, &cell) in temp_active_indices.iter().enumerate() {
            let mut num_alive = 0;

            for i in 0..=2 {
                for j in 0..=2{
                    if i == 1 && j == 1 {
                        continue;
                    }
                    if match temp_state_vec.get((cell as i32 + j as i32 -1 + ((i as i32 -1) * width as i32))as usize){
                        Some(t)=> t.state,
                        None => false
                    } {num_alive += 1;};
                }
            }
            if(num_alive == 0 && self.state_vec.get(cell).expect("INVALID INDEX").state == false){
                self.active_indices.lock().unwrap().retain(|&x| x != cell);
                continue;
            }
            if rule.get_next_state(num_alive, self.state_vec.get(cell).expect("INVALID INDEX").state)
            {
                if self.state_vec.get(cell).expect("INVALID INDEX").state == false {
                    self.turn_on_cell_with_index(cell, width);
                }
            }
            else {
                self.turn_off_cell_with_index(cell, width);
            }
        };
        self.generation += 1;
        println!("Number of Active Cells: {}", self.active_indices.lock().unwrap().len());
    }

    pub fn move_next_gen_parallel(&mut self, rule: &dyn NextGenApplicable2D, width: usize) {
        let temp_state_vec = Arc::new(self.state_vec.clone());
        let temp_active_indices = self.active_indices.lock().unwrap().clone();
        self.state_vec.par_iter_mut().enumerate().for_each(|(index, cell)| {
            if temp_active_indices.contains(&index) {
                let mut num_alive = 0;
                let mut neighbours = HashSet::new();
                for mut i in 0..=2 {
                    for mut j in 0..=2{
                        if i == 1 && j == 1 {
                            continue;
                        }
                        if(j == 2) && (index % width == width - 1){j = -(width as i32-2);}
                        if(j == 0) && (index % width == 0){j = width as i32;}
                        if(i == 2) && (index / width == width - 1){i = -(width as i32-2);}
                        if(i == 0) && (index / width == 0){i = width as i32;}
                        let neighbour = (index as i32 + j as i32 -1 + ((i as i32 -1) * width as i32))as usize;
                        if neighbour < 0 || neighbour >= (width * width) {
                            continue;
                        }
                        if match temp_state_vec.get(neighbour){
                            Some(t)=> t.state,
                            None => false
                        } {num_alive += 1;};
                        neighbours.insert(neighbour);
                    }
                }
                if(num_alive == 0 && cell.state == false){
                    self.active_indices.lock().unwrap().retain(|&x| x != index);
                }
                else{
                    cell.state = rule.get_next_state(num_alive, temp_state_vec[index].state);
                    let mut active_indices = self.active_indices.lock().unwrap();
                    for neighbour in neighbours {
                        active_indices.insert(neighbour);
                    }
                }
            }
        });
        self.generation += 1;
        println!("Number of Active Cells: {}", self.active_indices.lock().unwrap().len());
    }

    pub fn turn_on_cell_with_table(&mut self, row_index: usize, column_index: usize, width: usize) {
        let index = row_index * width + column_index;
        self.state_vec.get_mut(index).expect("INVALID CONVERSION FROM ROW AND COLUMN TO GLOBAL INDEX").state = true;
        //modify active indices
        self.active_indices.lock().unwrap().insert(index);
        //check all neighbours and add them to active indices if necessary
        for i in 0..=2 {
            for j in 0..=2{
                if i == 1 && j == 1 {
                    continue;
                }
                let neighbour_index = (row_index as i32 + i as i32 -1) * width as i32 + (column_index as i32 + j as i32 -1);
                if neighbour_index < 0 || neighbour_index >= (width * width) as i32 {
                    continue;
                }
                if self.active_indices.lock().unwrap().contains(&(neighbour_index as usize)) == false{
                    self.active_indices.lock().unwrap().insert(neighbour_index as usize);
                }
            }
        }
    }
    pub fn turn_on_cell_with_index(&mut self, index: usize, width: usize) {
        self.state_vec.get_mut(index).expect("INVALID INDEX").state = true;
        //modify active indices
        self.active_indices.lock().unwrap().insert(index);
        //check all neighbours and add them to active indices if necessary
        let row = index / width;
        let column = index % width;
        for i in 0..=2 {
            for j in 0..=2{
                if i == 1 && j == 1 {
                    continue;
                }
                let neighbour_index = (index as i32 + j as i32 -1 + ((i as i32 -1) * width as i32))as usize;
                if neighbour_index < 0 || neighbour_index >= (width * width)  {
                    continue;
                }
                if self.active_indices.lock().unwrap().contains(&(neighbour_index as usize)) == false{
                    self.active_indices.lock().unwrap().insert(neighbour_index as usize);
                }
            }
        }
    }

    pub fn turn_off_cell_with_index(&mut self, index: usize, width: usize) {
        self.state_vec.get_mut(index).expect("INVALID INDEX").state = false;
        //check if any neighbour is alive, if not remove myself from active indices
        let row = index / width;
        let column = index % width;
        let mut is_neighbour_alive = false;
        for i in 0..=2 {
            for j in 0..=2{
                if i == 1 && j == 1 {
                    continue;
                }
                let neighbour_index = (row as i32 + i as i32 -1) * width as i32 + (column as i32 + j as i32 -1);
                if neighbour_index < 0 || neighbour_index >= (width * width) as i32 {
                    continue;
                }
                if self.state_vec.get(neighbour_index as usize).expect("INVALID INDEX").state == true {
                    is_neighbour_alive = true;
                    break;
                }
            }
        }
        if is_neighbour_alive == false {
            self.active_indices.lock().unwrap().retain(|&x| x != index);
        }
    }

    pub fn start_with_glider(&mut self, width: usize) {
        let centre = width / 2;
        let glider_pattern = vec![
            (centre - 1) * width + centre,
            centre * width + centre + 1,
            (centre + 1) * width + centre - 1,
            (centre + 1) * width + centre,
            (centre + 1) * width + centre + 1,
        ];
        for index in glider_pattern {
            self.turn_on_cell_with_index(index, width);
        }
    }
    pub fn start_with_gosper_glider_gun(&mut self, width: usize) {
        let centre = width / 2;
        let gosper_glider_gun_pattern = vec![
            (5,1), (5,2), (6,1), (6,2), (5,11), (6,11), (7,11), (4,12), (3,13), (3,14), (8,12), (9,13), (9,14), (6,15), (4,16), (5,17), (6,17), (7,17), (6,18), (8,16), (3,21), (4,21), (5,21), (3,22), (4,22), (5,22), (2,23), (6,23), (1,25), (2,25), (6,25), (7,25), (3,35), (4,35), (3,36), (4,36)
        ];
        for (row, column) in gosper_glider_gun_pattern {
            let index = (centre + row) * width + (centre + column);
            self.turn_on_cell_with_index(index, width);
        }
    }

    pub fn start_with_lwss(&mut self, width: usize) {
        let centre = width / 2;
        let lwss_pattern = vec![
            (centre -2, centre),
            (centre -1, centre),
            (centre, centre),
            (centre, centre + 1),
            (centre, centre + 2),
            (centre, centre + 3),
            (centre -1, centre + 4),
            (centre -3, centre + 4),
            (centre -3, centre + 1),
        ];
        for (row, column) in lwss_pattern {
            let index = row * width + column;
            self.turn_on_cell_with_index(index, width);
        }
    }
}