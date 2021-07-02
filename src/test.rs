fn main() {
    for _ in 0..12 {
        std::thread::sleep(std::time::Duration::new(1, 0));
    }
}
