use std::{collections::HashMap, fmt::Display};

pub fn run() -> (usize, usize) {
    let input1 = State::new(
        ['.'; 11],
        [
            ['A', 'B', 'A', 'A'],
            ['D', 'C', 'B', 'B'],
            ['A', 'D', 'C', 'C'],
            ['B', 'C', 'D', 'D'],
        ],
    );

    let input2 = State::new(
        ['.'; 11],
        [
            ['A', 'D', 'D', 'B'],
            ['D', 'C', 'B', 'C'],
            ['A', 'B', 'A', 'D'],
            ['B', 'A', 'C', 'C'],
        ],
    );
    (search(input1), search(input2))
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    corridor: [char; 11],
    rooms: [[char; 4]; 4],
    locks: [[bool; 4]; 4],
}

type MoveOut = ((usize, usize), usize);
type MoveIn = (usize, (usize, usize));

impl State {
    const ROOM_MAP: [char; 4] = ['A', 'B', 'C', 'D'];
    const CORRIDOR_SLOTS: [usize; 7] = [0, 1, 3, 5, 7, 9, 10];

    fn new(corridor: [char; 11], rooms: [[char; 4]; 4]) -> Self {
        Self {
            corridor,
            rooms,
            locks: [[false; 4]; 4],
        }
    }

    fn is_room_solved(&self, room: usize) -> bool {
        self.rooms[room].iter().all(|e| *e == Self::ROOM_MAP[room])
    }

    fn is_room_full(&self, room: usize) -> bool {
        self.rooms[room].iter().all(|e| *e != '.')
    }

    fn is_room_empty(&self, room: usize) -> bool {
        self.rooms[room].iter().all(|e| *e == '.')
    }

    fn is_solved(&self) -> bool {
        (0..self.rooms.len()).all(|r| self.is_room_solved(r))
    }

    fn move_out(&self, ((room, slot), to): MoveOut) -> Self {
        let mut res = self.clone();
        res.corridor[to] = self.rooms[room][slot];
        res.rooms[room][slot] = '.';
        res
    }

    fn move_in(&self, (from, (room, slot)): MoveIn) -> Self {
        let mut res = self.clone();
        res.corridor[from] = '.';
        res.rooms[room][slot] = self.corridor[from];
        res.locks[room][slot] = true;
        res
    }

    fn possible_out_moves(&self) -> Vec<(MoveOut, usize)> {
        let mut moves_out = vec![];

        self.possible_moves_from_room(0)
            .map(|mvs| moves_out.extend(mvs));
        self.possible_moves_from_room(1)
            .map(|mvs| moves_out.extend(mvs));
        self.possible_moves_from_room(2)
            .map(|mvs| moves_out.extend(mvs));
        self.possible_moves_from_room(3)
            .map(|mvs| moves_out.extend(mvs));

        moves_out
    }

    fn possible_in_moves(&self) -> Vec<(MoveIn, usize)> {
        Self::CORRIDOR_SLOTS
            .iter()
            .filter(|corridor| self.corridor[**corridor] != '.')
            .filter_map(|corridor| self.possible_move_to_room(*corridor))
            .collect::<Vec<_>>()
    }

    fn possible_move_to_room(&self, corridor: usize) -> Option<(MoveIn, usize)> {
        let element = self.corridor[corridor];
        let room = (element as u8 - b'A') as usize;

        if self.is_room_full(room) {
            return None;
        }

        // is the corridor free up until the room?
        let is_valid_move = if corridor < ((room + 1) * 2) {
            self.is_valid_move(room, corridor + 1)
        } else {
            self.is_valid_move(room, corridor - 1)
        };

        if !is_valid_move {
            return None;
        }

        if !self.rooms[room]
            .iter()
            .all(|e| *e == '.' || *e == Self::ROOM_MAP[room])
        {
            return None;
        }

        let slot = self.rooms[room]
            .iter()
            .enumerate()
            .find(|(_, e)| **e != '.')
            .map(|(slot, _)| slot)
            .unwrap_or(self.rooms[room].len())
            - 1;

        let cost = self.cost(element) * Self::distance(((room, slot), corridor));

        Some(((corridor, (room, slot)), cost))
    }

