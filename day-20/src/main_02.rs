use std::{collections::{HashMap, HashSet}, error::Error, fs};

use crate::{
    find_path::find_path,
    Block::{self, *},
    Grid, Jump, Point,
};

fn progress(str: &str) {
    use std::io::{stdout, Write};
    print!("\r{}", str);
    stdout().flush().unwrap();
}

pub fn main(file: &str) -> Result<usize, Box<dyn Error>> {
    let content = fs::read_to_string(file)?;

    let mut grid: Grid<Block> = Grid::new(&content);

    let start = grid
        .each_item_pos()
        .find_map(|(b, p)| {
            if b == Start {
                return Some(p);
            }
            None
        })
        .unwrap();

    let stop = grid
        .each_item_pos()
        .find_map(|(b, p)| {
            if b == End {
                return Some(p);
            }
            None
        })
        .unwrap();

    *grid.set_at(&start) = Empty;
    *grid.set_at(&stop) = Empty;

    let path = find_path(&grid, &start, &stop).unwrap();

    let mut jumps = HashSet::new();

    let cheat_length = 20;

    for point in path.path.iter() {
        for y in (point.y - cheat_length)..=(point.y + cheat_length) {
            for x in (point.x - cheat_length)..=(point.x + cheat_length) {
                let p = Point::new(x, y);
                if p != *point
                    && p.manathan_distance(point) <= cheat_length as usize
                    && grid.is_valid_point(&p)
                    && grid.get_at(&p) == Block::Empty
                {
                    jumps.insert(Jump::new(point.clone(), p));
                }
            }
        }
    }

    println!("Got  {} jumps to test", jumps.len());

    let point_dist: HashMap<_, _> = path.path.iter().enumerate().map(|(i, p)| (p.clone(), i)).collect();
    let mut jumped_paths = HashMap::new();

    for (i, jump) in jumps.into_iter().enumerate() {
        progress(&format!("Jump {}", i));
        // let from_pos = path.path.iter().position(|p| *p == jump.from).unwrap();
        let from_pos = point_dist[&jump.from];
        // if let Some(to_pos) = path.path.iter().position(|p| *p == jump.to) {
        if let Some(to_pos) = point_dist.get(&jump.to) {
            let diff = to_pos.saturating_sub(from_pos).saturating_sub(jump.len());
            if diff > 0 {
                let entry = jumped_paths.entry(diff).or_insert(0);
                *entry += 1;
            }
        }
    }
    println!("");

    // for (diff, count) in jumped_paths.iter() {
    //     println!("Found {} paths cost difference of {}", count, diff);
    // }

    let sum = jumped_paths.into_iter().fold(
        0,
        |acc, (diff, count)| {
            if diff >= 100 {
                acc + count
            } else {
                acc
            }
        },
    );

    Ok(sum)
}
