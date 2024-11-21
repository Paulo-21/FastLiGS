mod graph;
mod cli;
mod parser;
mod gr_solver;
mod nn;
mod gradualsemantics_opt;
mod model;
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() {
    cli::launcher();
}
