mod part_1 {
    use super::common::{Crates, Move};

    fn solution(input: &str) -> String {
        let mut parts = input.split("\n\n"); // there's a blank line between the crates section and the moves section, so divide there

        let mut crates: Crates = parts.next().unwrap().parse().unwrap(); // parse the initial Crates state from the first section
        let moves = parts
            .next()
            .unwrap()
            .lines() // for each "move" line
            .map(|line| line.parse::<Move>().unwrap()); // parse into a Move

        for m in moves {
            // apply each Move
            m.apply_9000(&mut crates);
        }

        crates.tops() // gather top crates from each stack
    }

    const TEST_ANSWER: &str = "CMZ";
    const FULL_ANSWER: &str = "SBPQRSCDF";

    #[test]
    fn with_test_input() {
        use super::TEST_INPUT;
        assert_eq!(solution(TEST_INPUT), TEST_ANSWER);
    }

    #[test]
    fn with_full_input() {
        use super::FULL_INPUT;
        assert_eq!(solution(FULL_INPUT), FULL_ANSWER);
    }
}

mod part_2 {
    use super::common::{Crates, Move};

    fn solution(input: &str) -> String {
        let mut parts = input.split("\n\n"); // there's a blank line between the crates section and the moves section, so divide there

        let mut crates: Crates = parts.next().unwrap().parse().unwrap(); // parse the initial Crates state from the first section
        let moves = parts
            .next()
            .unwrap()
            .lines() // for each "move" line
            .map(|line| line.parse::<Move>().unwrap()); // parse into a Move

        for m in moves {
            // apply each Move
            m.apply_9001(&mut crates);
        }

        crates.tops() // gather top crates from each stack
    }

    const TEST_ANSWER: &str = "MCD";
    const FULL_ANSWER: &str = "RGLVRCQSB";

    #[test]
    fn with_test_input() {
        use super::TEST_INPUT;
        assert_eq!(solution(TEST_INPUT), TEST_ANSWER);
    }

    #[test]
    fn with_full_input() {
        use super::FULL_INPUT;
        assert_eq!(solution(FULL_INPUT), FULL_ANSWER);
    }
}

/// Shared functionality between both parts
mod common {
    use std::str::FromStr;

    /// Represents a current arrangement of stacked crates
    #[derive(Debug, Clone)]
    pub struct Crates {
        stacks: Vec<Vec<char>>, // each inner Vec is one stack
    }

    impl Crates {
        pub fn tops(self) -> String {
            self.stacks
                .into_iter() // for each stack
                .map(|stack| stack.into_iter().last().unwrap()) // get the last char (crate)
                .collect() // collect them into a single String
        }
    }

    impl FromStr for Crates {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut res = Crates { stacks: Vec::new() }; // create a new, empty Crates state

            for line in s
                .lines() // for each line
                // (skipping the bottom line that just holds the stack numbers, because we don't need them!)
                .filter(|line| line.contains('['))
            {
                // for the first line we get...
                if res.stacks.len() == 0 {
                    let total_stacks = (line.len() + 1) / 4; // get the total number of stacks

                    // (`[X] ` is four characters long, but there's no space after the final one on each line, so we add 1 before dividing by 4 characters)

                    res.stacks = std::iter::repeat_with(|| Vec::new()) // get empty Vec for each stack
                        .take(total_stacks) // get enough for the total number of stacks we need
                        .collect(); // collect them all into a Vec of stacks and re-initialize the Crates with the right number of (empty) stacks
                }

                for stack in 0..res.stacks.len() {
                    let stack_offset = stack * 4;
                    let crate_char = line
                        .chars() // for each char in the line
                        .nth(stack_offset + 1) // add 1 to skip the [ if there is one
                        .unwrap();

                    if crate_char != ' ' {
                        // if there's a crate here (if not we just have whitespace)
                        res.stacks[stack].push(crate_char); // push the crate char onto the stack
                    }
                }
            }

            // crate stacks are formatted such that we read in the top crates
            // *first*, so we need to reverse them all after reading in all the
            // input, so that the "top" crates are at the end of the Vec
            for stack in res.stacks.iter_mut() {
                stack.reverse();
            }

            Ok(res)
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub struct Move {
        count: usize,
        from_index: usize,
        to_index: usize,
    }

