use luhn::is_valid;

fn main() {
    println!("is_valid? {:?}", is_valid("4539 1488 0343 6467".to_string()));
}
