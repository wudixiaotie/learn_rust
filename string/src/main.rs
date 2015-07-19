fn main() {
    let s = "Hello, world!"; //string &'static str
    let mut s1 = s.to_string();//String
    s1.push_str("asdf");

    println!("S:{}, S1:{}", s, s1);

    let s2 = &s;//&str
    println!("s2:{}", s2);

    let s3 = s2.to_string();

    println!("s addr :{:p}", &s);
    println!("s1 addr :{:p}", &s1);
    println!("s2 addr :{:p}", &s2);
    println!("s3 addr :{:p}", &s3);

    println!("Hello, world! in main addr :{:p}", &"Hello, world!");
    println!("Hello, world! in main addr :{:p}", &"Hello, world!");
    ts();
    let s4 = "Hello, world!".to_string(); //string &'static str
    let s5 = "sb";
    let s6 = s4 + s5;
    println!("s6 addr:{}", s6);
}


fn ts() {
    println!("Hello, world! in ts addr :{:p}", &"Hello, world!");
}