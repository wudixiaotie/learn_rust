fn main() {
    let a = [1; 20];
    println!("{:?}", a);
    for i in 1..20 {
        println!("{}", i);
    }

    let p = &a;
    println!("{:p}", p);
    let pp = &p;
    println!("{:p}", pp);
}


