use poke_nav::codec::common::rom::RawRom;
use std::path::PathBuf;

fn main() {
    let rom_path = PathBuf::from("./test/hgss.nds");
    let file = std::fs::File::open(rom_path).unwrap();
    let mut reader = std::io::BufReader::new(file);

    let raw_rom = RawRom::read(&mut reader).unwrap();
    match raw_rom {
        RawRom::Nds(nds) => println!("{}", nds.header.game_title),
    }
}
