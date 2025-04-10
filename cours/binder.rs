fn main () {

    let i = 0i32;

    match i {
        x @ 10..=100 => println! {"{} est entre 10 et 100 (inclus)", x},
        x => println! {"{} n'est pas entre 10 et 100", x},
    }
}