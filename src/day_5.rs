pub fn count_dangerous_points(input: Vec<String>) {
    let lines = input_to_lines(&input);
    let board = lines_to_board(&lines, 1000);
    let count = count_pts(board);
    println!("Count of dangerous points is {}", count);
}

fn count_pts(board: Vec<Vec<usize>>) -> usize {
    let mut count = 0;
    for x in 0..board.len() {
        for y in 0..board.len() {
            if board[x][y] > 1 {
                count += 1
            }
        };
    };
    count
}

fn input_to_lines(input: &Vec<String>) -> Vec<Line> {
    input.into_iter().map(|line| input_to_line(line)).collect()
}

fn input_to_line(input: &str) -> Line {
    let parts = input.split("->").collect::<Vec<&str>>();
    let start_pts = parts[0].trim().split(",").collect::<Vec<&str>>();
    let end_pts = parts[1].trim().split(",").collect::<Vec<&str>>();
    Line { 
        start: (start_pts[0].parse::<usize>().unwrap(), start_pts[1].parse::<usize>().unwrap()), 
        end: (end_pts[0].parse::<usize>().unwrap(), end_pts[1].parse::<usize>().unwrap()) 
    }
}

fn lines_to_board(lines: &Vec<Line>, size: usize) -> Vec<Vec<usize>> {
    let mut board = vec![vec![0; size]; size];
    for line in lines {
        if is_horizontal(line) {
            let range = if line.start.1 > line.end.1 {line.end.1..=line.start.1} else {line.start.1..=line.end.1};
            for y in range {
                board[line.start.0][y] += 1;
            }
        } else if is_vertical(line) {
            let range = if line.start.0 > line.end.0 {line.end.0..=line.start.0} else {line.start.0..=line.end.0};
            for x in range {
                board[x][line.start.1] += 1;
            }
        }
    };
    board
}

fn is_horizontal(line: &Line) -> bool {
    line.start.0 == line.end.0
}

fn is_vertical(line: &Line) -> bool {
    line.start.1 == line.end.1
}

#[derive(Debug)]
struct Line {
    start: (usize, usize),
    end: (usize, usize)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn test_input_to_line() {
        let sample = "0,9 -> 5,9";
        let line = input_to_line(sample);
        assert_eq!(line.start.0, 0);
        assert_eq!(line.start.1, 9);
        assert_eq!(line.end.0, 5);
        assert_eq!(line.end.1, 9);
    }

    #[test]
    fn test_lines_to_board() {
        let lines = input_to_lines(&get_test_data());
        let board = lines_to_board(&lines, 10);
        assert_eq!(board[3][4], 2);
    }

    #[test]
    fn test_count() {
        let lines = input_to_lines(&get_test_data());
        let board = lines_to_board(&lines, 10);
        let count = count_pts(board);
        assert_eq!(count, 5);
    }

    fn get_test_data() -> Vec<String> {
        util::read_resource("day_5_test.txt")
    }


}