from aocd import get_data
import os
import time

if not os.path.exists('inputs'):
    os.makedirs('inputs')

for day in range(1, 26):
    if not os.path.exists(f'inputs/day{day}.in'):
        with open(f'inputs/day{day}.in', 'w') as f:
            # avoid rate limit and be a good citizen :)
            time.sleep(1)
            f.write(get_data(day=day, year=2024))
            print(f"Day {day} input saved.")
