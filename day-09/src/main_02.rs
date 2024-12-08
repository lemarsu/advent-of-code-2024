use std::{error::Error, fs};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct FileBlock {
    idx: usize,
    size: usize,
    id: usize,
}

impl FileBlock {
    pub fn new(idx: usize, size: usize, id: usize) -> Self {
        Self { idx, size, id }
    }
}

fn reorder_files(files: &mut Vec<FileBlock>) {
    let remaining_files = files.clone();

    for file in remaining_files.iter().rev() {
        let free_pos = files[0..(files.len() - 1)].iter().enumerate().find_map(|(i, current)| {
            let next = &files[i + 1];

            if next.idx - (current.idx + current.size) < file.size {
                return None;
            }

            Some(i + 1)
        });
        if let Some(pos) = free_pos {
            let prev = &files[pos - 1];
            let new_idx = prev.idx + prev.size;
            let file_pos = files.iter().position(|f| f.id == file.id).unwrap();
            if files[file_pos].idx < new_idx {
                continue;
            }
            let mut file = files.remove(file_pos);
            file.idx = new_idx;
            files.insert(pos, file);
        }
    }
}

pub fn main(file: &str) -> Result<usize, Box<dyn Error>> {
    let content = fs::read_to_string(file)?;

    let mut pos = 0;
    let mut files: Vec<_> = content
        .chars()
        .enumerate()
        .filter(|(_, char)| char >= &'0' && char <= &'9')
        .filter_map(|(i, char)| {
            let size = (char as u8 - b'0') as usize;
            let ret = if i % 2 == 1 {
                None
            } else {
                let id = i / 2;
                let block = FileBlock::new(pos, size, id);
                Some(block)
            };
            pos += size;
            ret
        })
        .collect();

    reorder_files(&mut files);

    let checksum = files.iter().fold(0, |acc, file| {
        let mut acc = acc;
        for i in 0..file.size {
            acc += file.id * (file.idx + i);
        }
        acc
    });

    Ok(checksum)
}
