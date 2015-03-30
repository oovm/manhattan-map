use manhattan_map::{TaxicabMap};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    let map = TaxicabMap::<bool>::square(3, 4);
    for (p, maze) in &map {
        println!("{p}: {maze}")
    }
    for p in map.points() {
        println!("{p:?}")
    }
}
