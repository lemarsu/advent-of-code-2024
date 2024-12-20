use std::collections::HashMap;

use crate::Point;

enum PadOrder {
    LrUd,
    UdLr,
}

#[derive(Debug, Clone)]
pub enum PadType {
    Numpad,
    Dpad,
}

impl PadType {
    fn key_pos(&self) -> HashMap<char, Point> {
        let mut ret = HashMap::new();
        match self {
            Numpad => {
                ret.insert('7', Point::new(0, 0));
                ret.insert('8', Point::new(1, 0));
                ret.insert('9', Point::new(2, 0));
                ret.insert('4', Point::new(0, 1));
                ret.insert('5', Point::new(1, 1));
                ret.insert('6', Point::new(2, 1));
                ret.insert('1', Point::new(0, 2));
                ret.insert('2', Point::new(1, 2));
                ret.insert('3', Point::new(2, 2));
                ret.insert('0', Point::new(1, 3));
                ret.insert('A', Point::new(2, 3));
            },
            Dpad => {
                ret.insert('^', Point::new(1, 0));
                ret.insert('A', Point::new(2, 0));
                ret.insert('<', Point::new(0, 1));
                ret.insert('v', Point::new(1, 1));
                ret.insert('>', Point::new(2, 1));
            },
        }
        ret
    }

    fn specific_order(&self, from: &Point, to: &Point) -> Option<PadOrder> {
        match self {
            Numpad => {
                if from.y == 3 && to.x == 0 {
                    return Some(PadOrder::UdLr);
                }
                if from.x == 0 && to.y == 3 {
                    return Some(PadOrder::LrUd);
                }
            },
            Dpad => {
                if from.x == 0 {
                    return Some(PadOrder::LrUd);
                }
                if to.x == 0 {
                    return Some(PadOrder::UdLr);
                }
            },
        }
        None
    }
}

use PadType::*;

#[derive(Debug, Clone)]
pub struct Pad {
    pad_type: PadType,
    key_pos: HashMap<char, Point>,
}

impl Pad {
    pub fn new(pad_type: PadType) -> Self {
        Self { key_pos: pad_type.key_pos(), pad_type }
    }

    pub fn new_dpad() -> Self {
        Self::new(Dpad)
    }

    pub fn new_numpad() -> Self {
        Self::new(Numpad)
    }

    fn build_lr_or_ud(diff: i32, lt: char, gt: char) -> Vec<char> {
        let mut ret = Vec::new();
        if diff != 0 {
            let c = if diff < 0 { lt } else { gt };
            for _ in 0..(diff.abs()) {
                ret.push(c);
            }
        }
        ret
    }

    fn build_lr_ud(diff: &Point) -> (Vec<char>, Vec<char>) {
        (Self::build_lr_or_ud(diff.x, '<', '>'), Self::build_lr_or_ud(diff.y, '^', 'v'))
    }

    pub fn next_step(
        &self,
        key1: char,
        key2: char,
    ) -> Vec<char> {
        let from = self.key_pos[&key1];
        let to = self.key_pos[&key2];
        let diff = to - from;
        let (mut lr, mut ud) = Self::build_lr_ud(&diff);

        let order = if let Some(order) = self.pad_type.specific_order(&from, &to) {
            order
        } else {
            if diff.x < 0 {
                PadOrder::LrUd
            } else {
                PadOrder::UdLr
            }
        };

        match order {
            PadOrder::UdLr => {
                lr.push('A');
                ud.append(&mut lr);
                return ud;
            },
            PadOrder::LrUd => {
                ud.push('A');
                lr.append(&mut ud);
                return lr;
            },
        }
    }

    fn get_step(
        &self,
        key1: char,
        key2: char,
        level: usize,
        hash: &mut HashMap<(usize, char, char), Vec<char>>,
    ) -> Vec<char> {
        if let Some(ret) = hash.get(&(level, key1, key2)) {
            return ret.to_vec();
        }
        let from = self.key_pos[&key1];
        let to = self.key_pos[&key2];
        let diff = to - from;
        let (mut lr, mut ud) = Self::build_lr_ud(&diff);

        let order = if let Some(order) = self.pad_type.specific_order(&from, &to) {
            order
        } else {
            if diff.x < 0 {
                PadOrder::LrUd
            } else {
                PadOrder::UdLr
            }
        };

        match order {
            PadOrder::UdLr => {
                lr.push('A');
                ud.append(&mut lr);
                hash.insert((level, key1, key2), ud.clone());
                return ud;
            },
            PadOrder::LrUd => {
                ud.push('A');
                lr.append(&mut ud);
                hash.insert((level, key1, key2), lr.clone());
                return lr;
            },
        }
    }

    pub fn get_path(&self, keys: &[char]) -> Vec<char> {
        let mut head = self.next_step('A', keys[0]);
        let mut tail = keys.windows(2).flat_map(|keys| self.next_step(keys[0], keys[1])).collect();
        head.append(&mut tail);
        head
    }

    pub fn internal_get_path(
        &self,
        keys: &[char],
        level: usize,
        hash: &mut HashMap<(usize, char, char), Vec<char>>,
    ) -> Vec<char> {
        let mut head = self.get_step('A', keys[0], level, hash);
        let mut tail =
            keys.windows(2).flat_map(|keys| self.get_step(keys[0], keys[1], level, hash)).collect();
        head.append(&mut tail);
        head
    }

