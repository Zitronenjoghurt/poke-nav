use poke_nav::codec::common::rom::RawRom;
use poke_nav::codec::nds::formats::ParsedNdsFile;
use poke_nav::codec::nds::fs::file::NdsFileData;
use poke_nav::codec::nds::games::hgss::HgSsKnownFile;
use std::path::PathBuf;

fn main() {
    let rom_path = PathBuf::from("./test/hgss.nds");
    let file = std::fs::File::open(rom_path).unwrap();
    let mut reader = std::io::BufReader::new(file);

    let raw_rom = RawRom::read(&mut reader).unwrap();
    match raw_rom {
        RawRom::Nds(nds) => {
            let file = nds.fs.get_file(HgSsKnownFile::LandData).unwrap();
            match &file.data {
                NdsFileData::Parsed(parsed) => match parsed {
                    ParsedNdsFile::Narc(narc) => {
                        narc.fs.print_tree();
                    }
                },
                NdsFileData::Raw(_) => {}
            }
        }
    }
}
