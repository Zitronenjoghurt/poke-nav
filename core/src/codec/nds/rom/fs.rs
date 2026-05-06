use crate::codec::common::rom::RomReadError;
use crate::codec::nds::rom::fat::Fat;
use crate::codec::nds::rom::fnt::{FntMainEntry, FntSubEntry};
use crate::codec::nds::rom::NdsRomReadError;
use binrw::{BinRead, BinReaderExt};
use std::io::{Read, Seek, SeekFrom};

/// Source: https://problemkaputt.de/gbatek-ds-cartridge-nitrorom-and-nitroarc-file-systems.htm
pub struct NdsFileSystem {
    pub directories: Vec<NdsDirectory>,
    pub files: Vec<NdsFile>,
}

impl NdsFileSystem {
    pub fn read<R: Read + Seek>(
        reader: &mut R,
        fat_offset: u32,
        fat_size: u32,
        fnt_offset: u32,
    ) -> Result<Self, RomReadError> {
        reader.seek(SeekFrom::Start(fat_offset as u64))?;
        let fat = Fat::read_args(reader, (fat_size,))?;

        let mut directories = Vec::new();
        let mut files = Vec::new();

        reader.seek(SeekFrom::Start(fnt_offset as u64))?;
        let root_entry = FntMainEntry::read(reader).map_err(|_| NdsRomReadError::FNTRead)?;
        let total_directories = root_entry.parent_id_or_count;

        directories.push(NdsDirectory {
            id: 0xF000,
            name: String::new(),
            parent_dir_id: 0xF000,
        });

        reader.seek(SeekFrom::Start(fnt_offset as u64))?;
        let main_table: Vec<FntMainEntry> = reader
            .read_le_args(binrw::VecArgs {
                count: total_directories as usize,
                inner: (),
            })
            .map_err(|_| NdsRomReadError::FNTRead)?;

        for (i, main_entry) in main_table.iter().enumerate() {
            let current_dir_id = 0xF000 + i as u16;
            let mut current_file_id = main_entry.first_file_id;

            reader.seek(SeekFrom::Start(
                (fnt_offset + main_entry.sub_table_offset) as u64,
            ))?;

            loop {
                let entry = FntSubEntry::read(reader).map_err(|_| NdsRomReadError::FNTRead)?;

                if entry.is_end() {
                    break;
                }

                let name = entry.name();

                if entry.is_dir() {
                    directories.push(NdsDirectory {
                        id: entry.sub_dir_id.unwrap_or(0),
                        name,
                        parent_dir_id: current_dir_id,
                    });
                } else {
                    let fat_entry = &fat.entries[current_file_id as usize];

                    let mut data = vec![0u8; fat_entry.size() as usize];
                    let restore_pos = reader.stream_position()?;

                    reader.seek(SeekFrom::Start(fat_entry.start_address as u64))?;
                    reader.read_exact(&mut data)?;

                    reader.seek(SeekFrom::Start(restore_pos))?;

                    files.push(NdsFile {
                        id: current_file_id,
                        name,
                        parent_dir_id: current_dir_id,
                        data,
                    });

                    current_file_id += 1;
                }
            }
        }

        Ok(Self { directories, files })
    }

    pub fn print_tree(&self) {
        println!("/");
        self.print_dir(0xF000, String::new());
    }

    fn print_dir(&self, current_dir_id: u16, prefix: String) {
        let mut child_dirs: Vec<&NdsDirectory> = self
            .directories
            .iter()
            .filter(|d| d.parent_dir_id == current_dir_id && d.id != current_dir_id)
            .collect();

        let mut child_files: Vec<&NdsFile> = self
            .files
            .iter()
            .filter(|f| f.parent_dir_id == current_dir_id)
            .collect();

        child_dirs.sort_by(|a, b| a.name.cmp(&b.name));
        child_files.sort_by(|a, b| a.name.cmp(&b.name));

        let total_dirs = child_dirs.len();
        let total_files = child_files.len();

        for (i, dir) in child_dirs.into_iter().enumerate() {
            let is_last = (i == total_dirs - 1) && total_files == 0;
            let marker = if is_last { "└── " } else { "├── " };

            println!("{}{}{}/", prefix, marker, dir.name);

            let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
            self.print_dir(dir.id, new_prefix);
        }

        for (i, file) in child_files.into_iter().enumerate() {
            let is_last = i == total_files - 1;
            let marker = if is_last { "└── " } else { "├── " };

            println!(
                "{}{}{} ({} bytes)",
                prefix,
                marker,
                file.name,
                file.data.len()
            );
        }
    }
}

pub struct NdsFile {
    pub id: u16,
    pub name: String,
    pub parent_dir_id: u16,
    pub data: Vec<u8>,
}

pub struct NdsDirectory {
    pub id: u16,
    pub name: String,
    pub parent_dir_id: u16,
}
