use std::{
    collections::HashMap,
    fmt::{self},
};

use indextree::{Arena, NodeEdge};
use itertools::Itertools;

pub fn solve_part_one(input: String) -> String {
    const MAX_DIR_SIZE: FileSize = 100_000;
    let commands: Vec<Command> = parse_input(&input);
    let filesystem = FileSystem::from_commands(commands);
    filesystem
        .get_dir_sizes()
        .into_iter()
        .filter(|(_, size)| *size <= MAX_DIR_SIZE)
        .map(|(_, size)| size)
        .sum::<FileSize>()
        .to_string()
}

pub fn solve_part_two(input: String) -> String {
    const MAX_SPACE: FileSize = 70_000_000;
    const FREE_SPACE_REQUIRED: FileSize = 30_000_000;

    let filesystem = FileSystem::from_commands(parse_input(&input));
    let sizes = filesystem.get_dir_sizes();
    let root_size = sizes.iter().find(|(name, _)| name == "/").unwrap().1;
    let free_space = MAX_SPACE - root_size;
    let space_to_free = FREE_SPACE_REQUIRED - free_space;

    sizes
        .into_iter()
        .map(|(_, size)| size)
        .filter(|size| *size >= space_to_free)
        .min()
        .unwrap()
        .to_string()
}

#[derive(Default, PartialEq)]
struct FileSystem<'a> {
    files: Arena<FsEntry<'a>>,
}

impl<'a> FileSystem<'a> {
    pub fn from_commands(commands: Vec<Command<'a>>) -> Self {
        let mut files = Arena::new();
        let mut current_dir = files.new_node(FsEntry::Dir { name: "/" });

        for command in commands {
            match command {
                Command::Cd(cd_type) => match cd_type {
                    CdType::Up => current_dir = files[current_dir].parent().unwrap(),
                    CdType::Down(target) => {
                        current_dir = current_dir
                            .children(&files)
                            .find(|child| *files[*child].get() == FsEntry::Dir { name: target })
                            .unwrap()
                    }
                },
                Command::Ls(entries) => {
                    for entry in entries {
                        let node = files.new_node(entry);
                        current_dir.append(node, &mut files)
                    }
                }
            }
        }

        Self { files }
    }

    pub fn get_dir_sizes(&self) -> Vec<(String, FileSize)> {
        let root = self.files.iter().next().unwrap();
        let root_id = self.files.get_node_id(root).unwrap();
        let mut sizes: HashMap<String, FileSize> = HashMap::new();
        let mut current_dir = String::from("/");
        for edge in root_id.traverse(&self.files) {
            match edge {
                NodeEdge::Start(node_id) => match self.files[node_id].get() {
                    FsEntry::Dir { name } => {
                        let id = format!("{name}_{node_id}");
                        sizes.insert(id.clone(), 0);
                        current_dir = id;
                    }
                    FsEntry::File { size, .. } => *sizes.get_mut(&current_dir).unwrap() += size,
                },
                NodeEdge::End(node_id) => {
                    let node = &self.files[node_id];
                    if let FsEntry::Dir { name } = node.get() {
                        if *name != "/" {
                            let parent_id = node.parent().unwrap();
                            let parent = &self.files[parent_id];
                            let parent_name = parent.get().name();
                            current_dir = format!("{parent_name}_{parent_id}");
                            *sizes.get_mut(&current_dir).unwrap() +=
                                sizes[&format!("{name}_{node_id}")];
                        }
                    }
                }
            }
        }

        sizes
            .into_iter()
            .map(|(name, size)| (name.split('_').next().unwrap().to_string(), size))
            .collect_vec()
    }
}

impl<'a> fmt::Debug for FileSystem<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let root = self.files.iter().next().ok_or(fmt::Error)?;
        let root_id = self.files.get_node_id(root).ok_or(fmt::Error)?;
        writeln!(f, "{:?}", root_id.debug_pretty_print(&self.files))
    }
}

#[derive(Debug, PartialEq)]
enum FsEntry<'a> {
    Dir { name: &'a str },
    File { name: &'a str, size: FileSize },
}

impl<'a> FsEntry<'a> {
    pub fn from_str(s: &'a str) -> Self {
        if s.starts_with("dir") {
            Self::Dir { name: &s[4..] }
        } else {
            let mut parts = s.split_ascii_whitespace();
            let size = parts.next().unwrap().parse().unwrap();
            let name = parts.next().unwrap();
            Self::File { name, size }
        }
    }

    pub fn name(&self) -> &'a str {
        match self {
            FsEntry::Dir { name } => name,
            FsEntry::File { name, .. } => name,
        }
    }
}

type FileSize = u64;

#[derive(Debug, PartialEq)]
enum Command<'a> {
    Cd(CdType<'a>),
    Ls(Vec<FsEntry<'a>>),
}

#[derive(Debug, PartialEq)]
enum CdType<'a> {
    Up,
    Down(&'a str), // &str instead of FsEntry since you can only cd into FsEntry::Dir
}
impl<'a> CdType<'a> {
    fn from_str(cd_target: &'a str) -> Self {
        match cd_target {
            ".." => Self::Up,
            t => Self::Down(t),
        }
    }
}

fn parse_input(input: &str) -> Vec<Command> {
    let mut commands = Vec::new();

    let mut lines = input.lines().skip(1).peekable();
    while let Some(line) = lines.next() {
        if line.starts_with("$ cd") {
            let cd_target = &line[5..];
            commands.push(Command::Cd(CdType::from_str(cd_target)))
        } else if line.starts_with("$ ls") {
            let mut entries = Vec::new();
            #[allow(clippy::while_let_on_iterator)]
            while let Some(entry) = lines.next() {
                entries.push(FsEntry::from_str(entry));
                if let Some(entry) = lines.peek() {
                    if entry.starts_with('$') {
                        break;
                    }
                }
            }
            commands.push(Command::Ls(entries));
        }
    }

    commands
}
