import copy, math, time
from collections import defaultdict

DIRECTIONS = (-1, 1)
SYMBOLS = (0, 1)

class NotConcreteError(Exception):
    pass
class TuringMachine():
    def __init__(self, size):
        self.state = 0
        self.size = size
        self.step_num = 0

        self.highest_state = 0

        self.transition_matrix = {}
        self.tape = defaultdict(int) # two-directional tape initialized to zeros
        self.position = 0

    def step(self):
        """Returns any number of new machines >= 1, and the original should be discarded"""
        try:
            self.concrete_step()
            return [self]
        except NotConcreteError:
            machines = self.pre_split()
            for machine in machines:
                machine.concrete_step()
            return machines

    def concrete_step(self):
        if self.is_halted():
            return
        #print(self.position, self.state, self.transition_matrix, self.tape)
        transition = self.transition()
        if transition not in self.transition_matrix:
            raise NotConcreteError()
        self.state, symbol, move = self.transition_matrix[transition]
        self.step_num += 1
        if self.is_halted():
            return
        self.tape[self.position] = symbol
        self.position += move

    def transition(self):
        return (self.state, self.tape.get(self.position, 0))

    def pre_split(self):
        transition = self.transition()
        if transition in self.transition_matrix:
            return [self]

        split = []
        split.append(self.split(transition, write=None, direction=None, new_state=None))
        for state in range(0, min(self.highest_state + 2, self.size)):
            poss = DIRECTIONS
            if len(self.transition_matrix) == 0:
                poss = (1,)
            else:
                poss = DIRECTIONS
            for direction in poss:
                for symbol in SYMBOLS:
                    split.append(self.split(transition, write=symbol, direction=direction, new_state=state))
        assert all(transition in machine.transition_matrix for machine in split)
        return split

    def split(self, transition, new_state, write, direction):
        #print(transition, new_state, write, direction)
        assert (new_state is None and write is None and direction is None) or (0 <= new_state <= self.highest_state + 1 and new_state <= self.size - 1 and write in SYMBOLS and direction in DIRECTIONS)
        new = copy.deepcopy(self)
        new.transition_initialized = True
        new.transition_matrix[transition] = (new_state, write, direction)
        if new_state is not None:
            new.highest_state = max(new.highest_state, new_state)
        return new

    def is_halted(self):
        return self.state is None

def lazy_beaver_depth(MACHINE_SIZE, MAX_STEPS):
    machine_stack = [TuringMachine(MACHINE_SIZE)]
    all_steps = set(range(1, MAX_STEPS+1))
    hit_steps = set()
    machines_searched = 1
    while len(machine_stack) > 0:
        machine = machine_stack.pop()
        try:
            while machine.step_num < MAX_STEPS and not machine.is_halted():
                machine.concrete_step()
            if machine.is_halted():
                hit_steps.add(machine.step_num)
        except NotConcreteError:
            new_machines = machine.step()
            machines_searched += len(new_machines)
            machine_stack.extend(new_machines)
        
    missing_steps = list(all_steps - hit_steps)
    if len(missing_steps) == 0:
        return None, machines_searched
    else:
        return min(missing_steps), machines_searched

steps_hint = { 4: 100, 5: 500 }
max_steps = 1
for machine_size in range(1,10):
    start_time = time.time()
    max_steps = max(max_steps, steps_hint.get(machine_size, 1))
    steps = None
    while steps is None:
        steps, machines_searched = lazy_beaver_depth(machine_size, max_steps)
        if steps is None and max_steps >= 10:
            print("LB({}) > {} [{} machines searched, {}s search time so far]".format(machine_size, max_steps, machines_searched, math.ceil(time.time() - start_time)), flush=True)
        max_steps = 10**(int(math.log(max_steps, 10))+1)
    print("LB({}) = {} [{} machines searched, {}s search time]".format(machine_size, steps, machines_searched, math.ceil(time.time() - start_time)), flush=True)
