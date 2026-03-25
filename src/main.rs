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

// ported from https://github.com/processing/p5.js/blob/v1.11.11/src/math/calculation.js#L534
fn remap_range(value: i64, start1: i64, stop1: i64, start2: i64, stop2: i64) -> f64 {
    (value - start1) as f64 / (stop1 - start1) as f64 * (stop2 - start2) as f64 + start2 as f64
}

const IMAGE_SIZE: usize = 512;
const IMAGE_CAP: usize = (IMAGE_SIZE * IMAGE_SIZE) as usize;

fn main() {
    let source = std::fs::read_to_string("prospero.vm").unwrap();

    let timer = std::time::Instant::now();
    let program = std::sync::Arc::new(parse(&source));

    let mut coords: Vec<(f64, f64)> = Vec::with_capacity(IMAGE_CAP);
    for i in 0..IMAGE_SIZE {
        let y = remap_range(i as i64, 0, IMAGE_SIZE as i64, 1, -1);
        for j in 0..IMAGE_SIZE {
            let x = remap_range(j as i64, 0, IMAGE_SIZE as i64, -1, 1);
            coords.push((x, y));
        }
    }

    let thread_pool_size = std::thread::available_parallelism().unwrap().get();
    let progress = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));

    let mut handles = Vec::new();
    for chunk in coords.chunks(IMAGE_CAP / thread_pool_size) {
        let chunk = chunk.to_owned();
        let program = std::sync::Arc::clone(&program);
        let progress = std::sync::Arc::clone(&progress);
        let handle = std::thread::spawn(move || {
            let mut image: Vec<u8> = Vec::with_capacity(chunk.len());
            let mut memory = Vec::<f64>::with_capacity(program.len());
            for (x, y) in chunk {
                for inst in program.iter() {
                    memory.push(match *inst {
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
                    });
                }

                image.push((memory[program.len() - 1] < 0.0) as u8 * 255);
                let count = progress.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
                eprint!("\rProgress: {}/{} pixels", count, IMAGE_CAP);
                memory.clear();
            }

            image
        });
        handles.push(handle);
    }

    let mut image = Vec::new();
    for handle in handles {
        image.extend_from_slice(&handle.join().unwrap());
    }
    eprintln!("\nDone in {}s.", timer.elapsed().as_secs_f64());
    // Initial implementation: 1476.815809719s
    // Swapped to flat memory: 44.731658977s
    // Moved to multithreading: 7.466684195s

    let image_data = [
        format!("P5\n{IMAGE_SIZE} {IMAGE_SIZE}\n255\n").as_bytes(),
        image.as_slice(),
    ]
    .concat();
    std::fs::write("out.ppm", image_data).unwrap();
}
