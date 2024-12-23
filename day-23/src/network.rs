use std::collections::{HashMap, HashSet};

use crate::{Combinator, Link};

#[derive(Debug, Clone)]
pub struct Network {
    links: Vec<Link>,
    link_to: HashMap<String, Vec<String>>,
}

impl Network {
    pub fn new() -> Self {
        Self { links: Vec::new(), link_to: HashMap::new() }
    }

    pub fn add(&mut self, link: Link) {
        let link_a = link.clone();
        let entry = self.link_to.entry(link_a.a).or_insert(Vec::new());
        entry.push(link_a.b);
        entry.sort();

        let link_b = link.clone();
        let entry = self.link_to.entry(link_b.b).or_insert(Vec::new());
        entry.push(link_b.a);
        entry.sort();

        self.links.push(link);
        self.links.sort();
    }

    pub fn is_link_to(&self, a: &str, b: &str) -> bool {
        if let Some(links) = self.link_to.get(a) {
            return links.contains(&b.to_string());
        }
        false
    }

    pub fn links(&self) -> &[Link] {
        &self.links
    }

    pub fn linked_to(&self, comp: &str) -> Vec<String> {
        self.link_to[comp].clone()
    }

    pub fn computers(&self) -> Vec<String> {
        let mut ret: Vec<_> = self.link_to.keys().cloned().collect();
        ret.sort();
        ret
    }

    pub fn get_links(&self, size: usize) -> Vec<Vec<String>> {
        let ret: HashSet<_> = self
            .computers()
            .into_iter()
            .filter(|comp| comp.starts_with("t"))
            .flat_map(|comp| {
                let others = self.linked_to(&comp);
                let mut sub_links: Vec<_> = Combinator::distinct(others, size - 1)
                    .filter(|computers| {
                        Combinator::distinct(computers, 2).all(|comps| {
                            let a = &comps[0];
                            let b = &comps[1];
                            self.is_link_to(a, b)
                        })
                    })
                    .collect();
                for link in sub_links.iter_mut() {
                    link.push(comp.clone());
                    link.sort();
                }
                sub_links
            })
            .collect();
        let mut ret: Vec<_> = ret.into_iter().collect();
        ret.sort();
        ret
    }
}
