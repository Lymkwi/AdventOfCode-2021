//! Library module with all the logic

use std::cmp::{Ord, Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet};

/// Solve Advent of Code day 23 part one
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, the result for part one of advent of code
/// day 23.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_one(data: &str) -> usize {
    // The data is more or less always fixed for that exercise
    let hallway: [Option<Amphipod>; 11] = [None; 11];
    let mut chambers: [[Option<(Amphipod, bool)>; 4]; 4] = [[None; 4]; 4];
    // Parse out our chambers
    let lines = data.trim().split('\n').skip(2)
        .map(|line|
             line.chars()
                .enumerate()
                .filter_map(|(p, x)|
                            if p == 3 || p == 5 || p == 7 || p == 9 {
                                Some(x)
                            } else {
                                None
                            })
                .collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    for y in 0..=1 {
        for (x, chamber) in chambers.iter_mut().enumerate() {
            chamber[y] = Some((match lines[y].get(x) {
                Some('A') => Amphipod::Amber,
                Some('B') => Amphipod::Bronze,
                Some('C') => Amphipod::Copper,
                Some('D') => Amphipod::Desert,
                _ => panic!()
            }, false));
        }
    }
    let mut heap: BinaryHeap<Reverse<AmphipodPuzzleState>> = BinaryHeap::new();
    let mut known: HashSet<[usize; 23]> = HashSet::new();
    let orgstate: AmphipodPuzzleState = AmphipodPuzzleState {
        cost: 0,
        hallway,
        chambers,
        part_one: true
    };
    heap.push(Reverse(orgstate));
    while !heap.is_empty() {
        // Pop a state from the heap
        let Reverse(state) = heap.pop().unwrap();
        let dc = state.state();
        if known.contains(&dc) { continue; }
        known.insert(dc);
        // Ok is this a winning state?
        assert!(state.is_valid());
        if state.is_winning() {
            return state.cost;
        }
        for new_state in state.next_states() {
            heap.push(Reverse(new_state));
        }
    }
    panic!("I didn't find an exit");
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert
}

impl Amphipod {
    #[must_use]
    fn cost(self) -> usize {
        match self {
            Amphipod::Amber => 1,
            Amphipod::Bronze => 10,
            Amphipod::Copper => 100,
            Amphipod::Desert => 1000
        }
    }

    #[must_use]
    fn room_no(self) -> usize {
        match self {
            Amphipod::Amber => 0,
            Amphipod::Bronze => 1,
            Amphipod::Copper => 2,
            Amphipod::Desert => 3
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct AmphipodPuzzleState {
    cost: usize,
    hallway: [Option<Amphipod>; 11],
    chambers: [[Option<(Amphipod, bool)>; 4]; 4],
    part_one: bool
}

impl std::fmt::Display for AmphipodPuzzleState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut st: String = "#############\n#".into();
        for x in self.hallway {
            st.push(match x {
                None => '.',
                Some(ampod) => match ampod {
                    Amphipod::Amber => 'A',
                    Amphipod::Bronze => 'B',
                    Amphipod::Copper => 'C',
                    Amphipod::Desert => 'D'
                }
            });
        }
        st += "#\n";
        let mdepth = if self.part_one { 1 } else { 3 };
        for y in 0..=mdepth {
            st += "###";
            for x in 0..=3 {
                st.push(match self.chambers[x][y] {
                    None => '.',
                    Some((ampod, state)) => match ampod {
                        Amphipod::Amber => if state { 'A' } else { 'a' },
                        Amphipod::Bronze => if state { 'B' } else { 'b' },
                        Amphipod::Copper => if state { 'C' } else { 'c' },
                        Amphipod::Desert => if state { 'D' } else { 'd' }
                    }
                });
                st.push('#');
            }
            st += "##\n";
        }
        write!(f, "{}###{:07}###", st, self.cost)
    }
}

fn can_i_go(org: usize, dst: usize, hallway: [Option<Amphipod>; 11]) -> bool {
    let min_pos = org.min(dst);
    let max_pos = org.max(dst);
    (min_pos..=max_pos).all(|p| hallway[p].is_none())
}

impl AmphipodPuzzleState {
    fn state(&self) -> [usize; 23] {
        let mut k: [usize; 23] = [0; 23];
        let mut i = 0;
        for x in [0, 1, 3, 5, 7, 9, 10] {
            k[i] = match self.hallway[x] {
                Some(a) => a.room_no() + 1,
                None => 0
            };
            i+=1;
        }
        for chamber in self.chambers {
            for spot in chamber {
                k[i] = match spot {
                    Some((a,b)) => a.room_no() + if b { 5 } else { 1 },
                    None => 0
                };
                i+=1;
            }
        }
        k
    }

