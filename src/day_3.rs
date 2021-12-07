pub fn calculate_power_consumption(input: Vec<String>) {
    let gamma_rate = calculate_gamma_rate(&input);
    let epsilon_rate = calculate_epsilon_rate(&gamma_rate);
    println!("Power consumption is {}", to_decimal(&gamma_rate) * to_decimal(&epsilon_rate))
}

pub fn calculate_life_support_rating(input: Vec<String>) {
    let oxygen = calculate_oxygen_generator_rating(&input);
    let co2 = calculate_co2_scrubber_rating(&input);
    println!("Life support rating is {}", to_decimal(&oxygen) * to_decimal(&co2))
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

fn calculate_most_common_at_pos(input: &Vec<String>, pos: usize) -> char {
    let zeros = count_chars_in_strs(input, '0', pos);
    let ones = count_chars_in_strs(input, '1', pos);
    if ones > zeros {'1'} else if ones == zeros {'1'} else {'0'}
}

fn calculate_least_common_at_pos(input: &Vec<String>, pos: usize) -> char {
    let zeros = count_chars_in_strs(input, '0', pos);
    let ones = count_chars_in_strs(input, '1', pos);
    if ones > zeros {'0'} else if ones == zeros {'0'} else {'1'}
}

fn count_chars_in_strs(input: &Vec<String>, compared: char, pos: usize) -> usize {
    input.into_iter().filter(|s| {
        let c = s.chars().nth(pos).unwrap();
        c == compared
    }).count()
}

fn calculate_rating(input: &Vec<String>, use_least_common: bool) -> String {
    let mut remaining = input.clone();
    for i in 0..input[0].len() {
            let character = if use_least_common {calculate_least_common_at_pos(&remaining, i)} else {calculate_most_common_at_pos(&remaining, i)};
            remaining = remaining.into_iter().filter(|c| {
                let character_2 = c.chars().nth(i).unwrap();
                character_2 == character
            }).collect();
            if remaining.len() == 1 {
                return remaining[0].to_string();
            }
    }

    "".to_string()
}

fn calculate_oxygen_generator_rating(input: &Vec<String>) -> String {
    calculate_rating(input, false)
}

fn calculate_co2_scrubber_rating(input: &Vec<String>) -> String {
    calculate_rating(input, true)
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

    #[test]
    fn test_oxygen_gen_rating() {
        let oxygen_gen_rating = calculate_oxygen_generator_rating(&get_test_data());
        assert_eq!(oxygen_gen_rating, "10111");
    }

    #[test]
    fn test_co2_scrubber_rating() {
        let co2_rating = calculate_co2_scrubber_rating(&get_test_data());
        assert_eq!(co2_rating, "01010");
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