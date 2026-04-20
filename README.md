# The Prospero challenge renderer

The result of the `prospero.vm` from the [The Prospero challenge](https://www.mattkeeter.com/projects/prospero/) rendered onto a [ppm](https://netpbm.sourceforge.net/doc/ppm.html) image file.

## Benchmarks

Image dimensions: 256x256

```sh
$ hyperfine ./target/release/baseline ./target/release/bytecode ./target/release/multithread
Benchmark 1: ./target/release/baseline
  Time (mean ± σ):     80.380 s ±  0.894 s    [User: 80.347 s, System: 0.007 s]
  Range (min … max):   79.395 s … 81.973 s    10 runs

Benchmark 2: ./target/release/bytecode
  Time (mean ± σ):      2.871 s ±  0.051 s    [User: 2.855 s, System: 0.002 s]
  Range (min … max):    2.830 s …  3.008 s    10 runs

Benchmark 3: ./target/release/multithread
  Time (mean ± σ):     360.9 ms ±  14.3 ms    [User: 3957.3 ms, System: 7.7 ms]
  Range (min … max):   346.6 ms … 393.6 ms    10 runs

Summary
  ./target/release/multithread ran
    7.96 ± 0.34 times faster than ./target/release/bytecode
  222.70 ± 9.16 times faster than ./target/release/baseline
```
