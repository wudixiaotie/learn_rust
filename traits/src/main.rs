fn main() {
    let c = Circle { x: 2.3, y: 5.7, radius: 3.2 };
    let c1 = Circle { x: 2.0, y: 9.1, radius: 5.4 };
    let c2 = Circle { x: 2.1, y: 9.3, radius: 2.4 };
    let s = Square { x: -3.8, y: -2.4, side: 6.5 };
    print_area_position(c);
    print_area_position(s);
    print_area_position2(c1, c2);
    print_area(4);
}

trait HasArea {
    fn area(&self) -> f64;
}

trait HasPosition {
    fn position(&self) -> (f64, f64);
    fn baz(&self) { println!("We called baz.{:?}"); }
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

impl HasPosition for Circle {
    fn position(&self) -> (f64, f64) {
        (self.x, self.y)
    }
}

struct Square {
    x: f64,
    y: f64,
    side: f64,
}

impl HasArea for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

impl HasPosition for Square {
    fn position(&self) -> (f64, f64) {
        (self.x, self.y)
    }
}

fn print_area<T: HasArea>(sharp: T) {
    println!("This sharp has an area of {}", sharp.area());
}

fn print_area_position<T: HasArea + HasPosition>(sharp: T) {
    println!("This sharp has an area of {} and position is {:?}", sharp.area(), sharp.position());
    sharp.baz();
}

fn print_area_position2<T, K>(sharp: T, a: K)
    where T: HasArea + HasPosition,
          K: HasArea {
    println!("This sharp has an area of {} and position is {:?}, a is {:?}", sharp.area(), sharp.position(), a.area());
}

impl HasArea for i32 {
    fn area(&self) -> f64 {
        println!("this is silly");

        *self as f64
    }
}
