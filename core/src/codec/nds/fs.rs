use crate::codec::common::fmt::format_bytes;
use crate::codec::common::rom::RomReadError;
use crate::codec::nds::fs::dir::NdsDirectory;
use crate::codec::nds::fs::file::{NdsFile, NdsFileData};
use crate::codec::nds::fs::path::NdsPath;
use crate::codec::nds::rom::fat::{Fat, FatEntry};
use crate::codec::nds::rom::fnt::{FntMainEntry, FntSubEntry};
use crate::codec::nds::rom::NdsRomReadError;
use binrw::{BinRead, BinReaderExt};
use std::io::{Read, Seek, SeekFrom};

pub mod dir;
pub mod file;
pub mod path;

pub const ROOT_DIR_ID: u16 = 0xF000;

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
        Self::read_tables(reader, &fat.entries, fnt_offset as u64, 0)
    }

    pub fn read_tables<R: Read + Seek>(
        reader: &mut R,
        fat_entries: &[FatEntry],
        fnt_base: u64,
        img_base: u64,
    ) -> Result<Self, RomReadError> {
        let mut directories = Vec::new();
        let mut files = Vec::new();

        reader.seek(SeekFrom::Start(fnt_base))?;
        let root_entry = FntMainEntry::read(reader).map_err(|_| NdsRomReadError::FNTRead)?;
        let total_directories = root_entry.parent_id_or_count;

        directories.push(NdsDirectory {
            id: ROOT_DIR_ID,
            name: String::new(),
            parent_dir_id: ROOT_DIR_ID,
        });

        reader.seek(SeekFrom::Start(fnt_base))?;
        let main_table: Vec<FntMainEntry> = reader
            .read_le_args(binrw::VecArgs {
                count: total_directories as usize,
                inner: (),
            })
            .map_err(|_| NdsRomReadError::FNTRead)?;

        for (i, main_entry) in main_table.iter().enumerate() {
            let current_dir_id = ROOT_DIR_ID + i as u16;
            let mut current_file_id = main_entry.first_file_id;

            reader.seek(SeekFrom::Start(
                fnt_base + main_entry.sub_table_offset as u64,
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
                    let fat_entry = &fat_entries[current_file_id as usize];

                    let mut raw = vec![0u8; fat_entry.size() as usize];
                    let restore_pos = reader.stream_position()?;

                    reader.seek(SeekFrom::Start(img_base + fat_entry.start_address as u64))?;
                    reader.read_exact(&mut raw)?;
                    reader.seek(SeekFrom::Start(restore_pos))?;

                    let size = raw.len();
                    let data = NdsFileData::new(raw)?;

                    files.push(NdsFile {
                        id: current_file_id,
                        name,
                        parent_dir_id: current_dir_id,
                        size,
                        data,
                    });

                    current_file_id += 1;
                }
            }
        }

        let claimed: std::collections::HashSet<u16> = files.iter().map(|f| f.id).collect();

        for (idx, fat_entry) in fat_entries.iter().enumerate() {
            let file_id = idx as u16;
            if claimed.contains(&file_id) || fat_entry.is_unused() {
                continue;
            }

            let mut raw = vec![0u8; fat_entry.size() as usize];
            let restore_pos = reader.stream_position()?;

            reader.seek(SeekFrom::Start(img_base + fat_entry.start_address as u64))?;
            reader.read_exact(&mut raw)?;
            reader.seek(SeekFrom::Start(restore_pos))?;

            let size = raw.len();
            let data = NdsFileData::new(raw)?;

            files.push(NdsFile {
                id: file_id,
                name: format!("{:04}", idx),
                parent_dir_id: ROOT_DIR_ID,
                size,
                data,
            });
        }

        Ok(Self { directories, files })
    }

    pub fn print_tree(&self) {
        println!("/");
        self.print_dir(ROOT_DIR_ID, String::new());
    }

    fn print_dir(&self, current_dir_id: u16, prefix: String) {
        let mut child_dirs: Vec<&NdsDirectory> = self.child_dirs(current_dir_id).collect();
        let mut child_files: Vec<&NdsFile> = self.child_files(current_dir_id).collect();

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
                "{}{}{} {}",
                prefix,
                marker,
                file.name,
                format_bytes(file.size)
            );
        }
    }

    pub fn get_entry(&self, path: impl Into<NdsPath>) -> Option<NdsFileSystemEntry<'_>> {
        let path = path.into();
        if path.is_empty() {
            return Some(NdsFileSystemEntry::Directory(&self.directories[0]));
        }
        let components: Vec<&str> = path.components().collect();
        self.resolve(&components)
    }

    fn resolve(&self, components: &[&str]) -> Option<NdsFileSystemEntry<'_>> {
        let mut current_dir_id = ROOT_DIR_ID;

        for (i, &name) in components.iter().enumerate() {
            let rest = &components[i + 1..];

            if let Some(file) = self.file_in(current_dir_id, name) {
                return if rest.is_empty() {
                    Some(NdsFileSystemEntry::File(file))
                } else {
                    file.data.nested_fs()?.resolve(rest)
                };
            }

            let dir = self.dir_in(current_dir_id, name)?;
            if rest.is_empty() {
                return Some(NdsFileSystemEntry::Directory(dir));
            }
            current_dir_id = dir.id;
        }

        None
    }

    fn file_in(&self, dir_id: u16, name: &str) -> Option<&NdsFile> {
        self.files
            .iter()
            .find(|f| f.parent_dir_id == dir_id && f.name == name)
    }

    fn dir_in(&self, dir_id: u16, name: &str) -> Option<&NdsDirectory> {
        self.directories
            .iter()
            .find(|d| d.parent_dir_id == dir_id && d.name == name)
    }

    pub fn get_file(&self, path: impl Into<NdsPath>) -> Option<&NdsFile> {
        match self.get_entry(path) {
            Some(NdsFileSystemEntry::File(f)) => Some(f),
            _ => None,
        }
    }

    pub fn get_dir(&self, path: impl Into<NdsPath>) -> Option<&NdsDirectory> {
        match self.get_entry(path) {
            Some(NdsFileSystemEntry::Directory(d)) => Some(d),
            _ => None,
        }
    }

    pub fn child_dirs(&self, dir_id: u16) -> impl Iterator<Item = &NdsDirectory> {
        self.directories
            .iter()
            .filter(move |d| d.parent_dir_id == dir_id && d.id != dir_id)
    }

    pub fn child_files(&self, dir_id: u16) -> impl Iterator<Item = &NdsFile> {
        self.files.iter().filter(move |f| f.parent_dir_id == dir_id)
    }
}