    fn possible_moves_from_room(&self, room: usize) -> Option<Vec<(MoveOut, usize)>> {
        if self.is_room_solved(room) || self.is_room_empty(room) {
            return None;
        }
        // find first entry to move
        let (slot, _) = self.rooms[room]
            .iter()
            .enumerate()
            .find(|(_, e)| **e != '.')
            .unwrap();

        // If elements are already in the correct
        // room, we don't need to move anything out.
        if self.rooms[room][slot..]
            .iter()
            .all(|e| *e == Self::ROOM_MAP[room])
        {
            return None;
        }

        // create all valid moves based on the current state
        let moves = Self::CORRIDOR_SLOTS
            .iter()
            .map(|to| ((room, slot), *to))
            .filter(|((room, slot), _)| !self.locks[*room][*slot])
            .filter(|((room, _), corridor)| self.is_valid_move(*room, *corridor))
            .map(|mv @ ((room, slot), _)| {
                (mv, self.cost(self.rooms[room][slot]) * Self::distance(mv))
            })
            .collect::<Vec<_>>();

        Some(moves)
    }

    fn distance(((room, slot), corridor): MoveOut) -> usize {
        ((room + 1) * 2).abs_diff(corridor) + 1 + slot
    }

    fn cost(&self, e: char) -> usize {
        match e {
            'A' => 1,
            'B' => 10,
            'C' => 100,
            'D' => 1000,
            _ => 0,
        }
    }

    fn draw(&self) {
        println!("{}", self);
    }

    fn is_valid_move(&self, room: usize, corridor: usize) -> bool {
        let top = (room + 1) * 2;
        let lo = corridor.min(top);
        let hi = corridor.max(top);

        (lo..=hi).all(|s| self.corridor[s] == '.')
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.corridor.iter().collect::<String>())?;
        writeln!(
            f,
            "  {} {} {} {}",
            self.rooms[0][0], self.rooms[1][0], self.rooms[2][0], self.rooms[3][0]
        )?;
        writeln!(
            f,
            "  {} {} {} {}",
            self.rooms[0][1], self.rooms[1][1], self.rooms[2][1], self.rooms[3][1]
        )?;

        Ok(())
    }
}

fn search(state: State) -> usize {
    let mut states = HashMap::new();
    simulate(state, &mut states)
}

