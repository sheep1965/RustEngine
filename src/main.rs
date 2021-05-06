
fn main() {
    use std::{thread, time};
    println!("Hello World!");
    let ten_millis = time::Duration::from_millis(1000);
    let now = time::Instant::now();
    thread::sleep(ten_millis);
    assert!(now.elapsed() >= ten_millis);
 }