extern crate fern_sim;
use fern_sim::{Fern, run_simulation};

fn main() {
    let mut fern = Fern {
        size: 0.1,
        growth_rate: 0.001,
    };
    run_simulation(&mut fern, 1000);
    println!("final size: {}", fern.size);
}
