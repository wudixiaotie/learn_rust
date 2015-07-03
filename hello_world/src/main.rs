fn main() {
    let mut x = vec!["Hello", "World"];
    let y = &x[0].clone();
    let z = 1;
    print!("{:?}", x);
    x.push("foo");
}