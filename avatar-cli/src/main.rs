use avatar_probe_rs::AvatarProbe;
use avatar_common::MemoryInterface;

fn main() {
    let mut interface = match AvatarProbe::open_any() {
        Ok(probe) => probe,
        Err(e) => {
            println!("Can't open probe: {:?}", e);
            return;
        }
    };

    let v = interface.read32(0xE004_2000).unwrap();
    println!("Value: {:08x}", v);
}
