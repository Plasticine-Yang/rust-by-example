#[allow(dead_code)]
pub fn demo1() {
    // #[derive(Debug)] derive the `fmt::Debug` implementation for `Deep`.
    // Now `Structure` is printable.
    #[derive(Debug)]
    struct Structure(i32);

    #[derive(Debug)]
    struct Deep(Structure);

    // Printing with `{:?}` is similar to with `{}`
    println!("{:?} months in a year.", 12);
    println!("{} months in a year.", 12);

    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3)); // Now Structure(3) will print!

    // `Deep` is printable too!
    println!("Now {:?} will print!", Deep(Structure(7))) // Now Deep(Structure(7)) will print!
}

#[allow(dead_code)]
pub fn demo2() {
    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    let person = Person {
        name: "Plasticine",
        age: 21,
    };

    // Pretty print
    println!("{:#?}", person);
}
