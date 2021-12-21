use std::{collections::HashMap, num::ParseIntError, str::FromStr};

pub fn run(players: Vec<Player>) -> (usize, usize) {
    (part1(players.clone()), part2(players))
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Hash, Eq)]
pub struct Player {
    pos: u8,
    score: usize,
}

impl FromStr for Player {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.trim().split(' ').collect::<Vec<_>>();
        let pos = parts[4].parse::<u8>()?;
        let score = 0;

        Ok(Self { pos, score })
    }
}

struct Dice {
    rolls: usize,
    last: usize,
}

impl Dice {
    fn new() -> Self {
        Self { rolls: 0, last: 0 }
    }

    fn roll(&mut self) -> usize {
        self.rolls += 1;

        if self.last + 1 == 101 {
            self.last = 0;
        }

        self.last += 1;
        self.last
    }
}

fn part1(mut players: Vec<Player>) -> usize {
    let mut dice = Dice::new();

    loop {
        for p in players.iter_mut() {
            let keep = dice.roll();
            let on = dice.roll();
            let rollin = dice.roll();

            let add = keep + on + rollin;
            let score = (p.pos as usize - 1 + add) % 10 + 1;

            p.score += score as usize;
            p.pos = score as u8;

            if p.score >= 1000 {
                break;
            }
        }

        let winner = players.iter().find(|p| p.score >= 1000);
        let looser = players
            .iter()
            .reduce(|res, next| if next.score < res.score { next } else { res })
            .unwrap();

        if let Some(_) = winner {
            return looser.score * dice.rolls;
        }
    }
}

fn part2(players: Vec<Player>) -> usize {
    assert_eq!(players.len(), 2);

    let mut p1 = players[0];
    let mut p2 = players[1];

    let mut game_states = HashMap::new();
    p1.pos -= 1;
    p2.pos -= 1;

    let (wins1, wins2) = play(p1, p2, &mut game_states);

    return wins1.max(wins2);

    fn play(
        current: Player,
        next: Player,
        game_states: &mut HashMap<(Player, Player), (usize, usize)>,
    ) -> (usize, usize) {
        // 21 is the winning score
        if current.score >= 21 {
            return (1, 0);
        }
        if next.score >= 21 {
            return (0, 1);
        }

        // retrieve if already computed
        if let Some(state) = game_states.get(&(current, next)) {
            return *state;
        }

        // compute for given state
        let mut current_wins_total = 0;
        let mut next_wins_total = 0;

        for keep in [1, 2, 3] {
            for on in [1, 2, 3] {
                for rollin in [1, 2, 3] {
                    let mut player = current;
                    player.pos = (current.pos + keep + on + rollin) % 10;
                    player.score = current.score + player.pos as usize + 1;

                    let (next_wins, current_wins) = play(next, player, game_states);
                    current_wins_total += current_wins;
                    next_wins_total += next_wins;
                }
            }
        }

        game_states.insert((current, next), (current_wins_total, next_wins_total));

        (current_wins_total, next_wins_total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Player 1 starting position: 4
                           Player 2 starting position: 8"#;

    #[test]
    fn test_parse() {
        let players = INPUT
            .split('\n')
            .map(|line| line.parse::<Player>().unwrap())
            .collect::<Vec<_>>();

        assert_eq!(
            players,
            vec![Player { pos: 4, score: 0 }, Player { pos: 8, score: 0 },]
        )
    }

    #[test]
    fn test_dice() {
        let mut dice = Dice::new();

        assert_eq!(dice.roll(), 1);
        assert_eq!(dice.roll(), 2);
        assert_eq!(dice.roll(), 3);
    }

    #[test]
    fn test_dice_loop() {
        let mut dice = Dice::new();

        for i in 0..1000 {
            assert_eq!(dice.roll(), i % 100 + 1);
        }
    }

    #[test]
    fn test_part1() {
        let players = INPUT
            .split('\n')
            .map(|line| line.parse::<Player>().unwrap())
            .collect::<Vec<_>>();

        assert_eq!(part1(players), 739785);
    }

    #[test]
    fn test_part2() {
        let players = INPUT
            .split('\n')
            .map(|line| line.parse::<Player>().unwrap())
            .collect::<Vec<_>>();

        assert_eq!(part2(players), 444356092776315);
    }
}
