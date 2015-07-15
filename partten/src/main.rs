fn main() {
    let x = 5;

    match x {
        e @ 4 ... 5 => println!("1 ... 5, {}", e),
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let a = vec!["a", "b"];
    println!("{:?}", a);


    let name = "Steve".to_string();
    let x2 = Some(x);
    match x2 {
        ref a @ Some(_) => println!("{:?}", a),
        _ => {}
    }
    let x1: Option<Person> = Some(Person { name: Some(name) });
    match x1 {
        Some(Person { name: ref a @ Some(_), .. }) => println!("a addr ={:p}", &a),
        _ => {}
    }
    match x1 {
        Some(Person { name: a @ Some(_), .. }) => println!("ref a addr ={:p}", &a),
        _ => {}
    }




    let x3 = OptionalInt::Value(5);

    match x3 {
        OptionalInt::Value(i) if i > 4 => println!("Got an int bigger than 4!"),
        OptionalInt::Value(_) => println!("Got an int!"),
        OptionalInt::Missing => println!("No such luck."),
    }

    match 6 {
        ref r => println!("Got a reference to {}", r),
    }

    match 7 {
        ref mut mr => println!("Got a mutable reference to {}", mr),
    }

    let x5 = Point { x: 1, y: 2 };

    match x5 {
        Point { x: x, .. } => println!("x is {}", x),
    }

    match x5 {
        Point { x: x, y: y } => println!("x is {}, y is {}", x, y),
    }
}
#[derive(Debug)]
struct Person {
    name: Option<String>,
}

enum OptionalInt {
    Value(i32),
    Missing,
}

struct Point {
    x: i32,
    y: i32,
}