    impl Move {
        /// Apply this Move to a set of Crates, CrateMover 9000-style
        pub fn apply_9000(self, crates: &mut Crates) {
            for _ in 0..self.count {
                let moved = crates.stacks[self.from_index].pop().unwrap(); // remove a crate from the "from" stack
                crates.stacks[self.to_index].push(moved); // put it on the "to" stack
            }
        }

        /// Apply this Move to a set of Crates, CrateMover 9001-style
        pub fn apply_9001(self, crates: &mut Crates) {
            let from_stack_len = crates.stacks[self.from_index].len(); // length of the "from" stack
            let to_skip = from_stack_len - self.count; // number of crates in the stack *beneath* the ones we're moving

            for moved_index in to_skip..from_stack_len {
                // for (the index of) each crate to be moved...
                let moved = crates.stacks[self.from_index][moved_index]; // get the crate
                crates.stacks[self.to_index].push(moved); // add it to the "to" stack
            }

            for _ in 0..self.count {
                // go back and remove the moved crates from the "from" stack
                crates.stacks[self.from_index].pop();
            }
        }
    }

    impl FromStr for Move {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut parts = s.split(' ');

            parts.next().unwrap(); // skip "move"
            let count = parts.next().unwrap().parse().unwrap();
            parts.next().unwrap(); // skip "from"
            let from_index = parts.next().unwrap().parse::<usize>().unwrap() - 1; // subtract one because the input has a stack "number" (starting at 1) but we need an "index" (starting at 0)
            parts.next().unwrap(); // skip "to"
            let to_index = parts.next().unwrap().parse::<usize>().unwrap() - 1; // subtract one because the input has a stack "number" (starting at 1) but we need an "index" (starting at 0)

            Ok(Self {
                count,
                from_index,
                to_index,
            })
        }
    }
}

const TEST_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

const FULL_INPUT: &str = "[V]     [B]                     [F]
[N] [Q] [W]                 [R] [B]
[F] [D] [S]     [B]         [L] [P]
[S] [J] [C]     [F] [C]     [D] [G]
[M] [M] [H] [L] [P] [N]     [P] [V]
[P] [L] [D] [C] [T] [Q] [R] [S] [J]
[H] [R] [Q] [S] [V] [R] [V] [Z] [S]
[J] [S] [N] [R] [M] [T] [G] [C] [D]
 1   2   3   4   5   6   7   8   9 

