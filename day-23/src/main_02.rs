use std::{
    collections::{HashSet, VecDeque},
    error::Error,
    fs,
};

use crate::{Combinator, Link, Network};

pub fn main(file: &str) -> Result<usize, Box<dyn Error>> {
    let content = fs::read_to_string(file)?;

    let links: Vec<Link> =
        content.split("\n").filter(|line| *line != "").map(|line| line.parse().unwrap()).collect();

    let mut network = Network::new();

    for link in links.into_iter() {
        network.add(link);
    }

    let mut computers = network.computers();
    computers.sort();

    let mut computers: VecDeque<_> = computers.into_iter().collect();

    let mut fully_connected_networks = Vec::new();
    let mut max_network_size = 0;
    while let Some(comp) = computers.pop_front() {
        let connected: Vec<_> =
            network.linked_to(&comp).into_iter().filter(|c| *c > comp).collect();
        let same_network: HashSet<_> = Combinator::distinct(connected, 2)
            .filter(|cmps| network.is_link_to(&*cmps[0], &*cmps[1]))
            .flat_map(|i| i)
            .filter(|cmp| *cmp > comp)
            .collect();

        let all_connected = same_network.len() > 0
            && Combinator::distinct(&same_network, 2)
                .all(|cmps| network.is_link_to(cmps[0], cmps[1]));

        if all_connected {
            let mut pos: Vec<_> = same_network
                .iter()
                .filter_map(|cmp| computers.iter().position(|c| *c == **cmp))
                .collect();
            pos.sort();
            let mut fully_connected = vec![comp.clone()];
            let mut trail: Vec<_> = same_network.into_iter().collect();
            fully_connected.append(&mut trail);
            fully_connected.sort();
            if max_network_size < fully_connected.len() {
                max_network_size = fully_connected.len();
            }
            println!("Found fully conected network of {} members", fully_connected.len());
            fully_connected_networks.push(fully_connected);
            for idx in pos.iter().rev() {
                computers.remove(*idx);
            }
        }
    }

    let big_networks: Vec<_> =
        fully_connected_networks.iter().filter(|net| net.len() == max_network_size).collect();

    if big_networks.len() != 1 {
        println!("More than one network of size {}", max_network_size);
    } else {
        let net = &big_networks[0];
        println!("Bigest network password (of size {}): {:?}", max_network_size, net.join(","));
    }

    Ok(0)
}
