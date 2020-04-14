# Private resource allocators (PRA)

This library contains a set of private resource allocators that are
useful at allocating resources without leaking information about which
(or how many) resources are allocated. For details on PRAs, see our
[paper](https://www.cis.upenn.edu/~sga001/papers/pra-sp20.pdf) which appeared
at the IEEE Symposium of Security and Privacy (S&P) 2020.

# Compile
The code base is written in Rust and we have tested up to rustc version 1.42.0.

You can install Rust via [rustup](https://rustup.rs).
To install this particular version of Rust, simply go to PRA's directory and 
run:

```sh
$ rustup override set nightly-2020-03-09
```

To compile, simply run:

```sh
$ cargo build --release
```

This will produce several binaries in the "target/release/" folder. These
binaries can be use to reproduce the results in our paper.

In addition, you can run our microbenchmarks by calling

```sh
$ cargo bench
```
