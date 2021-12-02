pub fn count_sub_position(input: Vec<String>) {
    let mut pos = (0, 0);
    for line in input {
        let op = line_to_op(&line);
        match op.operation_type {
            OperationType::FORWARD => pos.0 += op.value,
            OperationType::DOWN => pos.1 += op.value,
            OperationType::UP => pos.1 -= op.value
        }
    }
    println!("Multiplied position is {}", pos.0 * pos.1)
}

pub fn count_sub_position_with_aim(input: Vec<String>) {
    let mut pos = (0, 0, 0);
    for line in input {
        let op = line_to_op(&line);
        match op.operation_type {
            OperationType::FORWARD => {
                pos.0 += op.value;
                pos.1 += op.value * pos.2;
            },
            OperationType::DOWN => pos.2 += op.value,
            OperationType::UP => pos.2 -= op.value
        }
    }
    println!("Multiplied position is {}", pos.0 * pos.1)
}

fn line_to_op(string: &str) -> Operation {
    let mut split = string.split_ascii_whitespace();
    Operation {
        operation_type: string_to_op_type(split.next().expect("error iterating")),
        value: split.next().expect("error iterating").parse().unwrap_or_default()
    }
}

fn string_to_op_type(string: &str) -> OperationType {
    return match string {
        "forward" => OperationType::FORWARD,
        "down" => OperationType::DOWN,
        "up" => OperationType::UP,
        _ => OperationType::FORWARD
    }
}


struct Operation {
    operation_type: OperationType,
    value: i32
}

#[derive(PartialEq)]
enum OperationType {
    FORWARD,
    DOWN,
    UP
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_to_op() {
        let op_1 = line_to_op(TEST_DATA[0]);
        let op_2 = line_to_op(TEST_DATA[1]);
        let op_3 = line_to_op(TEST_DATA[3]);
        assert!(matches!(op_1.operation_type, OperationType::FORWARD));
        assert_eq!(op_1.value, 5);
        assert!(matches!(op_2.operation_type, OperationType::DOWN));
        assert_eq!(op_2.value, 5);
        assert!(matches!(op_3.operation_type, OperationType::UP));
        assert_eq!(op_3.value, 3);
    }


    const TEST_DATA: [&str; 6] = [
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2"
    ];
}