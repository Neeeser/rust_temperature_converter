fn main() {
    println!("In fahren {}",cel_to_fah(32.0));
}


fn cel_to_fah(cel : f32) -> f32{
    1.8 * cel + 32.0
}