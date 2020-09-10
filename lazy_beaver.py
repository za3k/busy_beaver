import copy
from collections import defaultdict

DIRECTIONS = (-1, 1)
SYMBOLS = (0, 1)
MAX_STEPS=1000

class TuringMachine():
    def __init__(self, size):
        self.state = 0
        self.size = size

        self.highest_state = 0
        self.transition_initialized = False # premature optimization, can divide number of machines by 2 if we assume the first transition is to the right=1

        self.write = {}
        self.direction = {}
        self.transition_matrix = {}
        self.tape = defaultdict(int) # two-directional tape initialized to zeros
        self.position = 0
    def step(self):
        """Returns any number of new machines >= 1, and the original should be discarded"""
        machines = self.pre_split()
        for machine in machines:
            machine.concrete_step()
        return machines

    def concrete_step(self):
        if self.is_halted():
            return
        transition = self.transition()
        #print (self, transition, self.position, self.tape, self.transition_matrix)
        self.state = self.transition_matrix[transition]
        if self.is_halted():
            return
        self.tape[self.position] = self.write[transition]
        self.position += self.direction[transition]

    def transition(self):
        return (self.state, self.tape.get(self.position, 0))

    def pre_split(self):
        transition = self.transition()
        if transition in self.transition_matrix:
            return [self]

        split = []
        split.append(self.split(transition, write=None, direction=None, new_state=None))
        for state in range(0, min(self.highest_state + 2, self.size)):
            if self.transition_initialized:
                poss = DIRECTIONS
            else:
                poss = (1,)
            for direction in poss:
                for symbol in SYMBOLS:
                    split.append(self.split(transition, write=symbol, direction=direction, new_state=state))
        assert all(transition in machine.transition_matrix for machine in split)
        return split

    def split(self, transition, new_state, write, direction):
        #print(transition, new_state, write, direction)
        assert (new_state is None and write is None and direction is None) or (0 <= new_state <= self.highest_state + 1 and new_state <= self.size - 1 and write in SYMBOLS and direction in DIRECTIONS)
        new = copy.deepcopy(self)
        new.write[transition] = write
        new.direction[transition] = direction
        new.transition_matrix[transition] = new_state
        new.transition_initialized = True
        return new

    def is_halted(self):
        return self.state is None

def step_all(machines):
    return [x for machine in machines for x in machine.step()]

machines = [TuringMachine(1)]
for step_number in range(1, MAX_STEPS+1):
    stepped_machines = step_all(machines)
    running_machines = [machine for machine in stepped_machines if not machine.is_halted()]
    num_halted = len(stepped_machines) - len(running_machines)
    if num_halted != 0:
        print("Halting on step {}: {} machines".format(step_number, num_halted))
    machines = running_machines
print("Didn't halt in {} steps: {} machines".format(MAX_STEPS, len(machines)))
