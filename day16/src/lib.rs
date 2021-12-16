//! Library module with all the logic

/// Solve Advent of Code day 16 part one
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, the result for part one of advent of code
/// day 16.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_one(data: &str) -> usize {
    sum_version(&parse_packet(&unbuild(data), 0).0)
}

fn parse_packet(tbe: &[bool], pos: usize) -> (Packet, usize) {
    // We should be at the head of a Packet
    let version = pack_bits(&tbe[pos..pos+3]);
    let ptypev = pack_bits(&tbe[pos+3..pos+6]);
    let mut delta: usize = 6;
    // Get data for the ptype
    if ptypev == 4 {
        // Decode the literal
        let (literal, dd): (usize, usize) = break_literal(tbe, pos+delta);
        (Packet {
            version,
            ptype: PacketType::Literal(literal),
            tree: None
        }, delta+dd)
    } else {
        let ltypeid = tbe[pos+delta];
        delta += 1;
        let mut treevec: Vec<Packet> = Vec::new();
        if ltypeid {
            let subpcount = pack_bits(&tbe[pos+delta..pos+delta+11]);
            delta += 11;
            // As long as we haven't parsed enough packets, keep going
            while treevec.len() < subpcount {
                let (subp, dd) = parse_packet(&tbe[pos+delta..], 0);
                delta += dd;
                treevec.push(subp);
            }
        } else {
            // Get the length in bits of the subpackets section
            let subplength = pack_bits(&tbe[pos+delta..pos+delta+15]);
            delta += 15;
            // As long as we haven't reached the subplength, keep parsing
            let mut subpdelta: usize = 0;
            while subpdelta < subplength {
                let (subp, dd): (Packet, usize) = parse_packet(
                    &tbe[pos+delta+subpdelta..], 0);
                subpdelta += dd;
                treevec.push(subp);
            }
            if subpdelta > subplength { panic!("Overshot"); }
            delta += subplength;
        }
        (Packet {
            version,
            ptype: getoptype(ptypev),
            tree: Some(treevec)
        }, delta)
    }
}

fn getoptype(v: usize) -> PacketType {
    match v {
        0 => PacketType::Sum,
        1 => PacketType::Product,
        2 => PacketType::Minimum,
        3 => PacketType::Maximum,
        5 => PacketType::Greater,
        6 => PacketType::Less,
        7 => PacketType::Equal,
        _ => unreachable!()
    }
}

fn break_literal(tbe: &[bool], pos: usize) -> (usize, usize) {
    // Inspect the groups
    let mut delta = 0;
    let mut obtained = false;
    let mut bit_array = Vec::new();
    while !obtained {
        // Is it the final segment?
        if !tbe[pos+delta] { obtained = true; }
        bit_array.extend(&tbe[pos+delta+1..pos+delta+5]);
        delta += 5;
    }
    (pack_bits(&bit_array), delta)
}

fn pack_bits(bits: &[bool]) -> usize {
    bits.iter()
        .fold(0, |s, &t| s*2 + if t { 1 } else { 0 })
}

fn unbuild(data: &str) -> Vec<bool> {
    data.chars()
        .flat_map(|x| {
            let k = usize::from_str_radix(&String::from(x), 16).unwrap();
            vec![
                k & 0b1000 != 0,
                k & 0b0100 != 0,
                k & 0b0010 != 0,
                k & 0b0001 != 0
            ]
        })
        .collect::<Vec<bool>>()
}

#[derive(Clone, Copy)]
enum PacketType {
    Sum,
    Product,
    Minimum,
    Maximum,
    Literal(usize),
    Greater,
    Less,
    Equal
}

struct Packet {
    version: usize,
    ptype: PacketType,
    tree: Option<Vec<Packet>>
}

impl Packet {
    fn get_version(&self) -> usize {
        self.version
    }

    fn get_content(&self) -> PacketType {
        self.ptype
    }

    fn get_tree(&self) -> &Vec<Packet> {
        self.tree.as_ref().unwrap()
    }

    fn value(&self) -> usize {
        match self.ptype {
            PacketType::Sum => self.tree.as_ref().unwrap()
                .iter().map(Packet::value).sum::<usize>(),
            PacketType::Product => self.tree.as_ref().unwrap()
                .iter().map(Packet::value).product::<usize>(),
            PacketType::Minimum => self.tree.as_ref().unwrap()
                .iter().map(Packet::value).min().unwrap(),
            PacketType::Maximum => self.tree.as_ref().unwrap()
                .iter().map(Packet::value).max().unwrap(),
            PacketType::Literal(v) => v,
            PacketType::Greater => {
                let tree = self.tree.as_ref().unwrap();
                let a = tree.get(0).unwrap().value();
                let b = tree.get(1).unwrap().value();
                if a > b { 1 } else { 0 }
            },
            PacketType::Less => {
                let tree = self.tree.as_ref().unwrap();
                let a = tree.get(0).unwrap().value();
                let b = tree.get(1).unwrap().value();
                if a < b { 1 } else { 0 }
            },
            PacketType::Equal => {
                let tree = self.tree.as_ref().unwrap();
                let a = tree.get(0).unwrap().value();
                let b = tree.get(1).unwrap().value();
                if a == b { 1 } else { 0 }
            }
        }
    }
}

fn sum_version(p: &Packet) -> usize {
    p.get_version() + match p.get_content() {
        PacketType::Literal(_) => 0,
        _ => p.get_tree().iter()
            .map(|x| sum_version(x))
            .sum::<usize>()
    }
}

/// Solve Advent of Code day 16 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, the result for part
/// two of advent of code day 16.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_two(data: &str) -> usize {
    parse_packet(&unbuild(data), 0).0.value()
}

