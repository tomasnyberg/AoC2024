## Advent of code 2024
My solutions for the 2024 Advent of Code event. All solutions are written in Rust, and are of questionable quality since it's my first time using the language.
The code is reasonably performant however and solves all problems in ~ 200 ms (see benchmarks below).

### Benchmarks
Results from `./run.sh` in WSL using a 2016 Asus ZenBook UX303 (Ryzen 7 5700U 1.8GHz, 16GB RAM)
```
----------------------------------------
Day        Time (ms)       Cumulative (ms)
----------------------------------------
1          0.21            0.21
2          0.46            0.66
3          0.06            0.72
4          0.53            1.25
5          1.62            2.87
6          22.79           25.65
7          12.82           38.47
8          0.44            38.91
9          2.64            41.55
10         0.50            42.05
11         7.65            49.70
12         2.65            52.34
13         1.75            54.09
14         52.70           106.79
15         2.51            109.29
16         30.36           139.66
17         0.05            139.71
18         0.56            140.26
19         1.91            142.17
20         10.85           153.02
21         10.69           163.71
22         19.20           182.92
23         25.09           208.01
24         0.28            208.28
25         0.07            208.35
----------------------------------------
Total                      208.35
```

### Requirements
- Rust (I used 1.79.0 but other versions should work)
- The `aocd` python library (`pip install aocd`)
- A session token from AoC (`export AOC_SESSION=<your session token>`)

#### Inputs
AoC problems require (personal) AoC inputs, and the AoC creator has requested that they not be shared publicly (hence why they are not in this repo).
Instead, use the `get_inputs.py` script to download all inputs into the `inputs` directory prior to running the rust code.
The script assumes you have exported your session token as an environment variable.

### Running solutions
Run either
- `./run.sh` to run all days and get a benchark for the runtime
or
- `./run.sh dayx` to run a specific day (e.g. `./run.sh day1`)

