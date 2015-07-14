struct SquareMeters(i64);
fn main() {
    let m = (60,);
    let (a,) = m;
    let SquareMeters(room_size) = SquareMeters(30);
    println!("{}, {}", room_size, a);
}
