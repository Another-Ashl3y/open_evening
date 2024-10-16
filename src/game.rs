use colored::ColoredString;

use crate::console::Console;
use crate::tree::{Directory, File, Text};
use std::io::{stdin, stdout, Write};
use std::thread::sleep;
use std::time::Duration;

pub struct Game {
    pub score: usize,
    pub username: String,
}

impl Game {
    pub fn new() -> Self {
        let username = input("Enter your name > ");
        Self { username, score: 0 }
    }
    pub fn challenge_0(&mut self) {
        sleep(Duration::from_secs(1));
        println!(
            "{}",
            colour_command("\n=== Welcome to the CAT challenge ===")
        );
        //  println!(
        //      "This challenge requires the {} and {} command.",
        //        colour_command("cat"),
        //        colour_command("ls")
        //    );
        //    println!(
        //      "The {} command will show you the contents of a file.",
        //      colour_command("cat")
        //     );
        //  println!("The {} or \"list stuff\" command will show you all the files in your current directory.", colour_command("ls"));
        //  println!("A directory is like a position in a book. Except that book is split into folders and those folders contain more folders.");
        // sleep(Duration::from_secs(3));
        // println!("\n{}", colored::Colorize::bold("=== Instructions ==="));
        //  println!("1. Type {} and press enter.", colour_command("ls"));
        //  println!(
        //      "2. This will output the contents of the directory. \n  {} should be amongst them.",
        //      colour_command("flag.txt")
        //  );
        //  println!(
        //      "3. Type {} and press enter.",
        //      display(&format!(
        //              "{} {}",
        //          colour_command("cat"),
        //         colored::Colorize::cyan("flag.txt")
        //      ))
        // );
        //println!("4. Type {} and press enter.", colour_command("exit"));
        //println!(
        //    "5. Type the entire content of the flag.txt file into the terminal and press enter.\n\n"
        //);

        let flag = "flag{th3_cAt_fl@g!!!}";
        let mut flag_found = false;

        let mut root = File::Directory(Directory::new("/".to_string()));
        let mut home = File::Directory(Directory::new("home".to_string()));
        let mut user = File::Directory(Directory::new(self.username.clone()));
        let secret_0 = File::Text(Text::new("secret_0.txt".to_string(), "You have found a secret (1/3). Find all secrets and win a sweet!\nFirst Secret: Ducks Are Cool\nMake sure to write this down!".to_string()));
        let flag_file = File::Text(Text::new("flag.txt".to_string(), flag.to_string()));

        user.add_file(flag_file);
        user.add_file(secret_0);
        home.add_file(user);
        root.add_file(home);

        let mut console = Console::new(
            root,
            vec!["home".to_string(), self.username.clone()],
            self.username.clone(),
        );

        while !flag_found {
            console.start();
            self.time_to_enter_flag();
            let attempt = input("enter flag > ");
            if attempt == flag {
                flag_found = true;
            } else {
                println!("Flag incorrect!");
                println!("Try again? [Y/n]");
                if input("\"n\" to exit > ") == "n" {
                    println!("{} captured {} flags", self.username.clone(), self.score);
                    println!("exiting...");
                    return;
                }
            }
        }
        println!(
            "Congratulations {}! You completed the first challenge!",
            self.username.clone()
        );
        self.score += 1;
        self.challenge_1();
    }

    pub fn challenge_1(&mut self) {
        sleep(Duration::from_secs(1));
        println!(
            "\n{}",
            colour_command("=== Welcome to the MAZE challenge ===")
        );
        //println!("This challenge will teach you the \"cd\" or \"change directory\" command.");
        //println!("To use the \"cd\" command type:\n  cd [path]\nWhere [path] is the path you want to change directory to.");
        //println!("You can use the \"dir\" command to show your current directory");
        //println!("\nTo go back a folder type: cd ../\nTo enter a folder type: cd [folder name]");
        //sleep(Duration::from_secs(3));
        //println!(
        //    "To complete this challenge you must explore the folders and find the flag.txt file."
        //);
        //println!("Once you have found the flag.txt file run: cat flag.txt");
        //println!("Then exit the console and enter your flag.");

        let flag = "flag{i_@m_A_maz3_eXpert23!}";
        let mut flag_found = false;

        let mut root = File::Directory(Directory::new("/".to_string()));
        let mut home = File::Directory(Directory::new("home".to_string()));
        let mut user = File::Directory(Directory::new(self.username.clone()));
        let mut desktop = File::Directory(Directory::new("desktop".to_string()));
        let mut downloads = File::Directory(Directory::new("downloads".to_string()));
        let mut documents = File::Directory(Directory::new("documents".to_string()));
        let mut dead_end = File::Directory(Directory::new("dead_end".to_string()));
        let easter_egg = File::Text(Text::new(
            "easter_egg.txt".to_string(),
            "We get biscuits every week in A-Level computer science!".to_string(),
        ));
        let secret_1 = File::Text(Text::new("secret_1.txt".to_string(), "You have found a secret (2/3). Find all secrets and win a sweet!\nSecond Secret: Computer science is the best!\nMake sure to write this down!".to_string()));
        let flag_file = File::Text(Text::new("flag.txt".to_string(), flag.to_string()));

        desktop.add_file(easter_egg);
        dead_end.add_file(secret_1);
        documents.add_file(dead_end);
        downloads.add_file(flag_file);
        user.add_file(desktop);
        user.add_file(downloads);
        user.add_file(documents);
        home.add_file(user);
        root.add_file(home);

        let mut console = Console::new(
            root,
            vec!["home".to_string(), self.username.clone()],
            self.username.clone(),
        );

        while !flag_found {
            console.start();
            self.time_to_enter_flag();
            let attempt = input("enter flag > ");
            if attempt == flag {
                flag_found = true;
            } else {
                println!("Flag incorrect!");
                println!("Try again? [Y/n]");
                if input("\"n\" to exit > ") == "n" {
                    println!("{} captured {} flags", self.username.clone(), self.score);
                    println!("exiting...");
                    return;
                }
            }
        }
        println!(
            "Congratulations {}! You completed the second challenge!",
            self.username.clone()
        );
        self.score += 1;
        self.challenge_2();
    }

