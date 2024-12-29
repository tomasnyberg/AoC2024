## Advent of code 2024
My solutions for the 2024 Advent of Code event. All solutions are written in Rust, and are of questionable quality since it's my first time using the language.
The code is reasonably performant however and solves all problems in < 300 ms (see benchmarks below).

### Benchmarks
Results from `./run.sh` in WSL using a 2016 Asus ZenBook UX303 (Ryzen 7 5700U 1.8GHz, 16GB RAM)
```
----------------------------------------
Day        Time (ms)       Cumulative (ms)
----------------------------------------
1          0.30            0.30
2          0.53            0.83
3          0.16            0.99
4          0.55            1.54
5          1.71            3.25
6          24.98           28.23
7          13.11           41.34
8          0.38            41.72
9          2.40            44.13
10         0.57            44.70
11         7.81            52.50
12         2.60            55.11
13         1.41            56.52
14         53.28           109.80
15         2.51            112.31
16         31.24           143.55
17         0.05            143.60
18         0.57            144.17
19         1.87            146.04
20         70.11           216.14
21         10.96           227.11
22         20.79           247.89
23         25.34           273.23
24         0.31            273.54
25         0.07            273.61
----------------------------------------
Total                      273.61
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

