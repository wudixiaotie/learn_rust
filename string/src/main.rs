fn main() {
    let s = "Hello, world!"; //string &'static str
    let mut s1 = s.to_string();//String
    s1.push_str("asdf");

    println!("S:{}, S1:{}", s, s1);

    let s2 = &s;//&str
    println!("s2:{}", s2);

    let s3 = s2.to_string();

    println!("Hello, world! addr :{:p}", &"Hello, world!");
    println!("s addr :{:p}", s);
    println!("s1 addr :{:p}", &s1);
    println!("s2 addr :{:p}", &s2);
    println!("s3 addr :{:p}", &s3);
}
