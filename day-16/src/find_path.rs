use std::collections::{BinaryHeap, HashMap};

use crate::{
    Block::{self, *},
    Direction::*,
    Grid, Path, Point, Raindeer, Step,
};

fn update_best_path(bests: &mut HashMap<Raindeer, Path>, path: &Path) {
    if let Some(best_path) = bests.get(&path.stop) {
        if path.cost() < best_path.cost() {
            bests.insert(path.stop.clone(), path.clone());
        }
    } else {
        bests.insert(path.stop.clone(), path.clone());
    }
}

pub fn find_paths(grid: &Grid<Block>, deer: &Raindeer, stop: &Point) -> Vec<Path> {
    let mut queue = BinaryHeap::new();
    let path = Path::empty(deer);
    queue.push(path);
    let mut bests: HashMap<Raindeer, Path> = HashMap::new();
    let mut ret = Vec::new();

    while let Some(path) = queue.pop() {
        if let Some(best_path) = bests.get(&path.stop) {
            if path.cost() > best_path.cost() {
                continue;
            }
        }
        if path.stop.pos == *stop {
            ret.push(path.clone());
        }

        // println!("Queue size: {}", queue.len());
        // println!("Path cost: {}", path.cost());
        let dir = path.stop.direction;
        let next = path.stop.pos.next(dir);
        let dir_left = dir.left();
        let left = path.stop.pos.next(dir_left);
        let dir_right = dir.right();
        let right = path.stop.pos.next(dir_right);

        if grid.get_at(&next) == Empty {
            if let Some(path) = path.add_step(Step::Forward) {
                update_best_path(&mut bests, &path);
                queue.push(path);
            }
        }
        if grid.get_at(&left) == Empty {
            if let Some(path) = path.add_step(Step::Turn(Left)) {
                update_best_path(&mut bests, &path);
                queue.push(path);
            }
        }
        if grid.get_at(&right) == Empty {
            if let Some(path) = path.add_step(Step::Turn(Right)) {
                update_best_path(&mut bests, &path);
                queue.push(path);
            }
        }
    }
    ret
}
