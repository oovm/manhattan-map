use taxicab_map::TaxicabMap;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    let map = TaxicabMap::<usize>::rectangle(3, 4, &1);
    for (p, maze) in &map {
        println!("{p}: {maze}")
    }
}
