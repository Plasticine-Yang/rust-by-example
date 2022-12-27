mod primitives;

fn main() {
    let (bool_res, int_res) = primitives::tuples::reverse((666, true));
    println!("bool_res: {}, int_res: {}", bool_res, int_res);

    primitives::tuples::activity();
}
