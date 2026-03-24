enum Op<'a> {
    VarX,
    VarY,
    Const(f32),
    Add(&'a str, &'a str),
    Sub(&'a str, &'a str),
    Mul(&'a str, &'a str),
    Max(&'a str, &'a str),
    Min(&'a str, &'a str),
    Neg(&'a str),
    Square(&'a str),
    Sqrt(&'a str),
}

struct Inst<'a> {
    addr: &'a str,
    op: Op<'a>,
}

fn parse<'a>(source: &'a str) -> Vec<Inst<'a>> {
    let mut program = Vec::new();

    for line in source.lines() {
        if line.starts_with('#') {
            continue;
        }

        let inst = line.split_whitespace().collect::<Vec<_>>();
        let addr = inst[0];
        let op = inst[1];

        let op = match op {
            "var-x" => Op::VarX,
            "var-y" => Op::VarY,
            "const" => Op::Const(str::parse(inst[2]).unwrap()),
            "add" => Op::Add(inst[2], inst[3]),
            "sub" => Op::Sub(inst[2], inst[3]),
            "mul" => Op::Mul(inst[2], inst[3]),
            "max" => Op::Max(inst[2], inst[3]),
            "min" => Op::Min(inst[2], inst[3]),
            "neg" => Op::Neg(inst[2]),
            "square" => Op::Square(inst[2]),
            "sqrt" => Op::Sqrt(inst[2]),
            _ => unreachable!(),
        };

        program.push(Inst { addr, op });
    }

    program
}

// ported from https://github.com/processing/p5.js/blob/v1.11.11/src/math/calculation.js#L534
fn remap_range(value: i32, start1: i32, stop1: i32, start2: i32, stop2: i32) -> f32 {
    (value - start1) as f32 / (stop1 - start1) as f32 * (stop2 - start2) as f32 + start2 as f32
}

fn main() {
    let image_size = 512;
    let image_cap = (image_size * image_size) as usize;

    let timer = std::time::Instant::now();
    let source = std::fs::read_to_string("prospero.vm").unwrap();
    let program = parse(&source);
    let mut image = Vec::<u8>::with_capacity(image_cap);
    let mut memory = std::collections::HashMap::<&str, f32>::new();
    for i in 0..image_size {
        let y = remap_range(i, image_size, 0, -1, 1);
        for j in 0..image_size {
            let x = remap_range(j, 0, image_size, -1, 1);

            for inst in &program {
                memory.insert(
                    inst.addr,
                    match inst.op {
                        Op::VarX => x,
                        Op::VarY => y,
                        Op::Const(v) => v,
                        Op::Add(a1, a2) => memory[a1] + memory[a2],
                        Op::Sub(a1, a2) => memory[a1] - memory[a2],
                        Op::Mul(a1, a2) => memory[a1] * memory[a2],
                        Op::Max(a1, a2) => f32::max(memory[a1], memory[a2]),
                        Op::Min(a1, a2) => f32::min(memory[a1], memory[a2]),
                        Op::Neg(a) => -memory[a],
                        Op::Square(a) => memory[a] * memory[a],
                        Op::Sqrt(a) => f32::sqrt(memory[a]),
                    },
                );
            }

            image.push((memory[program.last().unwrap().addr] < 0.0) as u8 * 255);
            eprint!("\rProgress: {}/{}", i * image_size + j + 1, image_cap);
        }
    }
    eprintln!("\nDone in {}s.", timer.elapsed().as_secs_f64());
    // Initial implementation: 1476.815809719s

    let image_data = [
        format!("P5\n{image_size} {image_size}\n255\n").as_bytes(),
        image.as_slice(),
    ]
    .concat();
    std::fs::write("out.ppm", image_data).unwrap();
}
