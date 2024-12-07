pub fn each<T: Clone>(base: &Vec<T>, size: usize) -> Vec<Vec<T>> {
    if size <= 1 {
        return base.iter().map(|item| vec![item.clone()]).collect();
    }
    base.into_iter()
        .flat_map(|item| {
            each(base, size - 1)
                .into_iter()
                .map(|items| {
                    let mut items = items.clone();
                    items.push(item.clone());
                    items
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_combination_test() {
        assert_eq!(each(&vec![1, 2, 3], 1), vec![vec![1], vec![2], vec![3],]);
        assert_eq!(
            each(&vec![1, 2, 3], 2),
            vec![
                vec![1, 1],
                vec![2, 1],
                vec![3, 1],
                vec![1, 2],
                vec![2, 2],
                vec![3, 2],
                vec![1, 3],
                vec![2, 3],
                vec![3, 3],
            ]
        );
    }
}