    fn is_valid(&self) -> bool {
        // Assert the number of amphipods
        let h_count = self.hallway.iter().filter(|x| x.is_some()).count();
        let c_count = self.chambers.iter()
            .map(|c| c.iter().filter(|x| x.is_some()).count())
            .sum::<usize>();
        h_count + c_count == if self.part_one { 8 } else { 16 }
    }
    fn is_winning(&self) -> bool {
        // For each chamber
        (0..4).all(|x|
            // Both amphipods are the same
            self.chambers[x][0].is_some()
            && self.chambers[x][1].is_some()
            && self.chambers[x][0].unwrap().0
                == self.chambers[x][1].unwrap().0
        )
    }

    fn next_states(&self) -> Vec<AmphipodPuzzleState> {
        let mut res: Vec<AmphipodPuzzleState> = Vec::new();
        // Find all the shrimps that can move out of a chamber
        let depth = if self.part_one { 2 } else { 4 };
        for x in 0..=3 {
            for spot in 0..depth {
                // Look at the out spot
                if let Some((ampod, state)) = self.chambers[x][spot] {
                    // If we're already in our room, don't effing move
                    if spot == depth-1 && x == ampod.room_no() { break; }
                    if !state {
                        // We haven't left out room yet
                        for np in [0, 1, 3, 5, 7, 9, 10]  {
                            if !can_i_go(2+2*x, np, self.hallway) {
                                continue; 
                            }
                            // Move out
                            let mut nstate = *self;
                            nstate.chambers[x][spot] = None;
                            nstate.hallway[np] = Some(ampod);
                            // How much did we move
                            let cost_add = ampod.cost() *
                                (1 + spot + np.max(2+2*x) - np.min(2+2*x));
                            nstate.cost += cost_add;
                            res.push(nstate);
                        }
                    }
                    break;
                }
            }
        }
        // Find all the shrimps that can move into a chamber
        for p in [0, 1, 3, 5, 7, 9, 10] {
            if let Some(ampod) = self.hallway[p] {
                // Where is my room ?
                let r = ampod.room_no();
                let my_room_position = 2 + 2 * r;
                // If I can't go to my room it's pointless
                if !can_i_go(match p.cmp(&my_room_position) {
                    Ordering::Equal => panic!("impossible"),
                    Ordering::Less => p+1,
                    Ordering::Greater => p-1
                }, my_room_position, self.hallway) { continue; }
                // How busy is my room ?
                let depth_busy = (0..depth)
                    .filter(|&x| self.chambers[r][x].is_some())
                    .min().unwrap_or(depth);
                // Is there only my type in the room ?
                if !(depth_busy..depth)
                    .all(|x| self.chambers[r][x].unwrap().0 == ampod) {
                        continue;
                    }
                // Move back into my room
                let mut nstate = *self;
                nstate.hallway[p] = None;
                nstate.chambers[r][depth_busy-1] = Some((ampod, true));
                let new_cost = ampod.cost() *
                    (depth_busy + my_room_position.max(p) - my_room_position.min(p));
                nstate.cost += new_cost;
                res.push(nstate);
            }
        }
        res
    }
}

impl PartialEq for AmphipodPuzzleState {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost &&
        (0..11).all(|p| self.hallway[p] == other.hallway[p]) &&
        (0..4).all(|hn| {
            let depth = if self.part_one { 2 } else { 4 };
            (0..depth).all(|pos| {
                self.chambers[hn][pos] == other.chambers[hn][pos]
            })
        })
    }
}

