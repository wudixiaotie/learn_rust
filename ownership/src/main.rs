mod hello;
mod aaa;

fn main() {
    hello::print_hello();
    aaa::test();
    let y = &5; // this is the same as `let _y = 5; let y = &_y;`
    let f = aaa::Foo { x: &y };

    println!("{}", f.x);
}