    // pub fn search_path(
    //     keys: &[char],
    //     level: usize,
    //     hash: HashMap<(usize, Point, Point), Vec<char>>,
    // ) -> Vec<char> {
    //     if level == 0 {
    //         return keys.to_vec();
    //     } else if level == 1 {
    //         Self::new_numpad().get_path(keys)
    //     } else {
    //         Self::new_dpad().get_path(&Self::search_path(keys, level - 1))
    //     }
    // }

    // fn all_paths(&self, path: &Path) -> Vec<Path> {
    //     Direction::all()
    //         .into_iter()
    //         .map(|dir| path.next(dir))
    //         .filter(|path| {
    //             let grid = &self.grid;
    //             let point = path.point();
    //             grid.is_valid_point(point) && grid.get_at(point).is_some()
    //         })
    //         .collect()
    // }
    //
    // pub fn goto_orig(&mut self, char: char) -> Option<Vec<Direction>> {
    //     let mut visited = HashSet::new();
    //     let mut to_visit = BinaryHeap::new();
    //     to_visit.push(Path::new(&self.point));
    //
    //     while let Some(path) = to_visit.pop() {
    //         if visited.contains(path.point()) {
    //             continue;
    //         }
    //         if self.grid.get_at(&path) == Some(char) {
    //             self.point = *path;
    //             return Some(path.directions().to_vec());
    //         }
    //
    //         for next in self.all_paths(&path) {
    //             to_visit.push(next);
    //         }
    //
    //         visited.insert(path.point().clone());
    //     }
    //
    //     None
    // }
    //
    // pub fn move_by_x_and_y(&mut self, diff: &Point) -> Vec<Direction> {
    //     let mut ret = Vec::new();
    //     if diff.x > 0 {
    //         for _ in 0..diff.x {
    //             ret.push(Direction::Right);
    //         }
    //     } else if diff.x < 0 {
    //         for _ in 0..-(diff.x) {
    //             ret.push(Direction::Left);
    //         }
    //     }
    //
    //     if diff.y > 0 {
    //         for _ in 0..diff.y {
    //             ret.push(Direction::Bottom);
    //         }
    //     } else if diff.y < 0 {
    //         for _ in 0..-(diff.y) {
    //             ret.push(Direction::Top);
    //         }
    //     }
    //     ret
    // }
    //
    // pub fn move_by_y_and_x(&mut self, diff: &Point) -> Vec<Direction> {
    //     let mut ret = Vec::new();
    //     if diff.y > 0 {
    //         for _ in 0..diff.y {
    //             ret.push(Direction::Bottom);
    //         }
    //     } else if diff.y < 0 {
    //         for _ in 0..-(diff.y) {
    //             ret.push(Direction::Top);
    //         }
    //     }
    //
    //     if diff.x > 0 {
    //         for _ in 0..diff.x {
    //             ret.push(Direction::Right);
    //         }
    //     } else if diff.x < 0 {
    //         for _ in 0..-(diff.x) {
    //             ret.push(Direction::Left);
    //         }
    //     }
    //     ret
    // }
    //
    // fn is_valid_path(&self, directions: &[Direction]) -> bool {
    //     let mut p = self.point.clone();
    //     for direction in directions {
    //         p.move_next(*direction);
    //         if !self.grid.is_valid_point(&p) || self.grid.get_at(&p).is_none() {
    //             return false;
    //         }
    //     }
    //     true
    // }
    //
    // pub fn goto(&mut self, char: char) -> Option<Vec<Direction>> {
    //     let dest = self.grid.each_item_pos().find_map(|(key, pos)| {
    //         if let Some(key) = key {
    //             if key == char {
    //                 return Some(pos);
    //             }
    //         }
    //         None
    //     })?;
    //
    //     let diff = dest - self.point;
    //     let mut directions = self.move_by_x_and_y(&diff);
    //     if !self.is_valid_path(&directions) {
    //         directions = self.move_by_y_and_x(&diff);
    //     }
    //     if !self.is_valid_path(&directions) {
    //         panic!("No valid path !");
    //     }
    //
    //     self.point = dest;
    //
    //     Some(directions)
    // }
    //
    // pub fn reverse_type_code(&mut self, code: &Code) -> Option<String> {
    //     let str = code
    //         .clone()
    //         .keys()
    //         .into_iter()
    //         .map(|key| {
    //             let directions = self.goto(*key).unwrap();
    //             let mut str = directions.iter().map(|dir| dir.to_string()).collect::<String>();
    //             str += "A";
    //             str
    //         })
    //         .collect();
    //     Some(str)
    // }
    //
    // pub fn type_code(&mut self, directions: &str) -> String {
    //     let mut ret = String::new();
    //     for char in directions.chars() {
    //         match char {
    //             '<' => self.point.move_next(Left),
    //             '>' => self.point.move_next(Right),
    //             '^' => self.point.move_next(Top),
    //             'v' => self.point.move_next(Bottom),
    //             'A' => ret.push(self.grid.get_at(&self.point).unwrap()),
    //             _ => unreachable!(),
    //         }
    //     }
    //     ret
    // }
}
