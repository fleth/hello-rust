fn main() {
    base();
    types();
    cast();
}

fn types() {
    println!("---[types]---");
    let x = 12; // デフォルト i32
    let a = 12u8;
    let b = 4.3; // デフォルトでは f64
    let c = 4.3f32;
    let bv = true;
    let t = (13, false);
    let sentence = "hello world!";
    println!(
        "{} {} {} {} {} {} {} {}",
        x, a, b, c, bv, t.0, t.1, sentence
    );

    // const
    const PI: f32 = 3.14159;
    println!("{}", PI);

    // array
    let nums: [i32; 3] = [1, 2, 3];
    println!("{:?}", nums);
    println!("{}", nums[1]);

}

fn base() {
    println!("---[base]---");
    let x = 13; // 型推論
    println!("{}", x);

    let x: f64 = 3.14159; // 型指定
    println!("{}", x);

    let x;
    x = 0; // 宣言後の初期化 初期化後は不変
    println!("{}", x);

    let mut x = 42; // 可変
    println!("{}", x);
    x = 13;
    println!("{}", x);
}

fn cast() {
    println!("---[cast]---");
    let a = 13u8;
    let b = 7u32;
    let c = a as u32 + b;
    println!("{}", c);

    let t = true;
    println!("{}", t as u8);
}
