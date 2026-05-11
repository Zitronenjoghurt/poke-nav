use crate::platform::nds::formats::NdsFileFormat;
use crate::platform::nds::fs::file::NdsFile;
use crate::platform::nds::fs::{NdsFileSystem, ROOT_DIR_ID};
use std::collections::HashSet;

#[derive(Default)]
pub struct NdsFileTree {
    pub root: NdsFileTreeDir,
}

#[derive(Default)]
pub struct NdsFileTreeDir {
    pub name: String,
    pub path: String,
    pub children: Vec<NdsFileTreeEntry>,
}

pub struct NdsFileTreeFile {
    pub name: String,
    pub path: String,
    pub format: Option<NdsFileFormat>,
    pub size: usize,
    pub children: Option<Vec<NdsFileTreeEntry>>,
}

pub enum NdsFileTreeEntry {
    Dir(NdsFileTreeDir),
    File(NdsFileTreeFile),
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NdsFileTreeFilter {
    pub search: String,
    pub formats: HashSet<NdsFileFormat>,
    pub include_empty_dirs: bool,
    #[serde(skip, default = "default_true")]
    pub dirty: bool,
}

fn default_true() -> bool {
    true
}

impl Default for NdsFileTreeFilter {
    fn default() -> Self {
        Self {
            search: String::new(),
            formats: HashSet::new(),
            include_empty_dirs: true,
            dirty: true,
        }
    }
}

impl NdsFileTreeFilter {
    pub fn matches_dir(&self, dir: &NdsFileTreeDir) -> bool {
        let has_visible_children = !dir.children.is_empty();
        let matches_search = self.matches_search(&dir.name);
        has_visible_children || (matches_search && self.include_empty_dirs)
    }

    pub fn matches_file(&self, file: &NdsFileTreeFile) -> bool {
        let matches_format = self.formats.is_empty()
            || file
                .format
                .as_ref()
                .is_some_and(|f| self.formats.contains(f));
        let matches_search = self.matches_search(&file.name);
        let has_visible_children = file.children.as_ref().is_some_and(|c| !c.is_empty());
        (matches_format && matches_search) || has_visible_children
    }

    fn matches_search(&self, name: &str) -> bool {
        self.search.is_empty()
            || name
                .to_ascii_lowercase()
                .contains(&self.search.to_ascii_lowercase())
    }
}

impl NdsFileTree {
    pub fn build(fs: &NdsFileSystem, filter: &NdsFileTreeFilter) -> Self {
        Self {
            root: NdsFileTreeDir::build(fs, filter, ROOT_DIR_ID, String::new(), String::new()),
        }
    }
}

impl NdsFileTreeDir {
    pub fn build(
        fs: &NdsFileSystem,
        filter: &NdsFileTreeFilter,
        dir_id: u16,
        name: String,
        parent_path: String,
    ) -> Self {
        let path = if parent_path.is_empty() {
            name.clone()
        } else {
            format!("{}/{}", parent_path, name)
        };
        let children = build_children(fs, filter, dir_id, &path);
        Self {
            name,
            path,
            children,
        }
    }
}

impl NdsFileTreeFile {
    pub fn build(file: &NdsFile, path: String, filter: &NdsFileTreeFilter) -> Self {
        let path = format!("{}/{}", path, file.name);
        let children = file
            .data
            .nested_fs()
            .map(|nested_fs| build_children(nested_fs, filter, ROOT_DIR_ID, &path));

        Self {
            name: file.name.clone(),
            path,
            format: file.data.format(),
            size: file.size,
            children,
        }
    }
}

fn build_children(
    fs: &NdsFileSystem,
    filter: &NdsFileTreeFilter,
    dir_id: u16,
    path: &str,
) -> Vec<NdsFileTreeEntry> {
    let mut children = Vec::new();

    for dir in fs.child_dirs(dir_id) {
        let dir = NdsFileTreeDir::build(fs, filter, dir.id, dir.name.clone(), path.to_string());
        if !filter.matches_dir(&dir) {
            continue;
        }
        children.push(NdsFileTreeEntry::Dir(dir));
    }

    for file in fs.child_files(dir_id) {
        let file = NdsFileTreeFile::build(file, path.to_string(), filter);
        if !filter.matches_file(&file) {
            continue;
        }
        children.push(NdsFileTreeEntry::File(file));
    }

    children
}
