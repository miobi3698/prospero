use rayon::prelude::*;

enum Inst {
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
        program.push(match inst[1] {
            "var-x" => Inst::VarX,
            "var-y" => Inst::VarY,
            "const" => Inst::Const(str::parse(inst[2]).unwrap()),
            "add" => Inst::Add(translate_address(inst[2]), translate_address(inst[3])),
            "sub" => Inst::Sub(translate_address(inst[2]), translate_address(inst[3])),
            "mul" => Inst::Mul(translate_address(inst[2]), translate_address(inst[3])),
            "max" => Inst::Max(translate_address(inst[2]), translate_address(inst[3])),
            "min" => Inst::Min(translate_address(inst[2]), translate_address(inst[3])),
            "neg" => Inst::Neg(translate_address(inst[2])),
            "square" => Inst::Square(translate_address(inst[2])),
            "sqrt" => Inst::Sqrt(translate_address(inst[2])),
            _ => unreachable!(),
        });
    }

    program
}

fn exec(program: &[Inst], memory: &mut [f64], x: f64, y: f64) {
    for (idx, inst) in program.iter().enumerate() {
        memory[idx] = match *inst {
            Inst::VarX => x,
            Inst::VarY => y,
            Inst::Const(v) => v,
            Inst::Add(a1, a2) => memory[a1] + memory[a2],
            Inst::Sub(a1, a2) => memory[a1] - memory[a2],
            Inst::Mul(a1, a2) => memory[a1] * memory[a2],
            Inst::Max(a1, a2) => f64::max(memory[a1], memory[a2]),
            Inst::Min(a1, a2) => f64::min(memory[a1], memory[a2]),
            Inst::Neg(a) => -memory[a],
            Inst::Square(a) => memory[a] * memory[a],
            Inst::Sqrt(a) => f64::sqrt(memory[a]),
        };
    }
}

// ported from https://github.com/processing/p5.js/blob/v1.11.11/src/math/calculation.js#L534
fn remap_range(value: i64, start1: i64, stop1: i64, start2: i64, stop2: i64) -> f64 {
    (value - start1) as f64 / (stop1 - start1) as f64 * (stop2 - start2) as f64 + start2 as f64
}

const IMAGE_SIZE: usize = 512;
// const IMAGE_SIZE: usize = 1024;
const IMAGE_CAP: usize = IMAGE_SIZE * IMAGE_SIZE;

fn main() {
    let source = std::fs::read_to_string("prospero.vm").unwrap();
    let timer = std::time::Instant::now();
    let program = parse(&source);
    let progress = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));

    let image: Vec<u8> = (0..IMAGE_CAP)
        .into_par_iter()
        .map(|idx| {
            let progress = progress.clone();
            let mut memory = vec![0.0; program.len()];

            let x = idx % IMAGE_SIZE;
            let y = idx / IMAGE_SIZE;
            let vy = remap_range(y as i64, 0, IMAGE_SIZE as i64, 1, -1);
            let vx = remap_range(x as i64, 0, IMAGE_SIZE as i64, -1, 1);

            exec(&program, &mut memory, vx, vy);
            let count = progress.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            eprint!("\rProgress: {}/{} pixels", count + 1, IMAGE_CAP);

            (*memory.last().unwrap() < 0.0) as u8 * 255
        })
        .collect();

    eprintln!("\nDone in {}s.", timer.elapsed().as_secs_f64());

    // 512x512
    // Initial implementation: 1476.815809719s
    // Swapped to flat memory: 44.731658977s
    // Moved to multithreading: 7.466684195s
    // Swapped to flat image: 6.111408442s
    // Moved arc-mutex to owned image: 6.5873183730000004s (invalid: wrong result)
    // Moved to rayon: 6.382879828s

    // 1024x1024
    // Multithreading: 30.092374533s

    let image_data = [
        format!("P5\n{IMAGE_SIZE} {IMAGE_SIZE}\n255\n").as_bytes(),
        image.as_slice(),
    ]
    .concat();
    std::fs::write("out.ppm", image_data).unwrap();
}
