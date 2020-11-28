#[derive(PartialEq)]
struct CmpPE {
    num: f32,
}

#[derive(PartialOrd, PartialEq)]
struct CmpPO {
    num: f32,
}

fn test() {
    println!("=========== CmpPE ===========");
    let a = CmpPE { num: 1_f32 };
    let b = CmpPE { num: 2_f32 };

    println!("a == b {}", a == b);
    println!("a != b {}", a != b);
    println!("a == a {}", a == a);

    println!("=========== CmpPO ===========");
    let a = CmpPO { num: 1_f32 };
    let b = CmpPO { num: 2_f32 };

    println!("a == b {}", a == b);
    println!("a != b {}", a != b);
    println!("a == a {}", a == a);
    println!("a < b {}", a <= b);
    println!("!(a < b) {}", !(a <= b));
    println!("!(a < b) {}", !(a == a));
}
