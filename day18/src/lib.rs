//! Library module with all the logic

/// Solve Advent of Code day 18 part one
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, the result for part one of advent of code
/// day 18.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_one(data: &str) -> usize {
    // Test
    let n1 = unparse_snail("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]");
    let n2 = unparse_snail("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]");
    assert_eq!("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
               format!("{}", n1+n2));
    let result: SnailNumber = data.trim().split('\n')
        .map(|x| unparse_snail(x))
        .fold(None, |res, k| res.map_or(Some(k), |v| Some(v + k)))
        .unwrap();
    //println!("Managed to sum: {}", result);
    /* btw solve this */
    result.magnitude()
}

fn unparse_snail(data: &str) -> SnailNumber {
    let mut snail: [SnailItem; 64] = [SnailItem::None; 64];
    let mut curpos = 1;
    for c in data.chars() {
        match c {
            '[' => {
                // Enter next depth, left
                // This means the current position is the head
                // Of a snail number
                snail[curpos] = SnailItem::Snail;
                curpos *= 2;
            },
            ']' => {
                // Close current depth
                curpos >>= 1;
            },
            ',' => {
                // Next in depth
                curpos += 1;
            },
            n  => {
                // This is hopefully a number
                snail[curpos] = SnailItem::Regular(
                    String::from(n).parse::<usize>().unwrap());
            }
        }
    }
    SnailNumber { matrix: snail }
}

fn bit_height(a: usize) -> usize {
    let mut n = 6;
    while n > 0 && a & (1<<n) == 0 { n -= 1; }
    n
}

fn is_left(a: usize, b: usize) -> bool {
    // Which is the smallest
    let b_height_a = bit_height(a);
    let b_height_b = bit_height(b);
    match (b_height_a < b_height_b, b_height_a > b_height_b) {
        (false, false) => a < b,
        (false, true) => {
            let mut new_b = b;
            for _ in b_height_b..b_height_a { new_b *= 2; }
            a < new_b
        },
        (true, false) => {
            let mut new_a = a;
            for _ in b_height_a..b_height_b { new_a *= 2; }
            new_a < b
        },
        _ => unreachable!()
    }
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
enum SnailItem {
    Regular(usize),
    Snail,
    None
}

#[derive(Debug,Clone,Copy)]
struct SnailNumber {
    matrix: [SnailItem; 64]
}

impl std::ops::Add for SnailNumber {
    type Output = Self;

    fn add(self, data: Self) -> Self {
        //println!("Sum of {} and {}", self, data);
        // When you add, it's like shoving the two tables into one
        let lefttab = self.matrix;
        let righttab = data.matrix;
        let mut endtab = [SnailItem::None; 64];
        endtab[2] = SnailItem::Snail;
        endtab[3] = SnailItem::Snail;
        // Insert left elements
        for (pos, &item) in lefttab.iter().enumerate().skip(2) {
            if item == SnailItem::None { continue; }
            // How deep is this element ?
            let mut n = 6;
            while pos & (1 << n) == 0 { n -= 1; }
            // Trim the leading one, replace with 10
            let npos = (2 << n) | pos ^ (1 << n);
            //println!("Changing {:b} to {:b}", pos, npos);
            endtab[npos] = item;
        }
        // Insert right elements
        for (pos, &item) in righttab.iter().enumerate().skip(2) {
            if item == SnailItem::None { continue; }
            // How deep is this element ?
            let mut n = 6;
            while pos & (1 << n) == 0 { n -= 1; }
            // Trim the leading 1, replace with 11
            let npos = (3 << n) | pos ^ (1 << n);
            //println!("Chaging {:b} to {:b}", pos, npos);
            endtab[npos] = item;
        }
        let mut res = SnailNumber { matrix: endtab };
        //println!("after addition: {}", res);
        res.reduce();
        res
    }
}

impl std::fmt::Display for SnailNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.make_string(2), self.make_string(3))
    }
}

impl SnailNumber {
    // The coordinates trick I'm using is quite simple
    // Taking the left node appends a 0 to the bit pattern,
    // Taking the right node appends a 1 to the bit pattern.
    // Since multiple bit patterns could end up on `0`,
    // You just add a `1` to prepend all bit patterns.
    // That way, "left node at depth 1" is "10", then left node
    // of the left node at depth 2 is "100", and so on. That way,
    // only lower layer items have "1" in their prefix (second half)
    // of the storage matrix), and all others have "0" (first half of the
    // matrix).
    // Zero is, thus, never used.
    fn set(&mut self, p: usize, s: SnailItem) {
        self.matrix[p] = s;
    }
    fn get(&self, p: usize) -> SnailItem {
        *self.matrix.get(p).unwrap()
    }

