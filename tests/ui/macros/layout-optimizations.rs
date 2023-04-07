#![feature(pattern_types)]
#![allow(incomplete_features)]

fn main() {
    type Wrong = layout_optimizations!(u32, 1 | 2);
    //~^ Pattern types only support range expressions
    type Percent = layout_optimizations!(u32, 0..=100);
    //~^ pattern types are unstable
    let blah: layout_optimizations!(u32, 123..) = 256;
}
