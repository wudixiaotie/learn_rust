struct SquareMeters(i64);
struct Room {
    square: SquareMeters,
}
fn main() {
    let m = (60,);
    let (a,) = m;
    let SquareMeters(room_size) = SquareMeters(30);
    println!("{}, {}", room_size, a);

    let room1 = Room { square: SquareMeters(133) };
    let SquareMeters(x) = room1.square;
    println!("My room1's square is {} square meters.", x);
    let b = "bbbb";
    let c = (b,);
    let (d,) = c;
    println!("b:{} b addr:{:p} c:{} c addr:{:p}", b, &b, c.0, &c);
    println!("b:{} b addr:{:p} c:{} c.0 addr:{:p} d:{} d addr:{:p}", b, &b, c.0, &(c.0), d, &d);
}
