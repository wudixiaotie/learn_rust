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

    println!("{}", Circle::not_self());

    let c4 = CircleBuilder::new()
                .x(1.0)
                .y(2.0)
                .radius(3.6)
                .finalize();

    println!("area is {}", c4.area());
    println!("x: {}", c4.x);
    println!("y: {}", c4.y);
}
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn not_self() -> i32 {
        32
    }

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

struct CircleBuilder {
    x: f64,
    y: f64,
    radius: f64,
}

impl CircleBuilder {
    fn new() -> CircleBuilder {
        CircleBuilder { x: 0.0, y: 0.0, radius: 1.0, }
    }

    fn x(&mut self, coordinate: f64) -> &mut CircleBuilder {
        self.x = coordinate;
        self
    }

    fn y(&mut self, coordinate: f64) -> &mut CircleBuilder {
        self.y = coordinate;
        self
    }

    fn radius(&mut self, coordinate: f64) -> &mut CircleBuilder {
        self.radius = coordinate;
        self
    }

    fn finalize(&self) -> Circle {
        Circle { x: self.x, y: self.y, radius: self.radius }
    }
}