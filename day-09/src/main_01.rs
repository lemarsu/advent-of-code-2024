use std::{error::Error, fs};

#[derive(Debug)]
pub struct FileBlock {
    size: usize,
    id: Option<usize>,
}

impl FileBlock {
    pub fn new(size: usize, id: Option<usize>) -> Self {
        Self { size, id }
    }
}

// fn progress(str: &str) {
//     use std::io::{stdout, Write};
//     print!("\r{}", str);
//     stdout().flush().unwrap();
// }

fn reorder_blocks(disk: &mut Vec<Option<usize>>) {
    let mut start = 0;
    let mut stop = disk.len() - 1;

    while start < stop {
        if disk[start].is_some() {
            start += 1;
            // println!("move S ({})", start);
            continue;
        } else {
            while disk[stop].is_none() {
                stop -= 1;
            }
            if start >= stop {
                return;
            }
            disk[start] = disk[stop];
            disk[stop] = None;
        }
    }
}

fn calculate_checksum(disk: &Vec<Option<usize>>) -> usize {
    disk.iter()
        .enumerate()
        .filter_map(|(i, block)| block.map(|id| (i, id)))
        .fold(0, |acc, (i, id)| {
            acc + i * id
        })
}

pub fn main(file: &str) -> Result<usize, Box<dyn Error>> {
    let content = fs::read_to_string(file)?;

    let blocks: Vec<_> = content
        .chars()
        .enumerate()
        .filter(|(_, char)| char >= &'0' && char <= &'9')
        .map(|(i, char)| {
            let size = (char as u8 - b'0') as usize;
            let id = if i % 2 == 0 { Some(i / 2) } else { None };
            let block = FileBlock::new(size, id);
            block
        })
        .collect();

    let full_size = blocks.iter().fold(0, |acc, blk| acc + blk.size);

    let mut disk: Vec<Option<usize>> = Vec::with_capacity(full_size);

    for block in blocks.iter() {
        for _ in 0..block.size {
            disk.push(block.id);
        }
    }

    reorder_blocks(&mut disk);

    let checksum = calculate_checksum(&disk);

    Ok(checksum)
}
