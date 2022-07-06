# Svelte-Wasm-Rust-Dfsolver
WIP: work in progress

A solver for DragonFjord's A-Puzzle-A-Day (https://www.dragonfjord.com/) written in Rust, which is compiled to WASM and called by a svelte webapp.

Uses depth first search (implemented using a loop) to find all solutions for the specified day and month using a "brute force" method. The "backend" will return a solution set (an array of all possible solutions) to the GUI which is a svelte-kit webapp to disaply the solutions to the user.

The solver contains piece information to prevent the solver from flipping or rotating pieces which have symmetry i.e. flipping / rotating piece results in the same shape being placed. This information prevents solver from placing and checking redundent pieces.

Additional checks are used to prevent the solver checking dead end solutions (i.e. when placing a piece results in unreachable holes) in an attempt to increase the speed at which a solution set is found
