pub trait NextGenApplicable {
    fn get_next_state(&self, prev: bool, current: bool, next: bool) -> bool;
}