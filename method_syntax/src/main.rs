fn main() {
    println!("Hello, world!");
    let c1 = Circle { x: 3.4, y: 4.1, radius: 2.3 };
    c1.reference();
    let mut c2 = Circle { x: 3.4, y: 4.1, radius: 2.3 };
    c2.mutable_reference();
    let c3 = Circle { x: 3.4, y: 4.1, radius: 2.3 };
    c3.takes_ownership();

    println!("{}", c1.area());

    let d = c1.grow(2.0).area();
    println!("{}", d);
}
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn reference(&self) {
       println!("taking self by reference!");
    }

    fn mutable_reference(&mut self) {
       println!("taking self by mutable reference!");
    }

    fn takes_ownership(self) {
       println!("taking ownership of self!");
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }

    fn grow(&self, increment: f64) -> Circle {
        Circle { x: self.x, y: self.y, radius: self.radius + increment }
    }
}