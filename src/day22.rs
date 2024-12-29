fn evolve(mut secret: i64) -> i64 {
    secret ^= secret << 6 & 0xFFFFFF;
    secret ^= secret >> 5 & 0xFFFFFF;
    secret ^ secret << 11 & 0xFFFFFF
}

pub fn solve(input: String) {
    let nums: Vec<i64> = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut bananas = vec![0; 1 << 20];
    let mut seen = vec![2000; 1 << 20];
    let mut part_one = 0;
    (0..nums.len()).for_each(|j| {
        let mut curr = nums[j];
        let mut prev_ones = curr % 10;
        let mut curr_key = 0;
        for i in 0..2000 {
            curr = evolve(curr);
            let ones = curr % 10;
            let diff = ones - prev_ones;
            curr_key <<= 5;
            curr_key += diff;
            curr_key &= (1 << 20) - 1;
            prev_ones = ones;
            if i > 3 && seen[curr_key as usize] != j {
                bananas[curr_key as usize] += ones;
                seen[curr_key as usize] = j;
            }
        }
        part_one += curr;
    });
    println!("{}", part_one);
    println!("{}", bananas.iter().max().unwrap());
}
