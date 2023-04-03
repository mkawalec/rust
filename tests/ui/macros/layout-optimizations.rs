// run-pass
#![feature(pattern_types)]

fn main() {
    let blah: layout_optimizations!(u32, 123..) = 256;
}
