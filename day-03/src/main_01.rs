use std::{error::Error, fs};

enum Step {
    Init,
    OnM,
    OnU,
    OnL,
    OnNumber1,
    OnNumber2,
}

struct Context {
    pub multiplications: Vec<[i32; 2]>,
    pub step: Step,
    pub num_buffer: String,
    pub numbers: [Option<i32>; 2],
}

impl Context {
    fn new() -> Self {
        Context {
            multiplications: vec![],
            step: Step::Init,
            num_buffer: String::new(),
            numbers: [None, None],
        }
    }

    fn reset(&mut self) {
        self.step = Step::Init;
        self.num_buffer = String::new();
        self.numbers = [None, None];
    }

    fn start(&mut self) {
        self.step = Step::OnM;
        self.num_buffer = String::new();
    }

    fn parse_buffer(&mut self, idx: usize) -> bool {
        match self.num_buffer.parse() {
            Ok(num) => { self.numbers[idx] = Some(num); true}
            _ => false
        }
    }

    fn save_multiplication(&mut self) {
        let [a, b] = self.numbers;
        println!("numbers: {:?}, {:?}", a, b);
        self.multiplications.push([a.unwrap(), b.unwrap()]);
        self.numbers = [None, None];
    }
}

fn is_number(c: char) -> bool {
    c >= '0' && c <= '9'
}

pub fn main(file: &str) -> Result<i32, Box<dyn Error>> {
    let content = fs::read_to_string(file)?;

    let mut ctx = Context::new();

    content.chars().for_each(|char| {
        if char == 'm' {
            ctx.start();
        } else if char == 'u' && matches!(ctx.step, Step::OnM) {
            ctx.step = Step::OnU;
        } else if char == 'l' && matches!(ctx.step, Step::OnU) {
            ctx.step = Step::OnL;
        } else if char == '(' && matches!(ctx.step, Step::OnL) {
            ctx.step = Step::OnNumber1;
            ctx.num_buffer = String::new();
        } else if is_number(char) && matches!(ctx.step, Step::OnNumber1) {
            ctx.num_buffer += &char.to_string();
            if ctx.num_buffer.len() > 3 {
                ctx.reset();
            }
        } else if char == ',' && matches!(ctx.step, Step::OnNumber1) {
            ctx.step = Step::OnNumber2;
            if ctx.parse_buffer(0) {
                ctx.num_buffer = String::new();
            } else {
                ctx.reset();
            }
        } else if is_number(char) && matches!(ctx.step, Step::OnNumber2) {
            ctx.step = Step::OnNumber2;
            ctx.num_buffer += &char.to_string();
            if ctx.num_buffer.len() > 3 {
                ctx.reset();
            }
        } else if char == ')' && matches!(ctx.step, Step::OnNumber2) {
            if ctx.parse_buffer(1) {
                ctx.save_multiplication();
            }
            ctx.reset();
        } else {
            ctx.reset();
        }
    });

    let result = ctx
        .multiplications
        .into_iter()
        .map(|[a, b]| a * b)
        .fold(0, |acc, num| acc + num);

    Ok(result)
}
