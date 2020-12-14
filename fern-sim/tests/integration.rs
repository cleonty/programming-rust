extern crate fern_sim;
use fern_sim::{run_simulation, Fern};

#[test]
fn test_fern_sim() {
  let mut fern = Fern {
    size: 1.0,
    growth_rate: 1.0,
  };
  run_simulation(&mut fern, 1);
  assert_eq!(fern.size, 2.0);
}
