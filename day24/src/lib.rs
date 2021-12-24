//! Library module with all the logic

use std::collections::{VecDeque, HashMap};

use regex::Regex;
use lazy_static::lazy_static;
lazy_static! {
    static ref OPLINE: Regex = Regex::new(r"^(inp|add|mul|div|mod|eql) (w|x|y|z) ?(-?\d+|w|x|y|z)?$").unwrap();
}

type ModNum = [i32; 14];

fn masked_decrease(a: ModNum, mask: ModNum) -> ModNum {
    let mut i = 13;
    let mut new_modnum = a;
    loop {
        if mask[i] == 0 {
            i -= 1;
            continue;
        }
        new_modnum[i] -= 1;
        if new_modnum[i] == 0 {
            new_modnum[i] = 9;
            if i > 0 {
                i -= 1;
                continue;
            }
        }
        break;
    }
    new_modnum
}

fn masked_increase(a: ModNum, mask: ModNum) -> ModNum {
    let mut i = 13;
    let mut new_modnum = a;
    loop {
        if mask[i] == 0 {
            i -= 1;
            continue;
        }
        new_modnum[i] += 1;
        if new_modnum[i] == 10 {
            new_modnum[i] = 1;
            if i > 0 {
                i -= 1;
                continue;
            }
        }
        break;
    }
    new_modnum
}

/// Solve Advent of Code day 24 part one
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, the result for part one of advent of code
/// day 24.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_one(data: &str) -> usize {
    let mut mona: Monad = data.parse::<Monad>().unwrap();
    // Working out the rules
    let mut stack: VecDeque<(usize, i32)> = VecDeque::new();
    let mut correspondances: HashMap<usize, (usize, i32)> = HashMap::new();
    // Mask of numbers that should be tweaked
    let mut morality: ModNum = [0; 14];
    for (i, moral) in morality.iter_mut().enumerate() {
        if mona.params[0][i] == 1 {
            // You're pushing that number onto the stack
            *moral = 1;
            stack.push_front((i, mona.params[2][i]));
        } else {
            let s = stack.pop_front().unwrap();
            correspondances.insert(i, (s.0, s.1 + mona.params[1][i]));
        }
    }
    let mut potential: ModNum = [9; 14];
    loop {
        // Mangle potential
        for (&k, v) in &correspondances {
            potential[k] = potential[v.0] + v.1;
        }
        if potential.iter().all(|x| (1..=9).contains(x)) {
            // It's a valid modnum
            if mona.check(potential) {
                break;
            }
        }
        potential = masked_decrease(potential, morality);
    }
    potential.iter()
        .fold(0, |s, &v| {
            let v: usize = v.try_into().unwrap();
            s * 10 + v
        })
}

enum Instruction {
    Input(char),
    Add(char, char),
    AddV(char, i32),
    Mul(char, char),
    MulV(char, i32),
    Div(char, char),
    DivV(char, i32),
    Mod(char, char),
    ModV(char, i32),
    Eql(char, char),
    EqlV(char, i32)
}

struct Monad {
    params: [[i32; 14]; 3]
}

#[derive(Debug)]
struct MonadParseError { }
impl std::fmt::Display for MonadParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error parsing out MONAD!")
    }
}
impl std::error::Error for MonadParseError {  }