    pub fn challenge_2(&mut self) {
        sleep(Duration::from_secs(1));
        println!(
            "{}",
            colour_command("\n=== Welcome to the TREES challenge ===")
        );
        println!("Trees are really cool don't you think?");
        sleep(Duration::from_secs(1));
        //println!("Anyway, there is another command called \"tree\".");
        //println!("You can use this to find files much faster.");
        //println!("Try it yourself!\n\nRemember to type exit once you have the flag and then enter it into the terminal.");

        let flag = "flag{tre3s_MAk3_lIfe_eas1er}";
        let mut flag_found = false;

        let mut root = File::Directory(Directory::new("/".to_string()));
        let mut home = File::Directory(Directory::new("home".to_string()));
        let mut user = File::Directory(Directory::new(self.username.clone()));
        let mut documents = File::Directory(Directory::new("documents".to_string()));
        let mut downloads = File::Directory(Directory::new("downloads".to_string()));
        let mut desktop = File::Directory(Directory::new("desktop".to_string()));
        let mut trash = File::Directory(Directory::new("trash".to_string()));
        let mut music = File::Directory(Directory::new("music".to_string()));
        let mut homework = File::Directory(Directory::new("homework".to_string()));
        let mut images = File::Directory(Directory::new("images".to_string()));

        let flag_file = File::Text(Text::new("flag.txt".to_string(), flag.to_string()));
        let duck_0 = File::Text(Text::new(
            "ducks.txt".to_string(),
            "Mr Frost really likes ducks for some reason...".to_string(),
        ));
        let songs = File::Text(Text::new("songs.txt".to_string(), "Green Day\nSeether\nPearl Jam\nAlice In Chains\nNirvana\nRaue\nOlive Vox\nStone Sour\nAURORA\nMotzart\nBeathoven".to_string()));
        let physics_homework = File::Text(Text::new(
            "physics_homework.txt".to_string(),
            "My homework ate my dog and then my cat ate my homework :(".to_string(),
        ));
        let maths_homework = File::Text(Text::new(
            "maths_homework.txt".to_string(),
            "a^2+b^2=c^2\n3^2 + 4^2 = ?^2\n9 + 16 = ?^2\n25 = ?^2\n5 = ?\n=======\n  Answer: 31.23"
                .to_string(),
        ));
        let blackmail = File::Text(Text::new(
            "blackmail.txt".to_string(),
            "I know about the biscuits".to_string(),
        ));
        let secret_2 = File::Text(Text::new("secret_2.txt".to_string(), "You have found a secret (3/3). Find all secrets and win a sweet!\nThird Secret: I looked from root\nMake sure to write this down!".to_string()));

        trash.add_file(blackmail);
        desktop.add_file(trash);
        homework.add_file(physics_homework);
        homework.add_file(maths_homework);
        documents.add_file(homework);
        music.add_file(songs);
        downloads.add_file(duck_0);
        downloads.add_file(flag_file);

        user.add_file(desktop);
        user.add_file(music);
        user.add_file(downloads);
        user.add_file(documents);
        user.add_file(images);

        home.add_file(user);

        root.add_file(secret_2);
        root.add_file(home);

        let mut console = Console::new(
            root,
            vec!["home".to_string(), self.username.clone()],
            self.username.clone(),
        );

        while !flag_found {
            console.start();
            self.time_to_enter_flag();
            let attempt = input("enter flag > ");
            if attempt == flag {
                flag_found = true;
            } else {
                println!("Flag incorrect!");
                println!("Try again? [Y/n]");
                if input("\"n\" to exit > ") == "n" {
                    println!("{} captured {} flags", self.username.clone(), self.score);
                    println!("exiting...");
                    return;
                }
            }
        }
        println!(
            "Congratulations {}! You completed the final challenge!",
            self.username.clone()
        );
        self.score += 1;

        println!(
            "{} captured {} (all) flags",
            self.username.clone(),
            self.score
        );
    }

    fn time_to_enter_flag(&self) {
        println!(
            "{}",
            colored::Colorize::on_black("/=====================\\")
        );
        println!("{}", colored::Colorize::on_black("| Time to enter flag! |"));
        println!(
            "{}",
            colored::Colorize::on_black("\\=====================/")
        );
    }
}

pub fn input(prompt: &str) -> String {
    print!("{}", colored::Colorize::bold(prompt));
    let _ = stdout().flush();
    let mut q = String::new();
    stdin().read_line(&mut q).unwrap();
    q.replace("\n", "").replace("\r", "")
}

pub fn colour_command(change: &str) -> ColoredString {
    colored::Colorize::underline(colored::Colorize::bold(colored::Colorize::cyan(change)))
}

pub fn display(change: &str) -> ColoredString {
    colored::Colorize::on_black(change)
}
