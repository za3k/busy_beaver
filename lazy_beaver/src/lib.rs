use std::cmp;
use bit_set::BitSet;

#[derive(Copy, Clone)]
enum Direction {
    Left,
    Right,
}

type State = u8;
const MAX_STATES: usize = 10;

#[derive(Copy, Clone)]
struct ATM {
    max_states: u8,
    next_available_state: u8,
    transitions: [Option<Option<(State, bool, Direction)>>; MAX_STATES*2],
    first_machine_ever: bool,
}

struct ATMInstance {
    state: State,
    head_position: usize,
    tape: BitSet,
}

fn execute_n_steps(tm: ATM, max_steps: u64) -> Result<Option<u64>, Vec<ATM>> {
    let mut instance = ATMInstance {
        state: 0,
        head_position: (max_steps+1) as usize,
        tape: BitSet::with_capacity(max_steps as usize*2+1),
    };
    //print!("Running a machine for {} steps\n", max_steps);
    for step in 1..=max_steps {
        let symbol_under_head = instance.tape.contains(instance.head_position);
        let transition_number = (instance.state as usize)*2+(symbol_under_head as usize);
        let transition = tm.transitions[transition_number];
        match transition {
            None => {
                // Non-defined transition encountered. Generate a bunch of more specific machines for each possible machine rule on this transition
                let mut refinement: Vec<ATM> = Vec::with_capacity(tm.max_states as usize * 4 + 1);
                
                // (1 machine) Halt on this transition
                let mut copy = ATM {  
                    max_states: tm.max_states,
                    next_available_state: tm.next_available_state,
                    transitions: tm.transitions,
                    first_machine_ever: false,
                };
                copy.transitions[transition_number] = Some(None);
                refinement.push(copy);
                    
                // (2*2*N machines) Don't halt on this transition
                for write_symbol in [false, true].iter() { // Don't halt on this transition
                    for move_direction in if tm.first_machine_ever { [Direction::Right].iter() } else { [Direction::Left, Direction::Right].iter()} { // 2x speedup by assuming first move is to the right
                        for new_state in 0..=tm.next_available_state { // (n-1)! speedup by assuming state X is always accessed before state X+1
                            let mut copy = ATM {
                                max_states: tm.max_states,
                                next_available_state: cmp::min(cmp::max(new_state+1, tm.next_available_state), tm.max_states-1),
                                transitions: tm.transitions,
                                first_machine_ever: false,
                            };
                            copy.transitions[transition_number] = Some(Some((new_state, *write_symbol, *move_direction)));
                            refinement.push(copy);
                        }
                    }
                }
                //print!("split into {} machines\n", refinement.len());
                return Err(refinement);
            },
            Some(None) => {
                //print!("halted...\n");
                return Ok(Some(step)); // Halted
            },
            Some(Some((new_state, write_symbol, move_direction))) => {
                //print!(" ran a step...\n");
                if write_symbol {
                    instance.tape.insert(instance.head_position);
                } else {
                    instance.tape.remove(instance.head_position);
                }
                match move_direction { 
                    Direction::Left => { instance.head_position -= 1 },
                    Direction::Right => { instance.head_position += 1 },
                };
                instance.state = new_state;
            }
        }
    }
    return Ok(None); // Didn't halt in max_steps steps
}

pub fn lazy_beaver_limited(states: u8, max_steps: u64) -> Result<(u64, u64), u64> {
    assert!(states > 0);
    let mut steps_seen: Vec<bool> = vec![false; max_steps as usize];
    let mut machines: Vec<ATM> = Vec::new();
    let mut machines_seen: u64 = 0;
    machines.push(ATM {
        transitions: [None; MAX_STATES*2],
        max_states: states,
        next_available_state: cmp::min(states-1, 1),
        first_machine_ever: true,
    });
    while let Some(tm) = machines.pop() {
        machines_seen += 1;
        let result = execute_n_steps(tm, max_steps);
        match result {
            Ok(None) => {
                // Didn't finish running in max_steps steps
            },
            Ok(Some(steps)) => {
                steps_seen[steps as usize - 1] = true;
            },
            Err(new_machines) => {
                machines.extend(new_machines)
            },
        }
    }
    match steps_seen.iter().position(|x| !x) {
        None => Result::Err(machines_seen),
        Some(steps) => Result::Ok((machines_seen, steps as u64 + 1)),
    }
}

pub fn lazy_beaver(states: u8) -> u64 {
    for power in 0.. {
        let result = lazy_beaver_limited(states, 10u64.pow(power));
        if let Result::Ok((_, steps)) = result {
            return steps;
        }
    }
    unreachable!();
}
