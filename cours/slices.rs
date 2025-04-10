fn main() {
    let mut v = Vec::new();

        v.push(2);
        v.push(1);
        v.push(0);

    println!("{:?}",v);
    let s = &v[1..];
    println!("{:?}", s);
}