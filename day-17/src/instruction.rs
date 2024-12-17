#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Adv(u64),
    Bxl(u64),
    Bst(u64),
    Jnz(u64),
    Bxc(u64),
    Out(u64),
    Bdv(u64),
    Cdv(u64),
}

impl Instruction {
    pub fn from_str(s: &str) -> Vec<Self> {
        Self::from_numbers(s.split(",").map(|i| i.parse().unwrap()).collect())
    }

    pub fn from_numbers(numbers: Vec<u64>) -> Vec<Self> {
        let mut ret = Vec::new();
        for i in (0..numbers.len()).step_by(2) {
            let instruction = match numbers[i] {
                0 => Self::Adv(numbers[i + 1]),
                1 => Self::Bxl(numbers[i + 1]),
                2 => Self::Bst(numbers[i + 1]),
                3 => Self::Jnz(numbers[i + 1]),
                4 => Self::Bxc(numbers[i + 1]),
                5 => Self::Out(numbers[i + 1]),
                6 => Self::Bdv(numbers[i + 1]),
                7 => Self::Cdv(numbers[i + 1]),
                _ => unreachable!(),
            };
            ret.push(instruction);
        }
        ret
    }
}