    fn reduce(&mut self) {
        let mut clean = false;
        while !clean {
            clean = true; // Assumed clean
            // Do all of the explosions
            // In order to do so, iterate over all elements on layer 4,
            // and find those that are SnailItem::Snail
            for i in 16..32 {
                if self.get(i) == SnailItem::Snail {
                    clean = false;
                    // Get the two numbers
                    let nleft = self.get_val(i*2);
                    // Look for a regular on the left, if any
                    if let Some(left_ipos) = self.get_num_left(i) {
                        let a = self.get_val(left_ipos);
                        self.set(left_ipos, SnailItem::Regular(a+nleft));
                    }
                    self.set(i*2, SnailItem::None);
                    let nright = self.get_val(i*2+1);
                    if let Some(right_ipos) = self.get_num_right(i) {
                        let a = self.get_val(right_ipos);
                        self.set(right_ipos, SnailItem::Regular(a+nright));
                    }
                    self.set(i*2+1, SnailItem::None);
                    self.set(i, SnailItem::Regular(0));
                    //println!("after explode:\t{}", self);
                }
                if !clean { break; }
            }
            if !clean { continue; }
            // Find all numbers that need reduction
            let leftmost = self.matrix
                .iter()
                .enumerate()
                .filter_map(|(p, i)| if let SnailItem::Regular(v) = i {
                    if *v > 9 { Some(p) } else { None }
                } else { None })
                .fold(None, |o, n| {
                    match o {
                        None => Some(n),
                        Some(v) => if is_left(n, v) { Some(n) } else { Some(v) }
                    }
                });
            if let Some(pos) = leftmost {
                clean = false;
                let val = self.get_val(pos);
                // Insert
                self.set(pos, SnailItem::Snail);
                self.set(pos*2, SnailItem::Regular(val/2));
                self.set(pos*2+1, SnailItem::Regular(
                    if val % 2 == 0 { val / 2 } else { val / 2 + 1 }
                ));
                //println!("after split  :\t{}", self);
            }
        }
    }

    fn get_num_left(&self, p: usize) -> Option<usize> {
        if p == 0b10000 { return None; } // Nope
        // Operational order : look down on the left, then ask up
        let mut c = p;
        // Go up until there is a possible left (i.e c is odd)
        while c % 2 == 0 { c >>= 1; }
        // Move left
        c -= 1;
        // Now delve until you find a number
        while c < 64 {
            if let SnailItem::Regular(_) = self.get(c) {
                return Some(c);
            }
            c = c * 2 + 1;
        }
        None
    }

    fn get_num_right(&self, p: usize) -> Option<usize> {
        if p == 0b11111 { return None; } // Nope
        let mut c = p;
        while c % 2 == 1 { c >>= 1; }
        c += 1;
        // Now delve and find a number
        while c < 64 {
            if let SnailItem::Regular(_) = self.get(c) {
                return Some(c);
            }
            c *= 2;
        }
        None
    }

    fn get_val(&self, p: usize) -> usize {
        if let SnailItem::Regular(v) = self.matrix[p] {
            v
        } else { panic!("NO VALUE AT {:b}", p) }
    }

    fn make_string(&self, p: usize) -> String {
        match self.get(p) {
            SnailItem::Regular(v) => format!("{}", v),
            SnailItem::Snail =>
                format!("[{},{}]", self.make_string(2*p), self.make_string(2*p+1)),
            SnailItem::None => String::new()
        }
    }

    fn magnitude(&self) -> usize {
        self.matrix.iter()
            .enumerate()
            .map(|(pos, val)| match val {
                SnailItem::Regular(v) => {
                    // One is right, zero is left
                    let lead_zeros = pos.leading_zeros();
                    let count_zeros = pos.count_zeros();
                    let c_ones = pos.count_ones();
                    // We have to deduce all leading zeros
                    let ac_zeros = count_zeros - lead_zeros;
                    //println!("{} located at {}=1 and {}=0", v, c_ones-1, ac_zeros);
                    2_usize.pow(c_ones-1) * 3_usize.pow(ac_zeros)
                        * v
                },
                _ => 0
            })
            .sum::<usize>()
    }
}

impl Default for SnailNumber {
    fn default() -> SnailNumber {
        SnailNumber {
            matrix: [SnailItem::None; 64]
        }
    }
}

/// Solve Advent of Code day 18 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, the result for part
/// two of advent of code day 18.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_two(data: &str) -> usize {
    // Parse the list
    let numbers: Vec<SnailNumber> = data.trim().split('\n')
        .map(unparse_snail)
        .collect::<Vec<SnailNumber>>();
    //let resies: HashMap<(usize, usize), usize> = HashMap::new();
    let mut resmax = 0;
    for i in 0..(numbers.len()) {
        for j in 0..i {
            let res = (numbers[i] + numbers[j]).magnitude();
            resmax = if resmax > res { resmax } else { res }; 
            let res = (numbers[j] + numbers[i]).magnitude();
            resmax = if resmax > res { resmax } else { res }; 
        }
    }
    resmax
}

