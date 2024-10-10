use crate::game::{colour_command, input};
use crate::tree::File;
use colored;
use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;

pub struct Console {
    root: File,
    position: Vec<String>,
    username: String,
}

impl Console {
    pub fn new(root: File, position: Vec<String>, username: String) -> Self {
        Self {
            root,
            position,
            username,
        }
    }
    pub fn start(&mut self) {
        //thread::spawn(|| loop {
        //    thread::sleep(Duration::from_secs(30));
        //    println!("\nType {} for help", colour_command("help"));
        //});

        let mut alive = true;

        while alive {
            let inp = input(&format!(
                "/{} | {}> ",
                self.position.join("/"),
                self.username.clone()
            ));
            let instructions: Vec<&str> = inp.split(' ').collect();

            //println!("{:?}", self.position);
            //println!("{}", self.root);
            //println!("{:?}", instructions);

            if instructions[0] == "" {
                continue;
            }

            if instructions[0] == "exit" {
                alive = false;
            } else if instructions[0] == "ls" {
                let mut search_path = self.position.clone();
                if instructions.len() > 1 {
                    search_path = self.get_search_path(instructions[1])
                }
                let collapsed_path = collapse_path(search_path);
                if let Some(p) = collapsed_path {
                    search_path = p;
                } else {
                    continue;
                }
                let file = self.root.get_file(search_path.clone());
                match file {
                    Some(f) => match f {
                        File::Directory(d) => {
                            for child in d.contents {
                                match child {
                                    File::Directory(d) => {
                                        println!("{}", colored::Colorize::red(d.name.as_str()))
                                    }
                                    File::Text(t) => println!("{}", t.name),
                                }
                            }
                        }
                        File::Text(t) => println!("{} is a TXT file", t.name),
                    },
                    Option::None => println!("Directory not found: /{}", search_path.join("/")),
                }
            } else if instructions[0] == "tree" {
                let mut search_path = self.position.clone();
                if instructions.len() > 1 {
                    search_path = self.get_search_path(instructions[1])
                }
                let collapsed_path = collapse_path(search_path);
                if let Some(p) = collapsed_path {
                    search_path = p;
                } else {
                    continue;
                }
                let file = self.root.get_file(search_path.clone());
                match file {
                    Some(f) => {
                        println!("{}", f);
                    }
                    Option::None => println!("Directory not found: /{}", search_path.join("/")),
                }
            } else if instructions[0] == "cat" {
                if instructions.len() == 1 {
                    println!("use \"cat\" to display the contents of a file.\nTo use the cat command run:\n  cat [path]");
                    continue;
                }
                let search_path = self.get_search_path(instructions[1]);
                let collapsed_path = collapse_path(search_path);
                if let Some(p) = collapsed_path {
                    let file = self.root.get_file(p.clone());
                    match file {
                        Some(f) => match f {
                            File::Text(t) => println!("{}", t.content),
                            File::Directory(_d) => println!("{} is a directory", p.join("/")),
                        },
                        Option::None => println!("File not found: {}", p.join("/")),
                    }
                }
            } else if instructions[0] == "cd" {
                if instructions.len() == 1 {
                    println!("cd (change directory) can be used to change your working directory (use the \"dir\" command to check your working directory)\nTo use the command run:\n  {}", colour_command("cd [path]"));
                    continue;
                }
                let search_path = self.get_search_path(instructions[1]);
                let collapsed_path = collapse_path(search_path);
                if let Some(path) = collapsed_path {
                    let file = self.root.get_file(path.clone());
                    if let Some(_f) = file {
                        if self.root.is_directory(path.clone()) {
                            self.position = path;
                        } else {
                            println!("cannot change directory to a file");
                        }
                    } else {
                        println!("directory does not exist: {}", path.join("/"));
                    }
                }
            } else if instructions[0] == "dir" {
                println!("/{}", self.position.join("/"));
            } else {
                println!("command not found: {}", instructions[0]);
            }
        }
    }
    fn get_search_path(&self, addon: &str) -> Vec<String> {
        let mut q = self.position.clone();
        q.extend(
            addon
                .split('/')
                .collect::<Vec<&str>>()
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>(),
        );
        q
    }
}

pub fn collapse_path(path: Vec<String>) -> Option<Vec<String>> {
    /*
     * moves the directory according to any "../"
     */

    // Clean Vec<String>
    let mut path = path;
    let mut pop_list: Vec<usize> = Vec::new();
    for i in 0..path.len() {
        if path[i].is_empty() {
            pop_list.push(i);
        }
    }
    for i in 0..pop_list.clone().len() {
        path.remove(pop_list[i] - i);
    }

    // Collapse path
    for i in 0..path.len() {
        if &path[i] == ".." {
            if i != 0 {
                let mut new_path = path[0..(i - 1)].to_vec();
                if i + 1 < path.len() {
                    //println!("extended");
                    new_path.extend(path[(i + 1)..].to_vec());
                }
                //println!("{:?} -> {:?}", path, new_path.clone());
                return collapse_path(new_path);
            } else {
                println!("No files behind the root folder");
                return Option::None;
            }
        }
    }
    Some(path)
}
