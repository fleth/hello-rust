fn main() {
    fnif();
    fnloop();
    fnwhile();
    fnfor();
    fnmatch();
    block_return();
}

fn fnif() {
    println!("---[if]---");
    let x = 42;
    if x < 42 {
        println!("42より小さい");
    } else if x == 42 {
        println!("42に等しい");
    } else {
        println!("42より大きい");
    }
}

fn fnloop() {
    println!("---[loop]---");
    let mut x = 0;
    let v = loop {
        x += 1;
        if x == 42 {
            break "42を発見"; //値も返せる
        }
    };
    println!("{} {}", x, v);
}

fn fnwhile() {
    println!("---[while]---");
    let mut x = 0;
    while x != 42 {
        x += 1;
    }
}

fn fnfor() {
    println!("---[for]---");
    for x in 0..5 { // 0~4
        println!("{}", x);
    }

    for x in 0..=5 { // 0~5
        println!("{}", x);
    }
}

fn fnmatch() {
    println!("---[match]---");
    let x = 42;

    match x {
        0 => {
            println!("found zero");
        }
        1 | 2 => {
            println!("found 1 or 2 !");
        }
        3..=9 => {
            println!("found a number 3 to 9 inclusively");
        }
        matched_num @ 10..=100 => {
            println!("found {} number between 10 to 100!", matched_num);
        }
        _ => {
            println!("found something else!");
        }
    }
}

fn block_return() {
    println!("---[block return]---");
    let x = 42;
    let v = if x < 42 { -1 } else { 1 };
    println!("if より: {}", v);

    let food = "ハンバーガー";
    let result = match food {
        "ホットドッグ" => "ホットドッグです",
        _ => "ホットドッグではありません"
    };
    println!("食品の識別: {}", result);

    let v = {
        let a = 1;
        let b = 2;
        a + b
    };
    println!("ブロックより:{}", v);
}
