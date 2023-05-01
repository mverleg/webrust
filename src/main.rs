use ::clap::Parser;

use crate::args::Args;

#[cfg(feature = "jemalloc")]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

mod args;

fn main() {
    let args = Args::parse();
    todo!("Hello, world!");
}
