use std::{collections::HashSet, error::Error, fs};

use crate::{Combinator, Link, Network};

pub fn main(file: &str) -> Result<usize, Box<dyn Error>> {
    let content = fs::read_to_string(file)?;

    let links: Vec<Link> =
        content.split("\n").filter(|line| *line != "").map(|line| line.parse().unwrap()).collect();

    let mut network = Network::new();

    for link in links.into_iter() {
        network.add(link);
    }

    let link3: HashSet<_> = network
        .computers()
        .into_iter()
        .filter(|comp| comp.starts_with("t"))
        .flat_map(|comp| {
            let others = network.linked_to(&comp);
            let mut sub_links: Vec<_> = Combinator::distinct(others, 2)
                .filter(|computers| {
                    let a = &computers[0];
                    let b = &computers[1];
                    network.is_link_to(a, b)
                })
                .collect();
            for link in sub_links.iter_mut() {
                link.push(comp.clone());
                link.sort();
            }
            sub_links
        })
        .collect();

    let mut links3: Vec<_> = link3.into_iter().collect();
    links3.sort();

    for link in links3.iter() {
        println!("link3: {:?}", link);
    }

    Ok(links3.len())
}
