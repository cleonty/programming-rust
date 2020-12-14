//! Fern allows you to manipulate ferns.
//! It's a sample crate to testing purpose.
//! 
//! ```js
//! console.log(1 === 1);
//! ```
#[derive(Debug)]
pub struct Fern {
    pub size: f64,
    pub growth_rate: f64,
}

impl Fern {
    /// Simulate a fern growing for one day.
    pub fn grow(&mut self) {
        self.size *= 1.0 + self.growth_rate;
    }
}

/// Run a fern simulation for some number of days.
pub fn run_simulation(fern: &mut Fern, days: usize) {
    for _ in 0..days {
        fern.grow();
    }
}
