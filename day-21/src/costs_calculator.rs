use std::collections::HashMap;

use crate::{Code, Pad};

// This one was really hard, it melted my brain so much !
// This algorithm was describe on reddit :
// https://www.reddit.com/r/adventofcode/comments/1hj8380/2024_day_21_part_2_i_need_help_three_days_in_row/
pub struct CostCalculator {
    costs: HashMap<(char, char), usize>,
    num_pad: Pad,
}

impl CostCalculator {
    pub fn new(level: usize) -> Self {
        assert!(level > 0, "level should be greater than zero");
        let dir_keys = ['<', '>', '^', 'v', 'A'];
        let mut costs = HashMap::new();

        let dpad = Pad::new_dpad();
        for from in dir_keys {
            for to in dir_keys {
                let moves = dpad.next_step(from, to);
                costs.entry((from, to)).or_insert(moves.len());
            }
        }

        let mut tmp = HashMap::new();
        for _ in 0..(level - 1) {
            for from in dir_keys {
                for to in dir_keys {
                    let mut dirs = dpad.next_step(from, to);
                    dirs.insert(0, 'A');
                    let cost =
                        dirs.windows(2).fold(0, |acc, chars| acc + costs[&(chars[0], chars[1])]);
                    tmp.entry((from, to)).or_insert(cost);
                }
            }
            (costs, tmp) = (tmp, HashMap::new());
        }

        Self { costs, num_pad: Pad::new_numpad() }
    }

    pub fn calc(&self, code: &Code) -> usize {
        let mut num_path = self.num_pad.get_path(&code);
        num_path.insert(0, 'A');

        let cost = num_path.windows(2).fold(0, |acc, keys| acc + self.costs[&(keys[0], keys[1])]);
        code.numeric_val() * cost
    }
}
