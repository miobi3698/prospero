enum Inst {
    VarX,
    VarY,
    Const(f32),
    Add(usize, usize),
    Sub(usize, usize),
    Mul(usize, usize),
    Max(usize, usize),
    Min(usize, usize),
    Neg(usize),
    Square(usize),
    Sqrt(usize),
}

fn parse_addr(word: &str) -> usize {
    return usize::from_str_radix(&word[1..], 16).unwrap();
}

fn parse(source: &str) -> Vec<Inst> {
    source
        .lines()
        .skip(1)
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .map(|words| match words[1] {
            "var-x" => Inst::VarX,
            "var-y" => Inst::VarY,
            "const" => Inst::Const(words[2].parse().unwrap()),
            "add" => Inst::Add(parse_addr(words[2]), parse_addr(words[3])),
            "sub" => Inst::Sub(parse_addr(words[2]), parse_addr(words[3])),
            "mul" => Inst::Mul(parse_addr(words[2]), parse_addr(words[3])),
            "max" => Inst::Max(parse_addr(words[2]), parse_addr(words[3])),
            "min" => Inst::Min(parse_addr(words[2]), parse_addr(words[3])),
            "neg" => Inst::Neg(parse_addr(words[2])),
            "square" => Inst::Square(parse_addr(words[2])),
            "sqrt" => Inst::Sqrt(parse_addr(words[2])),
            _ => unreachable!(),
        })
        .collect()
}

fn exec(program: &[Inst], memory: &mut [f32], x: f32, y: f32) {
    for (out, inst) in program.iter().enumerate() {
        memory[out] = match *inst {
            Inst::VarX => x,
            Inst::VarY => y,
            Inst::Const(value) => value,
            Inst::Add(left, right) => memory[left] + memory[right],
            Inst::Sub(left, right) => memory[left] - memory[right],
            Inst::Mul(left, right) => memory[left] * memory[right],
            Inst::Max(left, right) => f32::max(memory[left], memory[right]),
            Inst::Min(left, right) => f32::min(memory[left], memory[right]),
            Inst::Neg(addr) => -memory[addr],
            Inst::Square(addr) => memory[addr] * memory[addr],
            Inst::Sqrt(addr) => f32::sqrt(memory[addr]),
        };
    }
}

const IMAGE_SIZE: usize = 256;

fn main() {
    let source = std::fs::read_to_string("prospero.vm").unwrap();
    let program = parse(&source);
    let mut memory = vec![0.0; program.len()];

    let mut image = Vec::new();
    for i in 0..IMAGE_SIZE {
        let y = (IMAGE_SIZE - i - 1) as f32 / IMAGE_SIZE as f32 * 2.0 - 1.0;
        for j in 0..IMAGE_SIZE {
            let x = j as f32 / IMAGE_SIZE as f32 * 2.0 - 1.0;
            exec(&program, &mut memory, x, y);
            image.push((*memory.last().unwrap() < 0.0) as u8 * 255);
        }
    }

    std::fs::write(
        "out-bytecode.ppm",
        [
            format!("P5\n{IMAGE_SIZE} {IMAGE_SIZE}\n255\n").as_bytes(),
            image.as_slice(),
        ]
        .concat(),
    )
    .unwrap();
}
