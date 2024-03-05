pub trait NextGenApplicable:Sync {
    fn get_next_state(&self, prev: bool, current: bool, next: bool) -> bool;
}

pub trait NextGenApplicable2D:Sync {
    fn get_next_state(&self, alive_neighbours: u8, curr_cell:bool) -> bool;
}