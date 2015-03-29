use manhattan_map::{HexagonMap, Point};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    let map = HexagonMap::<bool>::rhombus(3, 4);
    for (p, maze) in &map {
        println!("{p}: {maze}")
    }
    for p in map.points() {
        println!("{p:?}")
    }
}
