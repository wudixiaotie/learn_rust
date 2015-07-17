fn main() {
    let a = [1,2,3,4,5,6,7];
    for i in &a {
        println!("{}", i);
    }

    let x = 1;
    let p = &x;
    println!("x addr:{:p}, *p addr:{:p}, p addr:{:p}", &x, &*p, &p);

    let mut v = vec![1,2,3,4,5,6,7];
    println!("v: {:?}", v);
    println!("&mut v: {:?}", &mut v);
    println!("&v: {:?}", &v);

    let mut v0 = 1;
    println!("v0: {:?}", v0);
    println!("&mut v0: {:?}", &mut v0);
    println!("&v0: {:?}", &v0);

    let mut v1 = &1;
    println!("v1: {:?}", v1);
    println!("&mut v1: {:?}", &mut v1);
    println!("&v1: {:?}", &v1);


    for i in &v {
        println!("A reference to {}", i);
    }

    for i in &mut v {
        *i += 1;
        println!("A mutable reference to {}", i);
    }

    for i in v {
        // i += 1;
        println!("Take ownership of the vector and its element {}", i);
    }
}


