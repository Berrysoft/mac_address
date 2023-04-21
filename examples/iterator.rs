use mac_address2::MacAddressIterator;

fn main() {
    for addr in MacAddressIterator::new().unwrap() {
        println!("{}", addr);
    }
}
