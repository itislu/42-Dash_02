# Dash - Beacon
## Contest Challenge

**Objective:** In this DASH contest, your objective is to design a program for strategically placing beacons on a provided map to maximize node coverage. The beacons at your disposal possess varying signal strengths, denoted by their respective radii. The goal is to optimize beacon placements, considering signal strengths and coverage, to achieve the most comprehensive network coverage possible.

### General instructions

- The program must compile using the command `make` in the root directory.
- The compiled executable should be named `beacon` and placed in the root directory during the compilation via the Makefile.
- `C` is mandatory (C/C++, Go-v1.22.5, Rust for `Open League`)
- No norm is required
- You may **only** use libraries natively included with the compiler (e.g., \<deque\> and \<math.h\>). Your program **must** compile and run entirely offline, relying solely on the compiler’s built-in libraries.
- Memory leaks are not a concern.
- Be prepared to explain your logic during presentation to ensure originality and authenticity.
- **No multithreading or multiprocessing**

### Optimization Flags
We allow only the `-O2` optimization flag for C/C++. Go already uses this optimization by default so just compile with `go build file.go`.
For rust you are not allowed to use the --release flag. Just add this into your Cargo.toml and compile with `cargo build`
```toml
[profile.dev]
opt-level = 2              # O2 optimization
debug = false              # No debug symbols
debug-assertions = false   # Removes debug_assert! and related
overflow-checks = false    # Allows wrapping/overflow without panic
```

### Input:

Your program will always be tested as follows:
```bash
> ./beacon "3 2 2" input.txt
```

- The **first argument** (`argv[1]`) represents the available beacons with their respective radii, separated by spaces and arranged in descending order (duplicates are allowed) e.g., `"3 2 2"`. **Note:** This is passed as a string, not a file.
- Beacons must be placed on empty spaces (.) and cannot be placed directly on nodes (*).
- The **second argument** (`argv[2]`) is the path to a file that contains the map. The map consists of two characters:
    - '*' represents a **node**.
    - '.' represents **empty space**.
- The input map will follow these rules:
    - All lines will have the same length.
    - The map file will always exist and have read permissions.
- The map file will be provided as a command-line argument, e.g., `./beacon "3 2 2" input.txt`.
- Your program will only be tested with two arguments (argc is always 3).


### Output:

- The program should output the coordinates of the beacons in the specified order that they were given
- Ensure beacon coordinates (`ROWx,COLx`) correspond to valid placement positions (on empty spaces `.`) while maximizing coverage of nodes (`*`).
- The coordinates must follow this format: `{ROW1},{COL1}|{ROW2},{COL2}|{ROW3},{COL3}`.
- You can place two beacons with overlapping coverage areas, but they cannot be placed on the same square.
- Each node can only be counted once for coverage, regardless of how many beacons' areas it falls under.
- The top left most point is 0,0

**Example:**

Given the following file:

```bash
> cat input.txt | cat -e
*..*..........*.......*.$
.........*.......**.....$
.......*.......*...*....$
........*.............*.$
...*.........*.....*....$
......................*.$
...*.*........*......*..$
```

When executed like this:

```bash
./beacon "3 2 2" input.txt | cat -e
```

The output might look like this (note: do not print an extra `'|'` at the end):

```
3,20|2,14|3,3$
```

### Representation of beacon radii:

- The beacon spreads evenly in all directions, maintaining a square-like shape.

A beacon with a radius of 3 would cover an area like this:

```
ooooooo$
ooooooo$
ooooooo$
ooo3ooo$
ooooooo$
ooooooo$
ooooooo$
```

A becon with a radius of 1 would cover an area like this:

```
ooo$
o1o$
ooo$
```

Multiple solutions can be submitted, but only the latest one ending with a newline will be considered.

```bash
> ./beacon "3 2 2" input.txt | cat -e
3,8|3,2|3,13|$
3,20|2,14|3,3|$
3,3|4
```

Here the second line `3,20|2,14|3,3$` will be considered as the solution

### Assessment

At the end of the coding phase, submitted programs will be compiled and tested against multiple unique maps, each with its own set of beacons and potentially different timeouts. The maximum map size for the rookie league is 700x700 and the maximum number of beacons is 72.

**For each map:**

- Your program will have a fixed amount of time to compute solution. If your program exceeds the time limit, it will be stopped automatically (no need to handle program termination).
- Points will be awarded based on the number of nodes covered, compared to other participants.

**Final Ranking:**
After all challenges are completed, teams will be ranked based on their total points. One covered beacon == one point. The team with the highest score will be declared the winner.

## Open League

For the Open League, the problem is the same as in the Rookie League, but with additional considerations:

1. **Terrain Influence**
   Beacons' effectiveness depends on the terrain they are placed on. The signal strength of a beacon is calculated as:

    Effective Signal Strength = `Beacon Radius + Terrain Height`

    Terrain heights range from 1 to 9. A beacon placed on a terrain with height `3` and radius `2` will cover an area equivalent to a range `5` beacon.

2. **Multiple Maps**
You will be given **4 maps** instead of one. These maps need to be combined into a single 2x2 square grid. The grid must always follow this structure:

```
[map1][map2]
[map3][map4]
```

After potential reshuffling, the order might change:

```
[map4] [map1]
[map2] [map3]
```

3. **No Overlaps**
The maps must not overlap, and they must fit together seamlessly in the 2x2 layout.

### Example Input:
An example of a terrain map, `input.txt`, might look like this:

```bash
> cat map_1.txt | cat -e
1123*33$
1*22344$
112234*$
2223456$
3*35677$
4345*67$
```

### Input additions

- All the rules from the Rookie League are applied here, except argc is always 6 instead of 3.
- All the maps will have the same shape.

### Output addition

- Begin with the order of your maps (starting from 1), separated by | from the beacons placement (after map creation).
- Followed by the beacon placements on the newly created map.

**Example:**
For this reshuffeling arrangement
```
[map4] [map1]
[map2] [map3]
```

```bash
> ./beacon "3 2 2" map_1.txt map_2.txt map_3.txt map_4.txt | cat -e
4123|4,8|5,20|15,5$
```

### Assesment
The maximum map size will be 300x300 for each of the 4 sub-maps. And the maximum number of beacons will be 100.

Keep in mind that the terrain where a beacon is placed can significantly influence its signal strength, so choose the placement carefully!
