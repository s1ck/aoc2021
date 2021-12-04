use std::collections::HashMap;

const SIZE: usize = 5;

#[derive(Debug, Clone, Copy)]
enum ItemState {
    Marked,
    Unmarked,
}

#[derive(Debug, Clone, Copy)]
struct Item {
    n: u32,
    state: ItemState,
}

impl Default for Item {
    fn default() -> Self {
        Self {
            n: 0,
            state: ItemState::Unmarked,
        }
    }
}

#[derive(Debug, Clone)]
struct Board {
    size: usize,
    rows: [[Item; SIZE]; SIZE],
    index: HashMap<u32, (usize, usize)>,
}

impl Board {
    fn mark(&mut self, n: u32) -> &mut Self {
        if let Some((i, j)) = self.index.get_mut(&n) {
            self.rows[*i][*j].state = ItemState::Marked;
        }
        self
    }

    fn bingo(&self) -> bool {
        for row in 0..self.size {
            if self.check_row(row) {
                return true;
            }
        }

        for col in 0..self.size {
            if self.check_col(col) {
                return true;
            }
        }

        false
    }

    fn check_row(&self, i: usize) -> bool {
        for col in 0..self.size {
            if matches!(self.rows[i][col].state, ItemState::Unmarked) {
                return false;
            }
        }
        true
    }

    fn check_col(&self, i: usize) -> bool {
        for row in 0..self.size {
            if matches!(self.rows[row][i].state, ItemState::Unmarked) {
                return false;
            }
        }
        true
    }

    fn sum_unmarked(&self) -> u32 {
        self.rows.iter().fold(0, |sum, row| {
            sum + row
                .iter()
                .filter_map(|item| match item.state {
                    ItemState::Unmarked => Some(item.n),
                    _ => None,
                })
                .sum::<u32>()
        })
    }
}

impl From<&[&str]> for Board {
    fn from(board: &[&str]) -> Self {
        let mut rows = [[Item::default(); SIZE]; SIZE];
        let mut index = HashMap::new();

        board
            .iter()
            .map(|row| row.trim().replace("  ", " "))
            .enumerate()
            .for_each(|(i, row)| {
                row.split(' ')
                    .map(|n| n.parse::<u32>().unwrap())
                    .enumerate()
                    .for_each(|(j, n)| {
                        index.insert(n, (i, j));
                        rows[i][j].n = n;
                    });
            });

        Self {
            size: rows.len(),
            rows,
            index,
        }
    }
}

fn part1(draws: &[u32], boards: &mut [Board]) -> u32 {
    for draw in draws {
        for board in boards.iter_mut() {
            board.mark(*draw);

            if board.bingo() {
                return *draw * board.sum_unmarked();
            }
        }
    }

    unreachable!();
}

fn part2(draws: &[u32], boards: &mut [Board]) -> u32 {
    let mut boards = boards.iter_mut().collect::<Vec<_>>();

    for draw in draws {
        let (bingos, bongos) = boards
            .into_iter()
            .map(|board| board.mark(*draw))
            .partition::<Vec<_>, _>(|board| board.bingo());

        if bingos.len() == 1 && bongos.is_empty() {
            return *draw * bingos[0].sum_unmarked();
        }

        boards = bongos;
    }

    unreachable!();
}

pub fn run(lines: &[&str]) -> (u32, u32) {
    let draws = lines[0]
        .split(',')
        .map(|c| c.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let mut boards = lines[2..]
        .split(|line| line.is_empty())
        .map(Board::from)
        .collect::<Vec<_>>();

    (part1(&draws, &mut boards), part2(&draws, &mut boards))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

        let input = input.split('\n').collect::<Vec<_>>();

        assert_eq!(run(&input[..]), (4512, 1924));
    }
}
