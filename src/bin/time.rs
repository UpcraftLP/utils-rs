fn main() {
    let start = std::time::Instant::now();
    let duration = start.elapsed();
    println!("{:?}", duration.as_secs());
}
