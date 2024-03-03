mod automata;

use bevy::prelude::*;
use std::io;

use automata::rule::Rule;
use automata::state::AutomataState;

fn main() {
    App::new()
        .add_systems(Update,cmd_line_elem_automata)
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
