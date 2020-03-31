use avatar_probe_rs::open_probe;
use std::time::{Instant, Duration};
use std::thread;

fn main() {
    let interface = open_probe();

    let n = 5000;

    let t0 = Instant::now();
    for _ in 0..n {
        let _ = interface.read32(0x2000_0000);
    }
    let dt = t0.elapsed();
    let uspr = dt.as_micros() / (n as u128);
    let rps = 1_000_000 / uspr;
    println!("read32:  {} us per request, {} requests per second", uspr, rps);

    thread::sleep(Duration::from_millis(200));

    let t0 = Instant::now();
    for i in 0..n {
        let _ = interface.write32(0x2000_0000, i as u32);
    }
    let dt = t0.elapsed();
    let uspr = dt.as_micros() / (n as u128);
    let rps = 1_000_000 / uspr;
    println!("write32: {} us per request, {} requests per second", uspr, rps);

    thread::sleep(Duration::from_millis(200));

    let mut buf = [0; 32];
    let t0 = Instant::now();
    for _ in 0..n/buf.len() {
        let _ = interface.read_block32(0x2000_0000, &mut buf);
    }
    let dt = t0.elapsed();
    let uspr = dt.as_micros() / (n as u128);
    let rps = 1_000_000 / uspr;
    println!("read_block32:  {} us per request, {} requests per second", uspr, rps);

    thread::sleep(Duration::from_millis(200));

    let buf = [0xdeadbeef; 32];
    let t0 = Instant::now();
    for _ in 0..n/buf.len() {
        let _ = interface.write_block32(0x2000_0000, &buf);
    }
    let dt = t0.elapsed();
    let uspr = dt.as_micros() / (n as u128);
    let rps = 1_000_000 / uspr;
    println!("write_block32:  {} us per request, {} requests per second", uspr, rps);
}
