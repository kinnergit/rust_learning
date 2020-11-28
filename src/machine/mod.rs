pub fn run() {
    println!("is little endian: {}", is_little_endian());
    println!("is big endian: {}", is_big_endian());
}

pub fn is_little_endian() -> bool {
    union U {
        s: u16,
        b: u8,
    };

    let u = U { s: 0xff00 };

    unsafe { u.b == 0x00 }
}

pub fn is_big_endian() -> bool {
    union U {
        s: u16,
        b: u8,
    };

    let u = U { s: 0xff00 };

    unsafe { u.b == 0xff }
}
