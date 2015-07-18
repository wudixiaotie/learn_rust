fn main() {
    let mut a = 34;
    println!("a's addr:{:p}", &a);
    let p = &mut a;
    println!("p's addr:{:p}", &p);
    println!("out test_pointer p's addr:{:p}", &test_pointer(p));

    let b = 1;
    let bp = &b;
    println!("in main bp's addr:{:p}", p);
    pointer_addr(bp);
    pointer_addr1(bp);
}

fn test_pointer(p: &mut i32) -> &mut i32 {
    *p = 32;
    println!("in test_pointer p's addr:{:p}", &p);
    p
}

fn pointer_addr(p: &i32) -> &i32 {
    println!("in pointer_addr p's addr:{:p}", p);
    p
}

fn pointer_addr1(p: &i32) -> &i32 {
    println!("in pointer_addr1 p's addr:{:p}", p);
    p
}