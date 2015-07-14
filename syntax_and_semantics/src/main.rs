fn main() {
    let a = true;
    let two_hearts: char = '💕';
    let f = 12.4_f64;
    println!("Hello, world!{}", f);
    let ar = [1; 3];
    let s_ar1 = &ar;
    let s_ar2 = &ar[..];
    for n in s_ar1 {
        println!("{},", n);
    }
    println!("{}", s_ar1 == s_ar2);
    let mut x = (1, 2); // x: (i32, i32)
    let y = (2, 3); // y: (i32, i32)

    x = y;
    println!("{:?}", x);

    fn foo(x: i32) -> i32 { x }
    let x1: fn(i32) -> i32 = foo;
    println!("{:?}", x1(3));

    if (5 == 5 && 1 == 1) || "a" == "b" {
        println!("true");
    } else {
        println!("false");
    }

    for x in 0..10 {
        println!("{:?}", x);
    }

    let mut w1 = 0;
    let mut done = false;
    println!("this is while");
    while !done {
        w1 += 1;
        println!("{}", w1);

        if w1 % 5  == 0 {
            done = true;
        }
    }
}
