fn main() {
    let mut x = vec!["Hello", "World"];
    let y = &x[0];
    let z = 1;
    print!("{:?}", x);
    x.push("foo");
    let mut a = String::new("aaabbb");
    let b = &a;
    println!("a:{:?} b:{:?}", a, b);
    println!("{}", y);    
}
