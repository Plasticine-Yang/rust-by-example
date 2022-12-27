use std::fmt;

#[allow(dead_code)]
pub fn demo1() {
    struct Structure(i32);

    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Plasticine -- {}", self.0)
        }
    }

    println!("{}", Structure(6)); // Plasticine -- 6
}

#[allow(dead_code)]
pub fn demo2() {
    #[derive(Debug)]
    struct MinMax(i64, i64);

    impl fmt::Display for MinMax {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.0, self.1)
        }
    }

    fn main() {
        let minmax = MinMax(6, 666);

        println!("Compare structures:");
        println!("Display: {}", minmax);
        println!("Debug: {:?}", minmax);

        let big_range = MinMax(-300, 300);
        let small_range = MinMax(-3, 3);

        println!(
            "The big range is {big} and the small is {small}",
            small = small_range,
            big = big_range
        );
    }

    main();
}

#[allow(dead_code)]
pub fn demo3() {
    // Define a structure where the fields are nameable for comparison.
    #[derive(Debug)]
    struct Point2D {
        x: f64,
        y: f64,
    }

    // Similarly, implement `fmt::Display` for `Point2D`
    impl fmt::Display for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "x: {}, y: {}", self.x, self.y)
        }
    }

    fn main() {
        let point = Point2D { x: 3.3, y: 7.2 };

        println!("Compare points:");
        println!("Display: {}", point);
        println!("Debug: {:?}", point);

        // This will not work.
        // Because `{:b}` requires `fmt::Binary` to be implemented.
        // println!("What does Point2D like in binary: {:b}?", point);
    }

    main();
}

#[allow(dead_code)]
pub fn activity() {
    #[derive(Debug)]
    struct Complex {
        real: f64,
        imag: f64,
    }

    impl fmt::Display for Complex {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} + {}i", self.real, self.imag)
        }
    }

    fn main() {
        let complex = Complex {
            real: 3.3,
            imag: 7.2,
        };

        println!("Display: {}", complex);
        println!("Debug: {:?}", complex);
    }

    main();
}
