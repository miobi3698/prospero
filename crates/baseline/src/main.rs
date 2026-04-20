use std::collections::HashMap;

const IMAGE_SIZE: usize = 256;

fn main() {
    let source = std::fs::read_to_string("prospero.vm").unwrap();
    let mut memory = HashMap::new();

    let mut image = Vec::new();
    for i in 0..IMAGE_SIZE {
        let y = (IMAGE_SIZE - i - 1) as f32 / IMAGE_SIZE as f32 * 2.0 - 1.0;
        for j in 0..IMAGE_SIZE {
            let x = j as f32 / IMAGE_SIZE as f32 * 2.0 - 1.0;
            let mut out = "";
            for line in source.lines().skip(1) {
                let words = line.split_whitespace().collect::<Vec<&str>>();
                out = words[0];
                memory.insert(
                    out,
                    match words[1] {
                        "var-x" => x,
                        "var-y" => y,
                        "const" => words[2].parse().unwrap(),
                        "add" => memory[words[2]] + memory[words[3]],
                        "sub" => memory[words[2]] - memory[words[3]],
                        "mul" => memory[words[2]] * memory[words[3]],
                        "max" => f32::max(memory[words[2]], memory[words[3]]),
                        "min" => f32::min(memory[words[2]], memory[words[3]]),
                        "neg" => -memory[words[2]],
                        "square" => memory[words[2]] * memory[words[2]],
                        "sqrt" => f32::sqrt(memory[words[2]]),
                        _ => unreachable!(),
                    },
                );
            }

            image.push((memory[out] < 0.0) as u8 * 255);
        }
    }

    std::fs::write(
        "out-baseline.ppm",
        [
            format!("P5\n{IMAGE_SIZE} {IMAGE_SIZE}\n255\n").as_bytes(),
            image.as_slice(),
        ]
        .concat(),
    )
    .unwrap();
}
