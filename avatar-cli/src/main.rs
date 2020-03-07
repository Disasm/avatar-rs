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

    let mut v = interface.try_read32(0xE004_2000).unwrap();
    if v == 0 {
        v = interface.try_read32(0x40015800).unwrap();
    }
    println!("IDCODE: {:08x}", v);
}
