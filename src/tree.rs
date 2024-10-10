use std::fmt::{write, Display};

#[derive(Clone)]
pub enum File {
    Directory(Directory),
    Text(Text),
}

impl File {
    pub fn add_file(&mut self, file: File) {
        match self {
            File::Directory(d) => d.add_file(file),
            _ => panic!("Cannot create files in non-directory"),
        }
    }
    pub fn name(&self) -> String {
        match self {
            File::Directory(d) => format!("{} DIRECTORY", d.name.clone()),
            File::Text(t) => format!("{} TXT", t.name.clone()),
        }
    }
    pub fn format(&self, position: usize) -> String {
        let mut q = format!("{}¬ {}", "  ".repeat(position), self.name());
        if let File::Directory(d) = self {
            for child in d.contents.clone() {
                q = format!("{}\n|{}", q, child.format(position + 1));
            }
        }
        q
    }
    pub fn is_directory(&self, path: Vec<String>) -> bool {
        let mut found = false;
        let file = self.get_file(path);
        if let Some(f) = file {
            if let Self::Directory(_d) = f {
                found = true;
            }
        }
        found
    }
    pub fn get_file(&self, path: Vec<String>) -> Option<File> {
        let mut path = path;
        let mut found = Option::None;
        if !path.is_empty() {
            let this = path.remove(0);
            if let File::Directory(d) = self {
                for child in d.contents.clone() {
                    if child.name() == this {
                        if path.clone().is_empty() {
                            //println!("{} vs {}", child.name(), this);
                            found = Some(child);
                        } else {
                            found = child.get_file(path.clone());
                        }
                    }
                }
            }
        } else if self.name() == "/" {
            found = Some(self.clone());
        }
        found
    }
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write(f, format_args!("{}", self.format(0)))
    }
}

#[derive(Clone)]
pub struct Directory {
    pub name: String,
    pub contents: Vec<File>,
}

impl Directory {
    pub fn new(name: String) -> Directory {
        Directory {
            name,
            contents: Vec::new(),
        }
    }
    pub fn add_file(&mut self, file: File) {
        self.contents.push(file);
    }
}

#[derive(Clone, Debug)]
pub struct Text {
    pub name: String,
    pub content: String,
}

impl Text {
    pub fn new(name: String, content: String) -> Text {
        Text { name, content }
    }
}
