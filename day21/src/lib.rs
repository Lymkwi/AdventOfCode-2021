//! Library module with all the logic

use std::collections::HashMap;

/// Solve Advent of Code day 21 part one
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, the result for part one of advent of code
/// day 21.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_one(data: &str) -> usize {
    let (mut player_one, mut player_two) = unparse_two_players(data);
    // Run
    let mut die: DeterministicDie = DeterministicDie::default();
    loop {
        if player_one.turn(&mut die, 1000) {
            return die.get_usage() * player_two.get_score();
        }
        if player_two.turn(&mut die, 1000) {
            return die.get_usage() * player_one.get_score();
        }
    }
}

fn unparse_two_players(data: &str) -> (Player, Player) {
    let ps = data.split('\n')
        .map(|x| Player::from_str(x))
        .collect::<Vec<Player>>();
    (ps[0], ps[1])
}

struct DeterministicDie {
    cur: usize,
    used: usize
}

impl Default for DeterministicDie {
    fn default() -> DeterministicDie {
        DeterministicDie { cur: 1, used: 0 }
    }
}

impl DeterministicDie {
    fn get_number(&mut self) -> usize {
        let cur = self.cur;
        self.cur += 1;
        self.used += 1;
        if self.cur == 101 { self.cur = 1; }
        cur
    }

    fn get_usage(&self) -> usize {
        self.used
    }
}

#[derive(Clone,Copy,Debug)]
struct Player {
    score: usize,
    position: usize
}

impl Player {
    fn turn(&mut self, f: &mut DeterministicDie, threshold: usize) -> bool {
        for _ in 0..3 {
            self.position = (self.position + f.get_number()%10)%10;
        }
        self.score += self.position+1;
        self.score >= threshold
    }

    fn get_score(&self) -> usize {
        self.score
    }

    fn get_position(&self) -> usize {
        self.position
    }

    fn from_str(data: &str) -> Player {
        let k = data.split(" starting position: ")
            .nth(1).unwrap()
            .parse::<usize>().unwrap();
        Player { score: 0, position: k-1 }
    }
}

type PointInTime = (usize, (usize, usize), (usize, usize));

/// Solve Advent of Code day 21 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, the result for part
/// two of advent of code day 21.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_two(data: &str) -> usize {
    let (player_one, player_two) = unparse_two_players(data);
    // Run
    let pos_one = player_one.get_position();
    let pos_two = player_two.get_position();
    let mut mems: HashMap<PointInTime, (usize, usize)> = HashMap::new();
    // Run
    let (win_one, win_two) = solve_for(0, pos_one, 0, pos_two, 0, &mut mems);
    win_one.max(win_two)
}

fn solve_for(step: usize, p_one: usize, s_one: usize,
             p_two: usize, s_two: usize,
             mem: &mut HashMap<PointInTime, (usize, usize)>) -> (usize, usize) {
    if let Some(&k) = mem.get(&(step, (p_one, s_one), (p_two, s_two))) { k }
    else {
        // Run the simulation
        // Has someone won ?
        if s_one >= 21 { return (1, 0); }
        if s_two >= 21 { return (0, 1); }
        let mut win_one = 0;
        let mut win_two = 0;
        match step {
            0 | 1 | 2 => {
                for x in 1..=3 {
                    let new_pos = (p_one + x)%10;
                    let new_score = s_one + if step == 2 {
                        new_pos + 1 
                    } else {
                        0
                    };
                    let (a, b) = solve_for(step+1, new_pos, new_score,
                                           p_two, s_two, mem);
                    win_one += a;
                    win_two += b;
                }
            },
            3 | 4 | 5 => {
                for x in 1..=3 {
                    let new_pos = (p_two + x)%10;
                    let new_score = s_two + if step == 5 {
                        new_pos + 1
                    } else {
                        0
                    };
                    let new_step = (step + 1)%6;
                    let (a, b) = solve_for(new_step, p_one, s_one, new_pos,
                                           new_score, mem);
                    win_one += a;
                    win_two += b;
                }
            },
            _ => unreachable!()
        }
        mem.insert((step, (p_one, s_one), (p_two, s_two)), (win_one, win_two));
        (win_one, win_two)
    }
}
