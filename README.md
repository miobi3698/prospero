# The Prospero challenge renderer

The result of the `prospero.vm` from the [The Prospero challenge](https://www.mattkeeter.com/projects/prospero/) rendered onto a [ppm](https://netpbm.sourceforge.net/doc/ppm.html) image file.

## Benchmarks

Image dimensions: 256x256

```sh
$ hyperfine --warmup 3 ./target/release/baseline ./target/release/bytecode ./target/release/multithread ./target/release/parallel
Benchmark 1: ./target/release/baseline
  Time (mean ± σ):     82.798 s ±  0.352 s    [User: 82.745 s, System: 0.024 s]
  Range (min … max):   82.200 s … 83.265 s    10 runs

Benchmark 2: ./target/release/bytecode
  Time (mean ± σ):      2.960 s ±  0.018 s    [User: 2.957 s, System: 0.002 s]
  Range (min … max):    2.928 s …  2.985 s    10 runs

Benchmark 3: ./target/release/multithread
  Time (mean ± σ):     393.8 ms ±  12.8 ms    [User: 4034.7 ms, System: 9.8 ms]
  Range (min … max):   376.3 ms … 419.5 ms    10 runs

Benchmark 4: ./target/release/parallel
  Time (mean ± σ):     366.6 ms ±   7.3 ms    [User: 4000.5 ms, System: 13.1 ms]
  Range (min … max):   360.3 ms … 383.6 ms    10 runs

Summary
  ./target/release/parallel ran
    1.07 ± 0.04 times faster than ./target/release/multithread
    8.07 ± 0.17 times faster than ./target/release/bytecode
  225.84 ± 4.60 times faster than ./target/release/baseline
```
