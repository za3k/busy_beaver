# Busy Beaver Problem and related Problems.
Here we use the usual definition of a turning machine--there is a double-sided, infinite tape of cells. Each cell can be 0 or 1, and they are initially all zeros. The turing machine uses its current state and the symbol under the read head, together with a fixed lookup table, to:
- HALT or
- Write a (fixed) value to the current cell, then move left or right (no staying still), then transition to a (fixed) new state.

Busy beaver problem. BB(x) = N. For each integer x, what is the highest number N such that there is a Turing machine with x states, halting after EXACTLY N steps? This function is uncomputable.

Lazy beaver problem - [Introduced](https://www.scottaaronson.com/blog/?p=4916#comment-1850265) by "Job", I read it in [Scott Aaronson](https://www.scottaaronson.com/blog/?p=4916)'s [busy beaver problem survey](https://www.scottaaronson.com/papers/bb.pdf). This function is computable, and a python script is included which can calculate small values.
LB(x) = N. For each integer x, what is the least number N such that there is NO Turing machine with x states halting after EXACTLY N steps?

Beeping beaver problem or "quasihalting" problem. Introduced by Harvey Friedman and Scott Aaronson in the same survey, problem reformulation by Nick Drozd.
BBB(x) = N. For each integer x, what is the greatest number N such that there is a Turing machine which is in state s_0 on step N, but never returns to state s_0 afterwards? Note that the Turing machine does not actually need the "halt" operation in this formulation.

# This repository

lazy_beaver.py is provided which calculates small values of LB(x) (good up until about x=5). Just run the program directly from Python, on a machine which will stay powered on and otherwise idle for at least a week.

# Results

TODO (will update after running longer)
