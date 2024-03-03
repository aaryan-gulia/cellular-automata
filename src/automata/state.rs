use crate::automata::traits::NextGenApplicable;
pub struct AutomataState{
    state_vec: Vec<bool>,
    generation: u32,
}
impl AutomataState {
    pub fn new(state_vec:Vec<bool>)-> AutomataState {
        let new_state = AutomataState { state_vec:state_vec, generation: 0};
        new_state
    }
    pub fn move_next_gen(&mut self, rule: &dyn NextGenApplicable){
        let max_len = self.state_vec.len();
        let mut new_state_vec:Vec<bool> = Vec::new();
        for pointer in 0..max_len{
            let prev:&bool = if pointer == 0 {
                self.state_vec.get(self.state_vec.len()-1).expect("SIZE - 1 IS ALWAYS BE A VALID INDEX")
            } else{
                self.state_vec.get(pointer-1).expect("pointer - 1 will never be invalid")
            };
            let next:&bool = match self.state_vec.get(pointer+1){
                Some(t) => t,
                None => {
                    self.state_vec.get(0).expect("0 IS ALWAYS A VALID INDEX")
                }
            };
            let ordinal:&bool = match self.state_vec.get(pointer){
                Some(t) => t,
                None => {
                    self.state_vec.get(0).expect("0 IS ALWAYS A VALID INDEX")
                }
            };
            new_state_vec.push(rule.get_next_state(*prev,*ordinal,*next));
        }
        self.state_vec = new_state_vec;
        self.generation += 1;
    }
    pub fn print_automata(&self){
        let mut print_string = String::new();
        //print_string.push_str("generation ");
        //print_string.push_str(&(self.generation.to_string()));
        //print_string.push_str(": ");
        for ordinal in &self.state_vec{
            if *ordinal {
                print_string.push('*');
            }else{
                print_string.push(' ');
            }
        }
        println!("{}",print_string);
    }
}