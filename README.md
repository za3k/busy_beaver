# busy_beaver
Lazy beaver problem (and possibly other "busy beaver problems" in the future)

Lazy beaver problem - [Introduced](https://www.scottaaronson.com/blog/?p=4916#comment-1850265) by "Job", I read it in [Scott Aaronson](https://www.scottaaronson.com/blog/?p=4916)'s [busy beaver problem survey](https://www.scottaaronson.com/papers/bb.pdf). LB(x): for an integer x, what is the least number N such that there is no turning machine with x states, halting after EXACTLY N steps. 

Here we use the usual definition of a turning machine--there is a double-sided, infinite tape of cells. Each cell can be 0 or 1, and they are initially all zeros. The turing machine uses its current state and the symbol under the read head, together with a fixed lookup table, to:
- HALT or
- Write a fixed value to the current cell, then ove left or right (no staying still), then transition to a new fixed state.
