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
    assert_eq!(points, vec![(0, 0)], "Points that distance is 0 to origin");
    let points = DiamondPoints::new(0, 0, 1).collect_vec();
    assert_eq!(points, vec![(1, 0), (0, 1), (-1, 0), (0, -1)], "Points that distance are 1 to origin");
    let points = DiamondPoints::new(0, 0, 2).collect_vec();
    assert_eq!(
        points,
        vec![(2, 0), (1, 1), (0, 2), (-1, 1), (-2, 0), (-1, -1), (0, -2), (1, -1)],
        "Points that distance are 2 to origin"
    );
    let points = DiamondPoints::new(0, 0, 3).collect_vec();
    assert_eq!(
        points,
        vec![(3, 0), (2, 1), (1, 2), (0, 3), (-1, 2), (-2, 1), (-3, 0), (-2, -1), (-1, -2), (0, -3), (1, -2), (2, -1)],
        "Points that distance are 3 to origin"
    );
    let map = TaxicabMap::<usize>::square(4, &1).with_cycle(true, false);
    for (x, y) in DiamondPoints::new(0, 0, 3) {
        println!("({x}, {y}):{:?}", map.get_point(x, y))
    }
}

#[test]
fn test_nearby() {
    let map = TaxicabMap::<usize>::square(4, &1).with_cycle(true, false);
    for (x, y) in map.points_nearby(0, 0) {
        println!("({x}, {y}):{:?}", map.get_point(x, y))
    }
}

#[test]
fn test_path() {
    let mut map = TaxicabMap::<usize>::square(10, &1);
    map.set_cycle(true, true);
    map.shift_origin(-5, -5);
    let (path, cost) = map.path_finder((-9, -9), (9, 9)).with_action_cost(|x, y, _| (x + y).abs() as f64).solve_joint();
    for j in path {
        println!("{j}: {cost}")
    }
}
