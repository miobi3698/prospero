use std::sync::Arc;

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
    let program = Arc::new(parse(&source));

    let chunk_count = std::thread::available_parallelism().unwrap().get();
    let chunk_size = (IMAGE_SIZE * IMAGE_SIZE).div_ceil(chunk_count);

    let mut handles = Vec::new();
    for chunk_id in 0..chunk_count {
        let program = program.clone();
        let low = chunk_id * chunk_size;
        let high = ((chunk_id + 1) * chunk_size).min(IMAGE_SIZE * IMAGE_SIZE);
        let mut memory = vec![0.0; program.len()];
        let mut chunk = vec![0 as u8; high - low];

        handles.push(std::thread::spawn(move || {
            let last_idx = program.len() - 1;
            for i in low..high {
                let x = (i % IMAGE_SIZE) as f32 / IMAGE_SIZE as f32 * 2.0 - 1.0;
                let y = (IMAGE_SIZE - (i / IMAGE_SIZE) - 1) as f32 / IMAGE_SIZE as f32 * 2.0 - 1.0;
                exec(&program, &mut memory, x, y);
                chunk[i - low] = (memory[last_idx] < 0.0) as u8 * 255;
            }

            chunk
        }));
    }

    let mut image = Vec::with_capacity(IMAGE_SIZE * IMAGE_SIZE);
    for handle in handles {
        image.extend(handle.join().unwrap());
    }

    std::fs::write(
        "out-multithread.ppm",
        [
            format!("P5\n{IMAGE_SIZE} {IMAGE_SIZE}\n255\n").as_bytes(),
            image.as_slice(),
        ]
        .concat(),
    )
    .unwrap();
}