impl Eq for AmphipodPuzzleState {}

impl PartialOrd for AmphipodPuzzleState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AmphipodPuzzleState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
            .then_with(|| {
                for x in [0, 1, 3, 5, 7, 9, 10] {
                    if self.hallway[x] != other.hallway[x] {
                        let my = self.hallway[x];
                        let theirs = other.hallway[x];
                        if my.is_none() { return Ordering::Less; }
                        else if theirs.is_none() { return Ordering::Greater; }
                        let my = my.unwrap();
                        let theirs = theirs.unwrap();
                        if my != theirs { return my.cmp(&theirs); }
                    }
                }
                // Chambers
                let mdepth = if self.part_one { 1 } else { 3 };
                for c in 0..=3 {
                    for s in 0..=mdepth {
                        let my = self.chambers[c][s];
                        let theirs = self.chambers[c][s];
                        if my.is_none() { return Ordering::Less; }
                        else if theirs.is_none() { return Ordering::Greater; }
                        let my = my.unwrap();
                        let theirs = theirs.unwrap();
                        if my != theirs { return my.cmp(&theirs); }
                    }
                }
                Ordering::Equal
        })
    }
}

/// Solve Advent of Code day 23 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, the result for part
/// two of advent of code day 23.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_two(data: &str) -> usize {
    // The data is more or less always fixed for that exercise
    let hallway: [Option<Amphipod>; 11] = [None; 11];
    let mut chambers: [[Option<(Amphipod, bool)>; 4]; 4] = [[None; 4]; 4];
    // Parse out our chambers
    let lines = data.trim().split('\n').skip(2)
        .map(|line|
             line.chars()
                .enumerate()
                .filter_map(|(p, x)|
                            if p == 3 || p == 5 || p == 7 || p == 9 {
                                Some(x)
                            } else {
                                None
                            })
                .collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    for y in 0..=1 {
        for (x, chamber) in chambers.iter_mut().enumerate() {
            chamber[y] = Some((match lines[y].get(x) {
                Some('A') => Amphipod::Amber,
                Some('B') => Amphipod::Bronze,
                Some('C') => Amphipod::Copper,
                Some('D') => Amphipod::Desert,
                _ => panic!()
            }, false));
        }
    }
    // Move the data over
    for chamber in &mut chambers {
        chamber[3] = chamber[1];
    }
    // Insert missing data
    chambers[0][1] = Some((Amphipod::Desert, false));
    chambers[0][2] = Some((Amphipod::Desert, false));
    chambers[1][1] = Some((Amphipod::Copper, false));
    chambers[1][2] = Some((Amphipod::Bronze, false));
    chambers[2][1] = Some((Amphipod::Bronze, false));
    chambers[2][2] = Some((Amphipod::Amber, false));
    chambers[3][1] = Some((Amphipod::Amber, false));
    chambers[3][2] = Some((Amphipod::Copper, false));
    let mut heap: BinaryHeap<Reverse<AmphipodPuzzleState>> = BinaryHeap::new();
    let mut known: HashSet<[usize; 23]> = HashSet::new();
    let orgstate: AmphipodPuzzleState = AmphipodPuzzleState {
        cost: 0,
        hallway,
        chambers,
        part_one: false
    };
    heap.push(Reverse(orgstate));
    while !heap.is_empty() {
        // Pop a state from the heap
        let Reverse(state) = heap.pop().unwrap();
        if known.contains(&state.state()) { continue; }
        known.insert(state.state());
        // Ok is this a winning state?
        assert!(state.is_valid());
        if state.is_winning() {
            return state.cost;
        }
        for new_state in state.next_states() {
            heap.push(Reverse(new_state));
        }
    }
    panic!("I didn't find an exit");
}

