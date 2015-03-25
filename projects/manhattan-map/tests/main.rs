use hexagon_map::{AxialPoint, HexagonMap};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    let map = HexagonMap::<bool>::width_first(3, 4, true);
    for (p, maze) in &map {
        println!("{p}: {maze}")
    }
    for p in map.points() {
        println!("{p:?}")
    }
}

#[test]
fn test2() {
    let map = HexagonMap::<bool>::width_first(3, 4, true);
    let cost = map.action_field(AxialPoint::new(0, 0), 10.0).with_cost(|p, _| (p.r + p.q).abs() as f64);
    for (p, maze) in cost {
        println!("{p}: {maze}")
    }
}
