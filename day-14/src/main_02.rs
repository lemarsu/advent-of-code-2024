use std::{error::Error, fs};

use crate::{Direction, Grid, Point, Robot, Size};

/// Entropy is calculated the percentage of close_robots over
/// the total of robots
pub fn calculate_entropy(grid: &Grid) -> usize {
    let mut total_robot_count = 0;
    let close_robots = grid.each_char_pos().fold(0, |acc, (robots, pos)| {
        if robots.len() == 0 {
            return acc;
        }
        total_robot_count += 1;
        acc + Direction::all()
            .into_iter()
            .map(|dir| pos.next(dir))
            .filter(|pos| grid.is_valid_point(&pos))
            .fold(0, |acc, pos| acc + if grid.get_at(&pos).is_empty() { 0 } else { 1 })
    });
    close_robots * 100 / total_robot_count
}

fn progress(str: &str) {
    use std::io::{stdout, Write};
    print!("\r{}", str);
    stdout().flush().unwrap();
}

pub fn find_grid_with_entropy_greater_than(grid_size: &Size, robots: &Vec<Robot>, max_entropy: usize) -> Option<(Grid, usize)> {
    let empty_grid = Grid::empty(grid_size.width, grid_size.height);
    let mut robots = robots.clone();
    let mut step = 0;
    loop {
        for robot in robots.iter_mut() {
            robot.move_next(&grid_size);
        }
        let mut grid = empty_grid.clone();
        for robot in robots.iter() {
            grid.set_at(&robot.pos).push(robot.clone());
        }

        let entropy = calculate_entropy(&grid);

        progress(&format!("entropy({}): {}", step + 1, entropy));

        step += 1;
        if entropy > max_entropy {
            println!("");
            return Some((grid, step));
        }
    }
}

pub fn main(width: usize, height: usize, file: &str) -> Result<usize, Box<dyn Error>> {
    let content = fs::read_to_string(file)?;

    let grid_size = Size::new(width, height);
    let robots = Robot::parse(&content);

    let result_grid = find_grid_with_entropy_greater_than(&grid_size, &robots, 60);

    if let Some((grid, step)) = result_grid {
        for y in 0..grid.rows() {
            for x in 0..grid.cols() {
                let bots_count = grid.get_at(&Point::new(x as i32, y as i32)).len();
                if bots_count > 0 {
                    print!("{}", bots_count)
                } else {
                    print!(".")
                }
            }
            println!("");
        }
        return Ok(step);
    }

    Ok(0)
}
