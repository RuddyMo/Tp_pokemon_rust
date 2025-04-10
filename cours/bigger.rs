fn main() {
    let bigger = get_bigger(1, 2);
    println!("{}", bigger);
}

fn get_bigger(nb1: i32, nb2: i32) -> i32 {
    if nb1 > nb2 {
        nb1
    } else {
        nb2
    }
}