// Zipping
#[cfg(feature = "zip")]
impl NdsFileSystem {
    pub fn to_zip(&self, expand_nested: bool) -> Result<Vec<u8>, zip::result::ZipError> {
        self.zip_dir_by_id(ROOT_DIR_ID, expand_nested)
    }

    pub fn zip_dir_by_path(
        &self,
        path: impl Into<NdsPath>,
        expand_nested: bool,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let path = path.into();
        let dir = self
            .get_dir(path.clone())
            .ok_or_else(|| format!("Directory not found: {}", path))?;
        self.zip_dir_by_id(dir.id, expand_nested)
            .map_err(|e| e.into())
    }

    pub fn zip_dir_by_id(
        &self,
        dir_id: u16,
        expand_nested: bool,
    ) -> Result<Vec<u8>, zip::result::ZipError> {
        let opts = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);
        let but = std::io::Cursor::new(Vec::new());
        let mut zip = zip::ZipWriter::new(but);

        self.write_dir_to_zip(&mut zip, dir_id, String::new(), &opts, expand_nested)?;

        let cursor = zip.finish()?;
        Ok(cursor.into_inner())
    }

    fn write_dir_to_zip<W: std::io::Write + Seek>(
        &self,
        zip: &mut zip::ZipWriter<W>,
        dir_id: u16,
        prefix: String,
        opts: &zip::write::SimpleFileOptions,
        expand_nested: bool,
    ) -> Result<(), zip::result::ZipError> {
        use std::io::Write;

        for file in self.child_files(dir_id) {
            let path = if prefix.is_empty() {
                file.name.clone()
            } else {
                format!("{}/{}", prefix, file.name)
            };

            if expand_nested && let Some(nested) = file.data.nested_fs() {
                let narc_dir = if prefix.is_empty() {
                    file.name.clone()
                } else {
                    format!("{}/{}", prefix, file.name)
                };
                zip.add_directory(&narc_dir, *opts)?;
                nested.write_dir_to_zip(zip, ROOT_DIR_ID, narc_dir, opts, expand_nested)?;
            } else {
                let raw = file.data.raw()?;
                zip.start_file(&path, *opts)?;
                zip.write_all(&raw)?;
            }
        }

        for dir in self.child_dirs(dir_id) {
            let path = if prefix.is_empty() {
                dir.name.clone()
            } else {
                format!("{}/{}", prefix, dir.name)
            };

            zip.add_directory(&path, *opts)?;

            self.write_dir_to_zip(zip, dir.id, path, opts, expand_nested)?;
        }

        Ok(())
    }
}

pub enum NdsFileSystemEntry<'a> {
    Directory(&'a NdsDirectory),
    File(&'a NdsFile),
}
