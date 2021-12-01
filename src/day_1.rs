pub fn count_measurement_increases(input: Vec<String>) {
    let mut increases: i32 = 0;
    let mut prev: i32 = -1;
    for line in input {
        let current: i32 = line.parse().unwrap_or_default();
        if prev != -1 && current > prev {
            increases += 1;
        }
        prev = current;
    }
    println!("Number increased {} times", increases);
}

pub fn count_measurement_increases_sliding(input: Vec<String>) {
    let mut increases: i32 = 0;
    let mut prev_sum: i32 = -1;
    for i in 1..input.len() - 1 {
        let sum = to_i32(&input[i-1]) + to_i32(&input[i]) + to_i32(&input[i+1]);
        if prev_sum != -1 && sum > prev_sum {
            increases += 1
        }
        prev_sum = sum
    }
    println!("Sliding sum increased {} times", increases);
}

fn to_i32(string: &str) -> i32 {
    string.parse().unwrap_or_default()
}