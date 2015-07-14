enum Fuck {
    Times(i32),
    Object { name: String },
}

fn main() {
    let x: Fuck = Fuck::Times(30);
    let y: Fuck = Fuck::Object { name: "liuyang".to_string() };
    // println!("{}, {}", get_type(x), get_type(y));
    get_type(x);
    get_type(y);
}

fn get_type(f: Fuck) {
    match f {
        Fuck::Times(i32) => println!("this is Fuck::Times"),
        Fuck::Object { name: String } => println!("this is Fuck::Object {{ name: String }}"),
    };
}