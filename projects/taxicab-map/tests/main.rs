use itertools::Itertools;
use taxicab_map::{DiamondPoints, TaxicabMap};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    let map = TaxicabMap::<usize>::rectangle(3, 4, &1);
    for (x, y, maze) in &map {
        println!("{x}, {y}: {maze}")
    }
}

#[test]
fn test_diamond() {
    let points = DiamondPoints::new(0, 0, 0).collect_vec();
    assert_eq!(points, vec![(0, 0)], "0");
    let points = DiamondPoints::new(0, 0, 1).collect_vec();
    assert_eq!(points, vec![(1, 0), (0, 1), (-1, 0), (0, -1)], "1");
}
