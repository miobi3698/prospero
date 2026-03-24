#[derive(Debug)]
enum Op {
    VarX,
    VarY,
    Const(f64),
    Add(usize, usize),
    Sub(usize, usize),
    Mul(usize, usize),
    Max(usize, usize),
    Min(usize, usize),
    Neg(usize),
    Square(usize),
    Sqrt(usize),
}

#[derive(Debug)]
struct Inst {
    op: Op,
}

fn translate_address(addr: &str) -> usize {
    usize::from_str_radix(addr.trim_start_matches('_'), 16).unwrap()
}

fn parse(source: &str) -> Vec<Inst> {
    let mut program = Vec::new();

    for line in source.lines() {
        if line.starts_with('#') {
            continue;
        }

        let inst = line.split_whitespace().collect::<Vec<_>>();
        let op = match inst[1] {
            "var-x" => Op::VarX,
            "var-y" => Op::VarY,
            "const" => Op::Const(str::parse(inst[2]).unwrap()),
            "add" => Op::Add(translate_address(inst[2]), translate_address(inst[3])),
            "sub" => Op::Sub(translate_address(inst[2]), translate_address(inst[3])),
            "mul" => Op::Mul(translate_address(inst[2]), translate_address(inst[3])),
            "max" => Op::Max(translate_address(inst[2]), translate_address(inst[3])),
            "min" => Op::Min(translate_address(inst[2]), translate_address(inst[3])),
            "neg" => Op::Neg(translate_address(inst[2])),
            "square" => Op::Square(translate_address(inst[2])),
            "sqrt" => Op::Sqrt(translate_address(inst[2])),
            _ => unreachable!(),
        };

        program.push(Inst { op });
    }

    program
}

// ported from https://github.com/processing/p5.js/blob/v1.11.11/src/math/calculation.js#L534
fn remap_range(value: i64, start1: i64, stop1: i64, start2: i64, stop2: i64) -> f64 {
    (value - start1) as f64 / (stop1 - start1) as f64 * (stop2 - start2) as f64 + start2 as f64
}

fn main() {
    let image_size = 512;
    let image_cap = (image_size * image_size) as usize;
    let timer = std::time::Instant::now();
    let source = std::fs::read_to_string("prospero.vm").unwrap();
    let program = parse(&source);

    let mut image = Vec::<u8>::with_capacity(image_cap);
    for i in 0..image_size {
        let y = remap_range(i, 0, image_size, -1, 1);
        for j in 0..image_size {
            let x = remap_range(j, 0, image_size, -1, 1);
            let mut memory = Vec::<f64>::with_capacity(program.len());

            for inst in &program {
                memory.push(match inst.op {
                    Op::VarX => x,
                    Op::VarY => y,
                    Op::Const(v) => v,
                    Op::Add(a1, a2) => memory[a1] + memory[a2],
                    Op::Sub(a1, a2) => memory[a1] - memory[a2],
                    Op::Mul(a1, a2) => memory[a1] * memory[a2],
                    Op::Max(a1, a2) => f64::max(memory[a1], memory[a2]),
                    Op::Min(a1, a2) => f64::min(memory[a1], memory[a2]),
                    Op::Neg(a) => -memory[a],
                    Op::Square(a) => memory[a] * memory[a],
                    Op::Sqrt(a) => f64::sqrt(memory[a]),
                });
            }

            image.push((memory[program.len() - 1] < 0.0) as u8 * 255);
            eprint!("\rProgress: {}/{}", i * image_size + j + 1, image_cap);
        }
    }
    eprintln!("\nDone in {}s.", timer.elapsed().as_secs_f64());
    // Initial implementation: 1476.815809719s
    // Swapped to flat memory: 44.731658977s

    let image_data = [
        format!("P5\n{image_size} {image_size}\n255\n").as_bytes(),
        image.as_slice(),
    ]
    .concat();
    std::fs::write("out.ppm", image_data).unwrap();
}
