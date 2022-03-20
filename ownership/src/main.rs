struct Foo {
    x: i32,
}

struct Bar {
    foo: Foo,
}

fn fndrop() {
    println!("---[fndrop]---");
    let foo_a = Foo { x: 42 };
    let foo_b = Foo { x: 13 };

    println!("{}", foo_a.x);
    println!("{}", foo_b.x);
    // foo_b はここでドロップ
    // foo_a はここでドロップ
}

fn fndrop_struct() {
    println!("---[fndrop_struct]---");
    let bar = Bar{foo: Foo {x : 42}};
    println!("{}", bar.foo.x);
    // bar -> bar.foo の順にドロップ
}

fn ownership() {
    println!("---[ownership]---");
    let foo = Foo { x: 42 };
    // fooは所有者
    println!("{}", foo.x);
}

fn do_something(f: Foo) {
    println!("{}", f.x);
}

fn do_something2() -> Foo {
    Foo { x: 42 }
    // 所有権は外に移動
}

fn fnmove() {
    println!("---[fnmove]---");
    let foo = Foo { x: 42 };
    // foo の所有権が do_something に移動
    do_something(foo);
    // fooは使えなくなる

    let foo2 = do_something2();
    // foo2は所有者になる
    println!("{}", foo2.x);
    // foo2はドロップ
}

fn fnreference() {
    println!("---[fnreference]---");
    let foo = Foo { x: 42 };
    let f = &foo;
    println!("{}", f.x)
    // f -> foo の順でドロップ
}

fn fnborrowing() {
    println!("---[fnborrowing]---");
    let mut foo = Foo { x: 42 };
    let f = &mut foo;

    // fooは借用されているため移動・変更できない
    // foo.x = 13; // エラー
    // do_something(foo); // エラー

    f.x = 13;
    // fはこの先利用されないためここでドロップ

    println!("{}", foo.x);

    // fがドロップされているため移動・変更可能
    foo.x = 7;
    do_something(foo);
}

fn fndereference() {
    println!("---[fndereference]---");
    let mut foo = 42;
    let f = &mut foo;
    let bar = *f;
    *f = 13;

    println!("{}", bar);
    println!("{}", foo);
}

fn do_something3(f: &mut Foo) {
    f.x += 1;
    // fへの可変名参照はここでドロップ
}

fn fnreference2() {
    println!("---[fnreference2]---");
    let mut foo = Foo { x: 42};
    do_something3(&mut foo);
    do_something3(&mut foo); //可変な参照はドロップされているので別の参照が作れる
    // fooはここでドロップ
}

fn do_something4(a: &Foo) -> &i32 {
    return &a.x;
}

fn fnreference3() {
    println!("---[fnreference3]---");
    let mut foo = Foo { x: 42 };
    let x = &mut foo.x;
    *x = 13;
    // x はここででドロップされるため、不変な参照が作成可能
    let y = do_something4(&foo);
    println!("{}", y);
    // y -> foo ドロップ
}

fn do_something5<'a, 'b>(foo_a: &'a Foo, foo_b: &'b Foo) -> &'b i32 {
    println!("{}", foo_a.x);
    println!("{}", foo_b.x);
    return &foo_b.x;
}

fn fnlifetime() {
    println!("---[fnlifetime]---");
    let foo_a = Foo {x: 42};
    let foo_b = Foo {x: 12};
    let x = do_something5(&foo_a, &foo_b);
    // foo_b は xと同じタイミングでドロップ
    // foo_aはここでドロップ
    println!("{}", x);
    // x -> foo_b ドロップ
}

static PI: f64 = 3.1415;

fn fnstaticlifetime() {
    println!("---[fnstaticlifetime]---");
    static mut SECRET: &'static str = "swordfis";

    let msg: &'static str = "Hello World!";
    let p: &'static f64 = &PI;
    println!("{} {}", msg, p);

    // ルールを破ることができるが明示する必要がある
    unsafe {
        // 'static なのでSECRETに代入可能
        SECRET = "abracadabra";
        println!("{}", SECRET);
    }
}

struct Hoge<'c> {
    i:&'c i32
}

fn fnstructlifetime() {
    println!("---[fnstructlifetime]---");
    let x = 42;
    let hoge = Hoge {
        i: &x
    };
    println!("{}", hoge.i);
}

fn main() {
    ownership();
    fndrop();
    fndrop_struct();
    fnmove();
    fnreference();
    fnborrowing();
    fndereference();
    fnreference2();
    fnreference3();
    fnlifetime();
    fnstaticlifetime();
    fnstructlifetime();
}