move 1 from 8 to 4
move 1 from 7 to 8
move 1 from 6 to 3
move 2 from 6 to 5
move 8 from 5 to 1
move 5 from 3 to 8
move 1 from 7 to 8
move 8 from 1 to 2
move 3 from 3 to 9
move 13 from 8 to 7
move 2 from 1 to 2
move 1 from 6 to 2
move 2 from 1 to 7
move 4 from 4 to 2
move 10 from 9 to 4
move 7 from 4 to 1
move 1 from 6 to 7
move 2 from 4 to 5
move 1 from 5 to 2
move 1 from 5 to 8
move 3 from 1 to 5
move 2 from 4 to 6
move 2 from 6 to 4
move 2 from 4 to 5
move 5 from 1 to 5
move 1 from 9 to 5
move 1 from 8 to 5
move 14 from 2 to 6
move 12 from 7 to 4
move 4 from 6 to 7
move 3 from 6 to 4
move 4 from 4 to 9
move 2 from 4 to 6
move 2 from 9 to 3
move 9 from 4 to 3
move 2 from 1 to 6
move 5 from 7 to 3
move 4 from 7 to 8
move 2 from 6 to 7
move 3 from 6 to 7
move 10 from 5 to 8
move 8 from 8 to 9
move 10 from 9 to 7
move 12 from 7 to 5
move 1 from 1 to 5
move 3 from 7 to 2
move 10 from 3 to 7
move 6 from 5 to 7
move 2 from 6 to 1
move 12 from 2 to 7
move 3 from 3 to 1
move 1 from 6 to 5
move 10 from 5 to 7
move 3 from 3 to 4
move 34 from 7 to 1
move 2 from 6 to 9
move 1 from 6 to 3
move 3 from 1 to 3
move 1 from 7 to 3
move 3 from 3 to 6
move 1 from 4 to 3
move 22 from 1 to 6
move 2 from 9 to 7
move 2 from 4 to 9
move 12 from 6 to 8
move 1 from 7 to 6
move 4 from 8 to 2
move 1 from 7 to 1
move 6 from 8 to 9
move 1 from 2 to 5
move 1 from 2 to 7
move 13 from 1 to 2
move 2 from 3 to 1
move 4 from 7 to 5
move 3 from 9 to 4
move 1 from 1 to 8
move 4 from 5 to 2
move 12 from 6 to 2
move 3 from 1 to 3
move 1 from 4 to 1
move 9 from 8 to 5
move 6 from 5 to 7
move 2 from 4 to 5
move 5 from 9 to 6
move 5 from 3 to 7
move 30 from 2 to 6
move 11 from 7 to 9
move 36 from 6 to 3
move 10 from 9 to 3
move 1 from 6 to 5
move 3 from 5 to 2
move 2 from 5 to 2
move 28 from 3 to 4
move 6 from 4 to 1
move 1 from 2 to 3
move 2 from 5 to 2
move 6 from 1 to 7
move 1 from 1 to 6
move 6 from 3 to 5
move 6 from 7 to 2
move 1 from 6 to 4
move 2 from 2 to 6
move 8 from 2 to 1
move 3 from 2 to 4
move 2 from 3 to 4
move 4 from 3 to 4
move 1 from 6 to 1
move 2 from 1 to 8
move 1 from 6 to 4
move 1 from 9 to 3
move 2 from 5 to 4
move 1 from 8 to 7
move 1 from 7 to 9
move 1 from 3 to 5
move 1 from 8 to 6
move 34 from 4 to 9
move 13 from 9 to 8
move 1 from 8 to 2
move 1 from 2 to 1
move 4 from 5 to 1
move 9 from 8 to 7
move 11 from 1 to 3
move 1 from 4 to 2
move 1 from 6 to 7
move 1 from 9 to 4
move 1 from 4 to 1
move 1 from 5 to 3
move 5 from 7 to 8
move 1 from 2 to 5
move 1 from 5 to 1
move 21 from 9 to 5
move 19 from 3 to 4
move 17 from 4 to 6
move 2 from 8 to 4
move 2 from 6 to 8
move 2 from 6 to 9
move 2 from 7 to 6
move 1 from 4 to 9
move 6 from 5 to 6
move 1 from 9 to 8
move 8 from 5 to 7
move 15 from 6 to 2
move 1 from 9 to 7
move 2 from 1 to 6
move 3 from 4 to 7
move 1 from 1 to 6
move 3 from 5 to 4
move 2 from 5 to 6
move 2 from 4 to 1
move 13 from 7 to 8
move 2 from 6 to 4
move 3 from 2 to 4
move 2 from 7 to 6
move 5 from 4 to 6
move 4 from 2 to 6
move 1 from 1 to 9
move 18 from 8 to 3
move 1 from 4 to 5
move 1 from 2 to 7
move 15 from 3 to 1
move 1 from 5 to 1
move 3 from 3 to 4
move 1 from 5 to 4
move 1 from 5 to 6
move 1 from 6 to 8
move 2 from 8 to 2
move 3 from 1 to 8
move 6 from 2 to 8
move 1 from 7 to 6
move 12 from 8 to 5
move 2 from 9 to 6
move 6 from 1 to 5
move 9 from 5 to 3
move 1 from 2 to 8
move 20 from 6 to 9
move 3 from 6 to 7
move 1 from 7 to 1
move 7 from 3 to 4
move 2 from 7 to 2
move 1 from 8 to 7
move 8 from 4 to 1
move 11 from 1 to 7
move 10 from 7 to 6
move 2 from 4 to 9
move 21 from 9 to 3
move 6 from 5 to 9
move 6 from 3 to 2
move 1 from 4 to 5
move 1 from 7 to 9
move 8 from 3 to 2
move 9 from 2 to 1
move 14 from 1 to 6
move 1 from 1 to 7
move 4 from 3 to 8
move 3 from 8 to 7
move 5 from 7 to 4
move 3 from 6 to 9
move 2 from 3 to 7
move 3 from 5 to 6
move 1 from 5 to 6
move 2 from 7 to 9
move 1 from 8 to 3
move 22 from 6 to 5
move 3 from 9 to 4
move 3 from 6 to 1
move 5 from 4 to 6
move 9 from 2 to 8
move 4 from 6 to 1
move 1 from 3 to 2
move 1 from 2 to 3
move 6 from 8 to 1
move 2 from 4 to 3
move 10 from 1 to 7
move 2 from 8 to 7
move 1 from 9 to 6
move 4 from 3 to 5
move 1 from 8 to 3
move 4 from 9 to 8
move 1 from 4 to 3
move 1 from 3 to 8
move 3 from 7 to 6
move 1 from 1 to 5
move 10 from 5 to 9
move 5 from 6 to 4
move 5 from 8 to 5
move 4 from 9 to 8
move 3 from 3 to 9
move 2 from 8 to 6
move 5 from 7 to 5
move 1 from 4 to 1
move 1 from 1 to 2
move 2 from 8 to 6
move 1 from 2 to 1
move 1 from 7 to 2
move 1 from 1 to 5
move 28 from 5 to 9
move 3 from 6 to 1
move 1 from 6 to 9
move 1 from 2 to 9
move 2 from 1 to 2
move 2 from 7 to 5
move 1 from 7 to 5
move 1 from 2 to 5
move 3 from 1 to 9
move 1 from 5 to 8
move 15 from 9 to 2
move 11 from 9 to 4
move 11 from 4 to 7
move 2 from 4 to 1
move 7 from 7 to 8
move 1 from 1 to 4
move 20 from 9 to 1
move 2 from 7 to 8
move 1 from 4 to 6
move 1 from 6 to 2
move 2 from 7 to 5
move 1 from 9 to 6
move 1 from 4 to 9
move 4 from 5 to 2
move 1 from 6 to 8
move 1 from 4 to 9
move 11 from 8 to 3
move 1 from 1 to 9
move 1 from 5 to 9
move 1 from 2 to 6
move 4 from 9 to 8
move 4 from 8 to 7
move 10 from 1 to 6
move 7 from 1 to 5
move 8 from 3 to 4
move 2 from 3 to 5
move 3 from 7 to 4
move 1 from 4 to 5
move 2 from 1 to 6
move 9 from 2 to 6
move 1 from 7 to 9
move 1 from 3 to 2
move 7 from 4 to 3
move 3 from 3 to 7
move 5 from 2 to 3
move 1 from 1 to 9
move 2 from 2 to 7
move 1 from 4 to 6
move 3 from 5 to 6
move 4 from 7 to 6
move 1 from 7 to 4
move 1 from 4 to 7
move 1 from 2 to 8
move 1 from 7 to 1
move 27 from 6 to 2
move 1 from 4 to 1
move 7 from 5 to 7
move 1 from 4 to 1
move 1 from 8 to 3
move 3 from 7 to 3
move 2 from 1 to 6
move 2 from 9 to 1
move 18 from 2 to 1
move 2 from 7 to 5
move 12 from 3 to 4
move 1 from 5 to 6
move 3 from 6 to 1
move 24 from 1 to 8
move 9 from 2 to 4
move 3 from 2 to 1
move 2 from 6 to 3
move 1 from 6 to 9
move 1 from 5 to 6
move 1 from 6 to 2
move 1 from 1 to 7
move 1 from 2 to 1
move 1 from 1 to 2
move 3 from 7 to 2
move 2 from 1 to 4
move 8 from 4 to 5
move 22 from 8 to 1
move 1 from 8 to 1
move 13 from 4 to 1
move 1 from 8 to 5
move 3 from 3 to 1
move 1 from 2 to 7
move 38 from 1 to 6
move 27 from 6 to 1
move 2 from 2 to 9
move 3 from 9 to 8
move 2 from 8 to 6
move 1 from 8 to 3
move 1 from 2 to 1
move 1 from 3 to 6
move 1 from 2 to 3
move 1 from 7 to 6
move 7 from 6 to 3
move 20 from 1 to 4
move 6 from 1 to 6
move 17 from 4 to 7
move 3 from 6 to 5
move 14 from 7 to 9
move 8 from 5 to 7
move 3 from 1 to 6
move 3 from 3 to 1
move 2 from 4 to 1
move 4 from 5 to 1
move 9 from 6 to 2
move 3 from 6 to 4
move 4 from 7 to 8
move 4 from 1 to 6
move 2 from 3 to 1
move 6 from 6 to 7
move 4 from 8 to 7
move 4 from 2 to 1
move 4 from 2 to 3
move 4 from 9 to 5
move 8 from 9 to 5
move 1 from 9 to 5
move 1 from 2 to 1
move 16 from 7 to 2
move 10 from 2 to 9
move 11 from 9 to 8
move 4 from 3 to 5
move 3 from 1 to 4
move 13 from 5 to 7
move 10 from 8 to 5
move 2 from 1 to 5
move 11 from 7 to 4
move 2 from 3 to 6
move 3 from 7 to 6
move 1 from 3 to 2
move 1 from 1 to 8
move 2 from 8 to 4
move 3 from 1 to 2
move 4 from 6 to 1
move 7 from 1 to 9
move 1 from 6 to 7
move 2 from 5 to 8
move 1 from 2 to 9
move 1 from 7 to 8
move 5 from 5 to 8
move 1 from 2 to 3
move 4 from 2 to 5
move 17 from 4 to 1
move 10 from 5 to 9
move 2 from 4 to 2
move 2 from 4 to 1
move 1 from 4 to 9
move 1 from 3 to 7
move 1 from 7 to 8
move 12 from 9 to 2
move 1 from 2 to 4
move 1 from 4 to 1
move 1 from 1 to 9
move 1 from 8 to 1
move 8 from 8 to 3
move 2 from 5 to 1
move 3 from 1 to 9
move 1 from 2 to 6
move 4 from 3 to 7
move 1 from 7 to 6
move 10 from 9 to 2
move 1 from 5 to 9
move 1 from 9 to 3
move 17 from 1 to 6
move 2 from 1 to 2
move 11 from 6 to 7
move 2 from 2 to 9
move 2 from 9 to 5
move 12 from 7 to 9
move 20 from 2 to 7
move 5 from 9 to 5
move 21 from 7 to 1
move 2 from 6 to 4
move 11 from 1 to 4
move 5 from 4 to 6
move 1 from 7 to 8
move 5 from 9 to 3
move 5 from 2 to 8
move 3 from 9 to 3
move 2 from 8 to 7
move 2 from 1 to 7
move 10 from 6 to 3
move 1 from 2 to 6
move 2 from 8 to 5
move 1 from 6 to 5
move 2 from 4 to 9
move 1 from 4 to 5
move 8 from 1 to 6
move 4 from 4 to 8
move 6 from 8 to 4
move 21 from 3 to 9
move 5 from 9 to 2
move 4 from 7 to 9
move 22 from 9 to 3
move 9 from 6 to 4
move 2 from 2 to 6
move 2 from 2 to 1
move 2 from 5 to 7
move 7 from 5 to 4
move 22 from 4 to 2
move 2 from 5 to 4
move 16 from 2 to 5
move 2 from 6 to 2
move 13 from 3 to 4
move 5 from 5 to 7
move 15 from 4 to 7
move 3 from 2 to 3
move 3 from 2 to 5
move 1 from 1 to 2
move 1 from 2 to 4
move 6 from 5 to 9
move 4 from 3 to 6
move 2 from 5 to 9
move 1 from 2 to 7
move 1 from 1 to 9
move 2 from 4 to 5
move 19 from 7 to 8
move 1 from 6 to 5
move 1 from 5 to 1
move 1 from 9 to 4
move 5 from 8 to 1
move 3 from 8 to 1
move 7 from 5 to 6
move 3 from 7 to 1
move 1 from 2 to 5
move 4 from 9 to 8
move 2 from 5 to 6
move 10 from 1 to 4
move 1 from 7 to 2
move 6 from 3 to 4
move 9 from 4 to 3
move 2 from 2 to 8
move 2 from 9 to 5
move 5 from 8 to 3
move 1 from 1 to 5
move 2 from 5 to 6
move 1 from 1 to 7
move 2 from 9 to 7
move 8 from 4 to 7
move 3 from 3 to 9
move 4 from 6 to 3
move 1 from 5 to 3
move 1 from 7 to 2
move 1 from 2 to 1
move 1 from 6 to 5
move 1 from 5 to 2
move 10 from 7 to 4
move 10 from 4 to 1
move 10 from 1 to 8
move 1 from 9 to 6
move 1 from 1 to 4
move 11 from 8 to 1
move 2 from 9 to 5
move 5 from 6 to 3
move 1 from 3 to 8
move 4 from 1 to 3
move 5 from 3 to 8
move 1 from 4 to 7
move 1 from 7 to 2
move 13 from 3 to 5
move 2 from 2 to 1
move 4 from 3 to 1
move 4 from 5 to 6
move 3 from 6 to 2
move 4 from 5 to 4
move 8 from 8 to 7
move 1 from 3 to 9";
