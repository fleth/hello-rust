fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

fn swap(x: i32, y: i32) -> (i32, i32) {
    return (y ,x);
}

fn make_nothing() -> () {
    return ();
}

fn make_nothing2() {
    // () が返る
}

fn main() {
    println!("{}", add(42, 13));

    let result = swap(123, 321);
    println!("{} {}", result.0, result.1);

    let (a, b) = swap(result.0, result.1);
    println!("{} {}", a, b);

    let a = make_nothing();
    let b = make_nothing2();

    println!("aの値: {:?}", a);
    println!("bの値: {:?}", b);
}
