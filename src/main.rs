use std::io::{BufReader, BufRead};
use std::fs::File;


struct BingoBoard {
    numbers: Vec<Vec<i8>>
}

impl BingoBoard {

    fn round(self: &mut Self, number: u8) -> bool {
        for (row_index, row) in self.numbers.iter_mut().enumerate() {
            for (col_index, element) in row.iter_mut().enumerate() {
                if *element == number as i8 {
                    *element *= -1;
                    return self.check_bingo(row_index, col_index);
                }
            }
        }
        return false;
    }

    fn check_bingo(self: &Self, row: usize, col: usize) -> bool {

        if self.numbers[row].iter().all(|e| e < &0) {
            return true;
        } else if self.numbers.iter().all(|r| r[col] < 0) {
            return true;
        } else if row == col {
            if (0..self.numbers.len()).all(|i| self.numbers[i][i] < 0) {
                return true;
            }
            if (0..self.numbers.len()).all(|i| self.numbers[i][self.numbers.len() - i - 1] < 0) {
                return true;
            }
        }
        return false;
    }

    fn sum_empty(self: &Self) -> usize {
        self.numbers.iter().fold(
            0, |total, row|
            total + row.iter().filter_map(|&e| if e > 0 {Some(e as usize)} else {None}).sum::<usize>()
        )
    }

}

fn main() {

    let mut lines = BufReader::new(File::open("input.txt").unwrap()).lines();

    let numbers: Vec<u8> = lines.next().unwrap().unwrap()
    .split(',').map(|l| return l.parse::<u8>().unwrap()).collect();

    lines.next();

    let (mut boards, _) = lines.map(|l| l.unwrap()).fold(
        (vec![], vec![]), |(mut boards, mut cache), line|
        {
            if cache.len() == 5 {
                boards.push(BingoBoard{numbers: cache});
                return (boards, vec![]);
            } else {
                cache.push(line.split_whitespace().map(|s| s.parse::<i8>().unwrap()).collect::<Vec<i8>>());
                return (boards, cache);
            }
        }
    );

    let mut sum_empty = 0;
    let mut last_number = 0;

    for number in numbers {
        println!("Number: {}", number);

        let mut board_index = 0;

        while board_index < boards.len() {
            if boards[board_index].round(number) {
                
                if boards.len() == 1 {
                    sum_empty = boards[board_index].sum_empty();
                    last_number = number;
                }

                boards.remove(board_index);
            } else {
                board_index += 1;
            }
        }

        if boards.is_empty() {break;}
    }

    println!("Bingo!!!");
    println!("Sum empty: {}", sum_empty);
    println!("Answer: {}", sum_empty * last_number as usize);

}
