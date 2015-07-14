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
    let mut x2 = Some(x);
    match x2 {
        ref a @ Some(_) => println!("{:?}", a),
        _ => {}
    }
    let mut x1: Option<Person> = Some(Person { name: Some(name) });
    match x1 {
        Some(Person { name: ref a @ Some(_), .. }) => println!("{:?}", a),
        _ => {}
    }
}
#[derive(Debug)]
struct Person {
    name: Option<String>,
}