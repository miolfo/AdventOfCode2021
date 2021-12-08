use std::vec;

use crate::util;

fn board_from_input(input: &Vec<String>) -> BingoBoard {
    let mut nums: Vec<usize> = Vec::new();
    let mut board: Vec<[[usize; 5]; 5]> = vec![];
    let mut curr_vec: Vec<usize> = vec![];
    //println!("{:?}", input);
    for (i, line) in input.iter().enumerate() {
        let trimmed = line.trim();
        if i == 0 {
            nums = line.split(",").into_iter().map(|s| {
                s.trim().parse::<usize>().unwrap()
            })
            .collect();
        //Read next 5 and append to nums
        } else if !trimmed.is_empty() && i > 1 {
            let split = trimmed.split_ascii_whitespace();
            let mut as_nums: Vec<usize> = split.map(|num_str| {
                num_str.trim().parse::<usize>().unwrap()
            }).collect();
            curr_vec.append(&mut as_nums);
        //Stop current boards calculation
        } else if trimmed.is_empty() && curr_vec.len() > 0 {
            println!("{:?}", curr_vec);
            let board_arr: [[usize; 5]; 5] = vec_to_5_x_5(curr_vec);
            board.push(board_arr);
            curr_vec = vec![];
        }
    }
    let board_arr: [[usize; 5]; 5] = vec_to_5_x_5(curr_vec);
    board.push(board_arr);
    BingoBoard::new(board, nums)
}

fn vec_to_5_x_5(input: Vec<usize>) -> [[usize; 5]; 5] {
    let mut arr =  [[0; 5]; 5];
    let mut index = 0;
    for x in 0..5 {
        for y in 0..5 {
            arr[x][y] = input[index];
            index += 1;
        } 
    }
    println!("{:?}", arr);
    arr
}


#[derive(Debug)]
struct BingoBoard {
    boards: Vec<[[usize; 5]; 5]>,
    numbers: Vec<usize>
}

impl BingoBoard {

    fn new(boards: Vec<[[usize; 5]; 5]>, numbers: Vec<usize>) -> BingoBoard {
        BingoBoard {
            boards,
            numbers
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_to_5x5() {
        let test_input: Vec<usize> = vec![22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20, 15, 19];
        let res = vec_to_5_x_5(test_input);
        assert_eq!(res[0][0], 22);
        assert_eq!(res[0][1], 13);
        assert_eq!(res[4][4], 19);
        assert_eq!(res[3][2], 3);
    }

    #[test]
    fn test_board_from_input() {
        let board = board_from_input(&get_test_data());
        assert_eq!(board.boards.len(),3);
    }

    fn get_test_data() -> Vec<String> {
        util::read_resource("day_4_test.txt")
    }
}