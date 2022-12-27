use std::fmt::Display;

#[allow(dead_code)]
pub fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of tuple to variables
    let (int_param, bool_param) = pair;

    // equal to `return (bool_param, int_param);`
    (bool_param, int_param)
}

#[allow(dead_code)]
pub fn activity() {
    struct Matrix(f32, f32, f32, f32);

    // activity 1
    impl Display for Matrix {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({} {})\n({} {})", self.0, self.1, self.2, self.3)
        }
    }

    // activity 2
    fn transpose(matrix: Matrix) -> Matrix {
        Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
    }

    fn main() {
        let matrix = Matrix(1.1, 1.2, 2.1, 2.2);

        println!("{}", matrix);

        println!("Matrix:\n{}", matrix);
        println!("Transpose:\n{}", transpose(matrix));
    }

    main();
}
