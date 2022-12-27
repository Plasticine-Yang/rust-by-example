use std::fmt;

#[allow(dead_code)]
pub fn demo() {
    struct List(Vec<i32>);

    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            // Extract the value using tuple indexing, and create a reference to `vec`.
            let vec = &self.0;

            write!(f, "[")?;

            for (count, v) in vec.iter().enumerate() {
                if count != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", v)?;
            }

            write!(f, "]")
        }
    }

    fn main() {
        let v = List(vec![1, 2, 3]);
        println!("{}", v);
    }

    main();
}