impl std::str::FromStr for Monad {
    type Err = MonadParseError;
    fn from_str(data: &str) -> Result<Self, Self::Err> {
        let v: Vec<Instruction> = data.trim().split('\n')
            .map(|x| {
                let caps = OPLINE.captures(x).unwrap();
                let operand_one = caps[2].chars().next().unwrap();
                let operand_two = caps.get(3);
                match &caps[1] {
                    "inp" => {
                        Instruction::Input(operand_one)
                    },
                    "add" => {
                        if let Ok(v) = operand_two.unwrap().as_str().parse::<i32>() {
                            Instruction::AddV(operand_one, v)
                        } else {
                            Instruction::Add(operand_one, operand_two.unwrap().as_str().chars().next().unwrap())
                        }
                    },
                    "mul" => {
                        if let Ok(v) = operand_two.unwrap().as_str().parse::<i32>() {
                            Instruction::MulV(operand_one, v)
                        } else {
                            Instruction::Mul(operand_one, operand_two.unwrap().as_str().chars().next().unwrap())
                        }
                    },
                    "div" => {
                        if let Ok(v) = operand_two.unwrap().as_str().parse::<i32>() {
                            Instruction::DivV(operand_one, v)
                        } else {
                            Instruction::Div(operand_one, operand_two.unwrap().as_str().chars().next().unwrap())
                        }
                    },
                    "mod" => {
                        if let Ok(v) = operand_two.unwrap().as_str().parse::<i32>() {
                            Instruction::ModV(operand_one, v)
                        } else {
                            Instruction::Mod(operand_one, operand_two.unwrap().as_str().chars().next().unwrap())
                        }
                    },
                    "eql" => {
                        if let Ok(v) = operand_two.unwrap().as_str().parse::<i32>() {
                            Instruction::EqlV(operand_one, v)
                        } else {
                            Instruction::Eql(operand_one, operand_two.unwrap().as_str().chars().next().unwrap())
                        }
                    },
                    _ => unreachable!()
                }
            })
            .collect::<Vec<Instruction>>();
        let mut p1: [i32; 14] = [0; 14];
        let mut p2: [i32; 14] = [0; 14];
        let mut p3: [i32; 14] = [0; 14];
        // Parse out the parameters of the cycles
        let cyclelength = 18;
        for x in 0..14 {
            // Divisor (p1)
            if let Instruction::DivV('z', v) = v[x*cyclelength+4] {
                p1[x] = v;
            }
            // Delta X (p2)
            if let Instruction::AddV('x', v) = v[x*cyclelength+5] {
                p2[x] = v;
            }
            // Delta Y (P3)
            if let Instruction::AddV('y', v) = v[x*cyclelength+15] {
                p3[x] = v;
            }
        }
        //println!("P1: {:?}", p1);
        //println!("P2: {:?}", p2);
        //println!("P3: {:?}", p3);
        Ok(Monad { params: [p1, p2, p3] })
    }
}

impl Monad {
    fn check(&mut self, num: ModNum) -> bool {
        // xn = (((z % 26) + p2) != w)
        // yn = (25 * zn) + 1
        // zn = ( xn * (p3 + w) ) + (yn * z / p1 )
        let mut z = 0;
        let mut y;
        let mut x;
        for (p, &w) in num.iter().enumerate() {
            if self.params[0][p] == 1 {
                assert!(self.params[1][p] >= 10); // that way we're sure it's noop
                z *= 26;
                z += w + self.params[2][p];
            } else {
                x = if (z%26)+self.params[1][p] == w { 0 } else { 1 };
                y = 25*x + 1;
                z = (x * (self.params[2][p] + w)) + (y * z / self.params[0][p]);
            }
        }
        z == 0
    }
}

/// Solve Advent of Code day 24 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, the result for part
/// two of advent of code day 24.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_two(data: &str) -> usize {
    let mut mona: Monad = data.parse::<Monad>().unwrap();
    // Working out the rules
    let mut stack: VecDeque<(usize, i32)> = VecDeque::new();
    let mut correspondances: HashMap<usize, (usize, i32)> = HashMap::new();
    // Mask of numbers that should be tweaked
    let mut morality: ModNum = [0; 14];
    for (i, moral) in morality.iter_mut().enumerate() {
        if mona.params[0][i] == 1 {
            // You're pushing that number onto the stack
            *moral = 1;
            stack.push_front((i, mona.params[2][i]));
        } else {
            let s = stack.pop_front().unwrap();
            correspondances.insert(i, (s.0, s.1 + mona.params[1][i]));
        }
    }
    let mut potential: ModNum = [1; 14];
    loop {
        // Mangle potential
        for (&k, v) in &correspondances {
            potential[k] = potential[v.0] + v.1;
        }
        if potential.iter().all(|x| (1..=9).contains(x)) {
            // It's a valid modnum
            if mona.check(potential) {
                break;
            }
        }
        potential = masked_increase(potential, morality);
    }
    potential.iter()
        .fold(0, |s, &v| {
            let v: usize = v.try_into().unwrap();
            s * 10 + v    
        })
}

