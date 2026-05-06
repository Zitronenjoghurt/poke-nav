use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal,
};
use poke_nav::codec::common::fmt::format_bytes;
use poke_nav::codec::common::rom::RawRom;
use poke_nav::codec::nds::formats::hgss_map::HgSsMap;
use poke_nav::codec::nds::formats::ParsedNdsFile;
use poke_nav::codec::nds::fs::file::NdsFileData;
use poke_nav::codec::nds::games::hgss::HgSsKnownFile;
use std::path::PathBuf;

fn main() {
    let rom_path = PathBuf::from("./test/hgss.nds");
    let file = std::fs::File::open(rom_path).unwrap();
    let mut reader = std::io::BufReader::new(file);

    let raw_rom = RawRom::read(&mut reader).unwrap();
    let RawRom::Nds(nds) = raw_rom;

    let file = nds.fs.get_file(HgSsKnownFile::LandData).unwrap();
    let NdsFileData::Parsed(ParsedNdsFile::Narc(narc)) = &file.data else {
        eprintln!("not a parsed NARC");
        return;
    };

    let total = narc.fs.files.len();
    let mut index: usize = 0;

    terminal::enable_raw_mode().unwrap();

    loop {
        print!("\x1B[2J\x1B[H");

        let data = narc.fs.files.get(index).unwrap().data.raw().unwrap();
        let mut cursor = std::io::Cursor::new(data);

        match HgSsMap::read(&mut cursor) {
            Ok(map) => {
                println!(
                    "map {index}/{total}  |  size {}  |  ←/→ navigate  |  q quit\r",
                    format_bytes(data.len())
                );
                println!(
                    "objects: {}  |  bdhc: {} bytes\r",
                    map.objects.len(),
                    map.bdhc.len()
                );
                println!("\r");
                map.permissions.print_grid();
            }
            Err(e) => {
                println!("map {index}/{total}  |  failed to parse: {e}\r");
            }
        }

        loop {
            if let Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
                match code {
                    KeyCode::Right | KeyCode::Down => {
                        index = (index + 1) % total;
                        break;
                    }
                    KeyCode::Left | KeyCode::Up => {
                        index = (index + total - 1) % total;
                        break;
                    }
                    KeyCode::Char('q') | KeyCode::Esc => {
                        terminal::disable_raw_mode().unwrap();
                        return;
                    }
                    _ => {}
                }
            }
        }
    }
}
