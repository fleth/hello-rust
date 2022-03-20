struct BagOfHolding<T> {
    item: T,
}

struct BagOfHoldingOpt<T> {
    item: Option<T>,
}

fn generics() {
    println!("---[generics]---");
    let i32_bag = BagOfHolding::<i32> { item: 42};
    let bool_bag = BagOfHolding::<bool> { item: true };

    let float_bag = BagOfHolding { item: 3.14 };

    let bag_in_bag = BagOfHolding {
        item: BagOfHolding { item: "boom!" },
    };

    println!(
        "{} {} {} {}",
        i32_bag.item, bool_bag.item, float_bag.item, bag_in_bag.item.item
    );
}

fn fnoption() {
    println!("---[fnoption]---");
    let i32_bag = BagOfHoldingOpt::<i32> { item: None };
    if i32_bag.item.is_none() {
        println!("バッグには何もない！");
    } else {
        println!("バッグには何かある！");
    }

    let i32_bag = BagOfHoldingOpt::<i32> { item: Some(42) };
    if i32_bag.item.is_some() {
        println!("バッグには何かある！");
    } else {
        println!("バッグには何もない！");
    }

    match i32_bag.item {
        Some(v) => println!("バッグに{}を発見！", v),
        None => println!("何も見つからなかった"),
    }
}

fn do_something_that_might_fail(i:i32) -> Result<f32,String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("正しい値ではありません"))
    }
}

fn fnresult() {
    println!("---[fnresult]---");
    let result = do_something_that_might_fail(12);

    match result {
        Ok(v) => println!("発見 {}", v),
        Err(e) => println!("Error: {}", e),
    }
}

fn simpleresult() -> Result<(), String> {
    println!("---[simpleresult]---");
    let v = do_something_that_might_fail(42)?; // 末尾?でErrならErrをreturnする
    println!("発見{}", v);
    Ok(())
}

fn panic() -> Result<(), String> {
    println!("---[panic]---");
    let v = do_something_that_might_fail(42).unwrap(); // 値を取り出す
    println!("発見{}", v);

    let v = do_something_that_might_fail(1).unwrap(); // None / Err の場合panic
    println!("発見{}", v);

    Ok(())
}

fn fnvector() {
    println!("---[fnvector]---");
    let mut i32_vec = Vec::<i32>::new();
    i32_vec.push(1);
    i32_vec.push(2);
    i32_vec.push(3);

    let mut float_vec = Vec::new(); // 型推論
    float_vec.push(1.3);
    float_vec.push(2.3);
    float_vec.push(3.4);

    let string_vec = vec![String::from("Hello"), String::from("World")];
    for word in string_vec.iter() {
        println!("{}", word);
    }
}

fn main() -> Result<(), String> { // mainがErrを返すこともある
    generics();
    fnoption();
    fnresult();
    fnvector();
    simpleresult()?;
    panic();
    Ok(())
}
