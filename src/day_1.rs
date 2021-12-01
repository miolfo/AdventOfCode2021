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
    println!("Number increased {} times", increases)
}