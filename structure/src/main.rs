#![allow(dead_code)] // この行でコンパイラのwaringsメッセージを止めます。

struct SeaCreature {
    animal_type: String,
    name: String,
    arms: i32,
    legs: i32,
    weapon: String,
}

struct Location(i32, i32);

struct Marker; // unitlike

enum Species {
    Crab,
    Octopus,
    Fish,
    Clam
}

struct EnumSeaCreature {
    species: Species,
    name: String,
    arms: i32,
    legs: i32,
    weapon: String, //dead_code
}

enum PositionType { Acidic, Painful, Lethal }
enum Size { Big, Small }
enum Weapon {
    Claw(i32, Size),
    Position(PositionType),
    None
}

struct EnumSeaCreature2 {
    species: Species,
    name: String,
    arms: i32,
    legs: i32,
    weapon: Weapon,
}

fn main() {
    method();
    memory();
    tuple();
    unitlike();
    fnenum();
}

fn method() {
    // スタティックメソッド
    let s = String::from("Hello world");
    // インスタンスメソッド
    println!("{} is {} characters long.", s, s.len());
}

fn memory() {
    //データメモリ 固定長 static
    //スタックメモリ 関数内で宣言された変数
    //ヒープメモリ 実行時に作られるデータ

    let ferris = SeaCreature {
        animal_type: String::from("crub"), // スタックに入るがヒープに入るデータの参照が入る
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: String::from("claw"),
    };

    let sarah = SeaCreature {
        animal_type: String::from("octopus"),
        name: String::from("Sarah"),
        arms: 8,
        legs: 0,
        weapon: String::from("none")
    };

    println!(
        "{} is a {}. They have {} arms, {} legs, and a {} weapon",
        ferris.name, ferris.animal_type, ferris.arms, ferris.legs, ferris.weapon
    );

    println!(
        "{} is a {}. They have {} arms, {} legs, and a {} weapon",
        sarah.name, sarah.animal_type, sarah.arms, sarah.legs, sarah.weapon
    );
}

fn tuple() {
    let loc = Location(42, 32);
    println!("{}, {}", loc.0, loc.1);
}

fn unitlike() { // 空構造体
    let _m = Marker;
}

fn fnenum() {
    let ferris = EnumSeaCreature {
        species: Species::Crab,
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: String::from("claw"),
    };

    match ferris.species {
        Species::Crab => println!("{} is a crab", ferris.name),
        Species::Octopus => println!("{} is a octopus", ferris.name),
        Species::Fish => println!("{} is a fish", ferris.name),
        Species::Clam => println!("{} is a clam", ferris.name),
    }
}

fn fnenum2() {
    let ferris = EnumSeaCreature2 {
        species: Species::Crab,
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: Weapon::Claw(2, Size::Small),
    };

    match ferris.species {
        Species::Crab => {
            match ferris.weapon {
                Weapon::Claw(num_claws, size) => {
                    let size_description = match size {
                        Size::Big => "big",
                        Size::Small => "small"
                    };
                    println!("ferris is a crab with {} {} claws", num_claws, size_description)
                },
                _ => println!("ferris is a crab with some other weapon")
            }
        },
        _ => println!("ferris is some other animal")
    }
}
