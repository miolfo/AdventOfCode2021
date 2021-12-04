pub fn calculate_power_consumption(input: Vec<String>) {
    let gamma_rate = calculate_gamma_rate(&input);
    let epsilon_rate = calculate_epsilon_rate(&gamma_rate);
    println!("Power consumption is {}", to_decimal(&gamma_rate) * to_decimal(&epsilon_rate))
}

fn calculate_gamma_rate(input: &Vec<String>) -> String {
    let mut values: Vec<char> = Vec::new();
    
    for i in 0..input[0].len() {
        let mut counts = (0, 0);

        for line in input {
            let character: char = line.chars().nth(i).unwrap();
            if character == '0' {
                counts.0 += 1
            } else {
                counts.1 += 1
            }
        }

        values.push(if counts.0 > counts.1 {'0'} else {'1'});
    }

    values.into_iter().collect()
}

fn calculate_epsilon_rate(gamma_rate: &str) -> String {
    gamma_rate.chars().map(|c| {if c == '0' {'1'} else {'0'}}).collect()
}

fn to_decimal(binary: &str) -> u32 {
    u32::from_str_radix(binary, 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gamma_rate() {
        let gamma_rate = calculate_gamma_rate(&get_test_data());
        assert_eq!(gamma_rate, "10110");
    }

    #[test]
    fn test_epsilon_rate() {
        let gamma_rate = "10110";
        let epsilon_rate = calculate_epsilon_rate(gamma_rate);
        assert_eq!(epsilon_rate, "01001");
    }

    #[test]
    fn test_to_decimal() {
        let decimal = to_decimal("10110");
        assert_eq!(22, decimal);
    }

    fn get_test_data() -> Vec<String> {
        vec![
            "00100".to_string(),
            "11110".to_string(),
            "10110".to_string(),
            "10111".to_string(),
            "10101".to_string(),
            "01111".to_string(),
            "00111".to_string(),
            "11100".to_string(),
            "10000".to_string(),
            "11001".to_string(),
            "00010".to_string(),
            "01010".to_string()
            
        ]
    }
}