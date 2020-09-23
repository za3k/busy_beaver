use std::cmp;
use std::collections::HashSet;
use bit_set::BitSet;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
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

// Return true only if we can prove it never halts
fn cannot_halt(tm: ATM) -> bool {
    let mut ALL_DIRECTIONS: HashSet<Direction> = HashSet::new();
    ALL_DIRECTIONS.insert(Direction::Left);
    ALL_DIRECTIONS.insert(Direction::Right);

    let mut reachable_states: HashSet<Option<u8>> = HashSet::new();
    let mut reachable_directions: HashSet<Direction> = HashSet::new();
    let mut writable_tape_symbols: HashSet<bool> = HashSet::new();
    reachable_states.insert(Some(0));
    loop {
        let mut readable_symbols = HashSet::new();
        readable_symbols.insert(false); // Tape always initialized with zeros
        if reachable_directions.len() > 1 { readable_symbols.extend(writable_tape_symbols.iter()); }

        let mut new_reachable_states = HashSet::new();

        for x in reachable_states.iter() {
            if let Some(state) = x {
                for tape_val in readable_symbols.iter() {
                    let transition = tm.transitions[*state as usize*2+(*tape_val as usize)];
                    match transition {
                        None => return false,
                        Some(None) => {
                            new_reachable_states.insert(None);
                        },
                        Some(Some((state, symbol, direction))) => {
                            new_reachable_states.insert(Some(state));
                            reachable_directions.insert(direction);
                            writable_tape_symbols.insert(symbol);
                        }
                    }
                }
            }
        }
        if reachable_states.is_superset(&new_reachable_states) {
            break
        } else {
            reachable_states.extend(new_reachable_states);
        }
    }
    if !reachable_states.contains(&None) {
        return true
    }
    false
    
}

enum ExecutionResult {
    Halted(u64),
    StillRunning,
    NeverHalts,
    Split(Vec<ATM>),
}

fn execute_n_steps(tm: ATM, max_steps: u64) -> ExecutionResult {
    let mut instance = ATMInstance {
        state: 0,
        head_position: (max_steps+1) as usize,
        tape: BitSet::with_capacity(max_steps as usize*2+1),
    };
    //print!("Running a machine for {} steps\n", max_steps);
    if cannot_halt(tm) {
        return ExecutionResult::NeverHalts;
    }
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
                return ExecutionResult::Split(refinement);
            },
            Some(None) => {
                //print!("halted...\n");
                return ExecutionResult::Halted(step);
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
    return ExecutionResult::StillRunning; // Didn't halt in max_steps steps
}

pub type Info = (u64, u64, u64);
pub fn lazy_beaver_limited(states: u8, max_steps: u64) -> (Info, Option<u64>) {
    assert!(states > 0);
    let mut steps_seen: Vec<bool> = vec![false; max_steps as usize];
    let mut machines: Vec<ATM> = Vec::new();
    let mut machines_seen: u64 = 0;
    let mut machines_halted: u64 = 0;
    let mut machines_neverhalt: u64 = 0;
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
            ExecutionResult::StillRunning => {
                // Didn't finish running in max_steps steps
            },
            ExecutionResult::Halted(steps) => {
                steps_seen[steps as usize - 1] = true;
                machines_halted += 1; 
            },
            ExecutionResult::Split(new_machines) => {
                machines.extend(new_machines)
            },
            ExecutionResult::NeverHalts => {
                machines_neverhalt += 1; 
            },
        }
    }
    
    let info = (machines_seen, machines_halted, machines_neverhalt);
    (info, steps_seen.iter().position(|x| !x).map(|x| x as u64))
}

pub fn lazy_beaver(states: u8) -> u64 {
    for power in 0.. {
        if let (_, Some(steps)) = lazy_beaver_limited(states, 10u64.pow(power)) {
            return steps;
        }
    }
    unreachable!();
}
