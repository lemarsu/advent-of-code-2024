use std::{env, error::Error, fs};

use crate::{
    block_02::Block::{self, *},
    Direction::{self, *},
    Grid, Point,
};

fn find_next_empty(grid: &Grid<Block>, point: &Point, dir: Direction) -> Option<Point> {
    let mut p = point.clone();
    loop {
        p.move_next(dir);
        match grid.get_at(&p) {
            // Looking for next point...
            BoxStart | BoxEnd => {},
            // The box chain to a wall, stopping
            Wall => return None,
            // Found empty, move box `next` here.
            Empty => return Some(p),
            Robot => unreachable!("A robot here ? Why !?!"),
        }
    }
}

fn can_move_vertically_on(grid: &Grid<Block>, point: &Point, dir: Direction) -> bool {
    match grid.get_at(&point) {
        // Looking for next point...
        BoxStart | BoxEnd => {
            let next = point.next(dir);
            let other = next.next(if grid.get_at(point) == BoxStart { Right } else { Left });
            can_move_vertically_on(grid, &next, dir) && can_move_vertically_on(grid, &other, dir)
        },
        // The box chain to a wall, stopping
        Wall => false,
        // Found empty, move box `next` here.
        Empty => true,
        Robot => unreachable!("A robot here ? Why !?!"),
    }
}

fn move_boxes_vertically(grid: &mut Grid<Block>, point: &Point, dir: Direction) -> bool {
    let block = grid.get_at(&point);
    match block {
        Empty => true,
        Wall => false,
        BoxStart | BoxEnd => match dir {
            Left | Right => unreachable!("dir is not vertical ?!?"),
            Top | Bottom => {
                let next = point.next(dir);
                let other = point.next(if block == BoxStart { Right } else { Left });
                let other_block = if block == BoxStart { BoxEnd } else { BoxStart };
                let other_next = other.next(dir);
                if can_move_vertically_on(&grid, &next, dir)
                    && can_move_vertically_on(&grid, &other_next, dir)
                {
                    move_boxes_vertically(grid, &next, dir);
                    move_boxes_vertically(grid, &other_next, dir);
                    *grid.set_at(&next) = block;
                    *grid.set_at(&other_next) = other_block;
                    *grid.set_at(&point) = Empty;
                    *grid.set_at(&other) = Empty;
                    return true;
                }
                false
            },
        },
        Robot => unreachable!("A robot here ? Why !?!"),
    }
}

fn move_once(point: &mut Point, grid: &mut Grid<Block>, dir: Direction) -> bool {
    let next = point.next(dir);
    let block = grid.get_at(&next);
    match block {
        Empty => {
            point.move_next(dir);
            true
        },
        Wall => false,
        BoxStart | BoxEnd => match dir {
            Left | Right => {
                if let Some(n) = find_next_empty(&grid, &next, dir) {
                    let (x1, x2) = if dir == Left { (n.x, next.x) } else { (next.x + 1, n.x + 1) };

                    let mut box_start = true;
                    for x in x1..x2 {
                        let p = Point::new(x, n.y);
                        *grid.set_at(&p) = if box_start { BoxStart } else { BoxEnd };
                        box_start = !box_start
                    }
                    *grid.set_at(&next) = Empty;
                    point.move_next(dir);
                    return true;
                }
                false
            },
            Top | Bottom => {
                if move_boxes_vertically(grid, &next, dir) {
                    point.move_next(dir);
                    return true;
                }
                false
            },
        },
        Robot => unreachable!("A robot here ? Why !?!"),
    }
}

fn run_movements(
    robot: &mut Point,
    grid: &mut Grid<Block>,
    movements: Vec<Direction>,
    debug: bool,
) {
    if debug {
        debug_grid(&grid);
    }
    *grid.set_at(&robot) = Block::Empty;
    for dir in movements.into_iter() {
        move_once(robot, grid, dir);
        if debug {
            *grid.set_at(&robot) = Block::Robot;
            debug_grid(&grid);
            *grid.set_at(&robot) = Block::Empty;
        }
    }
}

fn debug_grid(grid: &Grid<Block>) {
    for y in 0..grid.rows() {
        for x in 0..grid.cols() {
            let c = match grid.get_at(&(x as i32, y as i32).into()) {
                Wall => '#',
                Empty => '.',
                BoxStart => '[',
                BoxEnd => ']',
                Robot => '@',
            };
            print!("{}", c);
        }
        println!("");
    }
    println!("");
    println!("");
}

pub fn main(file: &str) -> Result<i32, Box<dyn Error>> {
    let content = fs::read_to_string(file)?;

    let parts: Vec<_> = content.split("\n\n").collect();
    let grid_str: String = parts[0]
        .chars()
        .flat_map(|c| match c {
            '#' => vec!['#', '#'],
            '.' => vec!['.', '.'],
            'O' => vec!['[', ']'],
            '@' => vec!['@', '.'],
            _ => vec![c],
        })
        .collect();
    let movements_str = parts[1];

    let mut grid: Grid<Block> = Grid::new(&grid_str);
    let movements: Vec<Direction> = movements_str
        .split("")
        .filter(|s| *s != "" && *s != "\n")
        .map(|s| s.parse().unwrap())
        .collect();

    let mut robot = grid.each_item_pos().find(|(b, _)| *b == Block::Robot).unwrap().1.clone();

    let debug = env::var("DEBUG").map_or(false, |var| var == "1");
    run_movements(&mut robot, &mut grid, movements, debug);

    let sum = grid
        .each_item_pos()
        .filter(|(b, _)| matches!(b, BoxStart))
        .map(|(_, p)| p)
        .fold(0, |acc, p| acc + p.x + p.y * 100);

    Ok(sum)
}
