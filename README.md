# NickOS

Simple OS written in Rust, just for learning purpouses. 

Nick is in honor of my son, Nicolas.

## Building

With `nightly` Rust:

```
$ cargo install cargo-xbuild bootimage

$ rustup component add rust-src

$ rustup component add llvm-tools-preview
```

Now build:

```
$ cargo xbuild
  Downloaded compiler_builtins v0.1.27
  Downloaded 1 crate (135.0 KB) in 2.18s
   Compiling compiler_builtins v0.1.27
   Compiling core v0.0.0 (/home/marcelocastellani/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore)
   Compiling rustc-std-workspace-core v1.99.0 (/home/marcelocastellani/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/tools/rustc-std-workspace-core)
   Compiling alloc v0.0.0 (/tmp/cargo-xbuild.f6l5bvMfm2Bw)
    Finished release [optimized] target(s) in 14.01s
   Compiling bootloader v0.9.4
   Compiling nick_os v0.1.0 (/home/marcelocastellani/data/nick_os)
    Finished dev [unoptimized + debuginfo] target(s) in 0.36s


$ cargo bootimage
Building kernel
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
Building bootloader
   Compiling bootloader v0.9.4 (/home/marcelocastellani/.cargo/registry/src/github.com-1ecc6299db9ec823/bootloader-0.9.4)
    Finished release [optimized + debuginfo] target(s) in 0.77s
Created bootimage for `nick_os` at `/home/marcelocastellani/data/nick_os/target/x86_64-blog_os/debug/bootimage-nick_os.bin`

```

## Running

Install QEMU and run:

```
$ cargo xrun
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `bootimage runner target/x86_64-blog_os/debug/nick_os`
Building bootloader
    Finished release [optimized + debuginfo] target(s) in 0.01s
Running: `qemu-system-x86_64 -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-nick_os.bin`
```

![NickOS no QEMU](qemurun.png)
