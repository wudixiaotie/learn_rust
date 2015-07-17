fn main() {
    let mut a = 34;
    println!("a's addr:{:p}", &a);
    let p = &mut a;
    println!("p's addr:{:p}", &p);
    println!("out test_pointer p's addr:{:p}", &test_pointer(p));
}

fn test_pointer(p: &mut i32) -> &mut i32 {
    *p = 32;
    println!("in test_pointer p's addr:{:p}", &p);
    p
}