fn simulate(state: State, states: &mut HashMap<State, usize>) -> usize {
    if state.is_solved() {
        return 0;
    }

    if let Some(cost) = states.get(&state) {
        return *cost;
    }

    let mut costs = vec![];

    for (mv, cost) in state.possible_out_moves() {
        let new_state = state.move_out(mv);
        let current_cost = simulate(new_state, states);
        costs.push(current_cost.saturating_add(cost));
    }

    for (mv, cost) in state.possible_in_moves() {
        let new_state = state.move_in(mv);
        let current_cost = simulate(new_state, states);
        costs.push(current_cost.saturating_add(cost));
    }

    let local_min = costs.iter().copied().min().unwrap_or(usize::MAX);

    states.insert(state, local_min);

    return local_min;
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    use super::*;

    #[test]
    fn test_steps() {
        assert_eq!(State::distance(((0, 0), 1)), 2);
        assert_eq!(State::distance(((0, 0), 3)), 2);
        assert_eq!(State::distance(((0, 0), 10)), 9);
        assert_eq!(State::distance(((3, 1), 0)), 10);
    }

    #[test]
    fn test_moves_from_room() {
        let state = State::new(
            ['.'; 11],
            [
                ['A', 'B', 'A', 'A'],
                ['D', 'C', 'B', 'B'],
                ['A', 'D', 'C', 'C'],
                ['B', 'C', 'D', 'D'],
            ],
        );

        let actual = state.possible_moves_from_room(0).unwrap();

        assert_eq!(
            actual,
            vec![
                (((0, 0), 0), 3),
                (((0, 0), 1), 2),
                (((0, 0), 3), 2),
                (((0, 0), 5), 4),
                (((0, 0), 7), 6),
                (((0, 0), 9), 8),
                (((0, 0), 10), 9)
            ]
        );
    }

    #[test]
    fn test_moves_from_room_blocking_corridor() {
        let mut corridor = ['.'; 11];
        corridor[1] = 'B';
        corridor[7] = 'C';

        let state = State::new(
            corridor,
            [
                ['.', 'B', 'A', 'A'],
                ['D', 'C', 'B', 'B'],
                ['A', 'D', 'C', 'C'],
                ['B', 'C', 'D', 'D'],
            ],
        );

        let actual = state.possible_moves_from_room(0).unwrap();

        assert_eq!(actual, vec![(((0, 1), 3), 30), (((0, 1), 5), 50)]);
    }

    #[test]
    fn test_moves_from_room_no_move() {
        let state = State::new(
            ['.'; 11],
            [
                ['.', 'A', 'A', 'A'],
                ['B', 'B', 'B', 'B'],
                ['.', 'C', 'C', 'C'],
                ['D', 'D', 'D', 'D'],
            ],
        );

        assert!(state.possible_moves_from_room(0).is_none());
        assert!(state.possible_moves_from_room(1).is_none());
        assert!(state.possible_moves_from_room(2).is_none());
        assert!(state.possible_moves_from_room(3).is_none());
    }

    #[test]
    fn test_moves_out() {
        let state = State::new(
            ['.'; 11],
            [
                ['B', 'A', 'A', 'A'],
                ['C', 'D', 'B', 'B'],
                ['B', 'C', 'C', 'C'],
                ['D', 'A', 'D', 'D'],
            ],
        );

        let actual = state.possible_out_moves();

        assert_eq!(
            actual,
            vec![
                (((0, 0), 0), 30),
                (((0, 0), 1), 20),
                (((0, 0), 3), 20),
                (((0, 0), 5), 40),
                (((0, 0), 7), 60),
                (((0, 0), 9), 80),
                (((0, 0), 10), 90),
                (((1, 0), 0), 500),
                (((1, 0), 1), 400),
                (((1, 0), 3), 200),
                (((1, 0), 5), 200),
                (((1, 0), 7), 400),
                (((1, 0), 9), 600),
                (((1, 0), 10), 700),
                (((2, 0), 0), 70),
                (((2, 0), 1), 60),
                (((2, 0), 3), 40),
                (((2, 0), 5), 20),
                (((2, 0), 7), 20),
                (((2, 0), 9), 40),
                (((2, 0), 10), 50),
                (((3, 0), 0), 9000),
                (((3, 0), 1), 8000),
                (((3, 0), 3), 6000),
                (((3, 0), 5), 4000),
                (((3, 0), 7), 2000),
                (((3, 0), 9), 2000),
                (((3, 0), 10), 3000)
            ]
        );
    }

    #[test]
    fn test_moves_in() {
        let mut corridor = ['.'; 11];
        corridor[0] = 'A';
        corridor[7] = 'B';
        corridor[10] = 'D';

        let state = State::new(
            corridor,
            [
                ['.', 'A', 'A', 'A'],
                ['.', 'B', 'B', 'B'],
                ['C', 'C', 'C', 'C'],
                ['.', '.', 'D', 'D'],
            ],
        );

        let actual = state.possible_in_moves();

        assert_eq!(
            actual,
            vec![((0, (0, 0)), 3), ((7, (1, 0)), 40), ((10, (3, 1)), 4000)]
        );
    }

    #[test]
    fn test_moves_in_example() {
        let mut corridor = ['.'; 11];
        corridor[5] = 'D';
        corridor[7] = 'D';
        corridor[9] = 'A';

        let state = State::new(
            corridor,
            [
                ['.', 'A', 'A', 'A'],
                ['B', 'B', 'B', 'B'],
                ['C', 'C', 'C', 'C'],
                ['.', '.', 'D', 'D'],
            ],
        );

        assert_eq!(state.possible_in_moves(), vec![((7, (3, 1)), 3000)]);
        let state = state.move_in((7, (3, 1)));
        assert_eq!(state.possible_in_moves(), vec![((5, (3, 0)), 4000)]);
        let state = state.move_in((5, (3, 0)));
        assert_eq!(state.possible_in_moves(), vec![((9, (0, 0)), 8)]);
        let state = state.move_in((9, (0, 0)));
        assert_eq!(state.possible_in_moves(), vec![]);
    }

    #[test]
    fn test_example() {
        let state = State::new(
            ['.'; 11],
            [
                ['B', 'A', 'A', 'A'],
                ['C', 'D', 'B', 'B'],
                ['B', 'C', 'C', 'C'],
                ['D', 'A', 'D', 'D'],
            ],
        );

        let moves = state.possible_out_moves();
        assert!(moves.contains(&(((2, 0), 3), 40)));
        let state = state.move_out(((2, 0), 3));

        let moves = state.possible_out_moves();
        assert!(moves.contains(&(((1, 0), 5), 200)));
        let state = state.move_out(((1, 0), 5));

        let moves = state.possible_in_moves();
        assert!(moves.contains(&((5, (2, 0)), 200)));
        let state = state.move_in((5, (2, 0)));

        let moves = state.possible_out_moves();
        assert!(moves.contains(&(((1, 1), 5), 3000)));
        let state = state.move_out(((1, 1), 5));

        let moves = state.possible_in_moves();
        assert!(moves.contains(&((3, (1, 1)), 30)));
        let state = state.move_in((3, (1, 1)));

        let moves = state.possible_out_moves();
        assert!(moves.contains(&(((0, 0), 3), 20)));
        let state = state.move_out(((0, 0), 3));

        let moves = state.possible_in_moves();
        assert!(moves.contains(&((3, (1, 0)), 20)));
        let state = state.move_in((3, (1, 0)));

        let moves = state.possible_out_moves();
        assert!(moves.contains(&(((3, 0), 7), 2000)));
        let state = state.move_out(((3, 0), 7));

        let moves = state.possible_out_moves();
        assert!(moves.contains(&(((3, 1), 9), 3)));
        let state = state.move_out(((3, 1), 9));

        let moves = state.possible_in_moves();
        assert!(moves.contains(&((7, (3, 1)), 3000)));
        let state = state.move_in((7, (3, 1)));

        let moves = state.possible_in_moves();
        assert!(moves.contains(&((5, (3, 0)), 4000)));
        let state = state.move_in((5, (3, 0)));

        let moves = state.possible_in_moves();
        assert!(moves.contains(&((9, (0, 0)), 8)));
        let state = state.move_in((9, (0, 0)));

        assert!(state.is_solved());
    }

    #[test]
    fn test_part1() {
        let state = State::new(
            ['.'; 11],
            [
                ['B', 'A', 'A', 'A'],
                ['C', 'D', 'B', 'B'],
                ['B', 'C', 'C', 'C'],
                ['D', 'A', 'D', 'D'],
            ],
        );
        // let state = State::new(['.'; 11], [['A', 'B'], ['D', 'C'], ['A', 'D'], ['B', 'C']]);

        assert_eq!(search(state), 12521);
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input1 = State::new(
            ['.'; 11],
            [
                ['A', 'B', 'A', 'A'],
                ['D', 'C', 'B', 'B'],
                ['A', 'D', 'C', 'C'],
                ['B', 'C', 'D', 'D'],
            ],
        );

        b.iter(|| assert_eq!(search(input1), 13455));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input2 = State::new(
            ['.'; 11],
            [
                ['A', 'D', 'D', 'B'],
                ['D', 'C', 'B', 'C'],
                ['A', 'B', 'A', 'D'],
                ['B', 'A', 'C', 'C'],
            ],
        );

        b.iter(|| assert_eq!(search(input2), 43567));
    }
}
