use std::collections::HashMap;
use crate::cmd::Command;
use std::{env, process};

#[derive(Debug)]
pub struct App<'a, 'c, 'f> {
    name: &'a str,
    author: &'a str,
    version: &'a str,
    description: &'a str,
    banner: String,
    commands: HashMap<String, Command<'c, 'f>>,
}

pub fn new<'a, 'c, 'f>() -> App<'a, 'c, 'f> {
    App {
        name: "Falsework",
        author: "Author name <email@email.com>",
        version: "0.0.1",
        description: "A command line program built with Falsework.",
        banner: "".to_string(),
        commands: HashMap::new(),
    }
}

impl<'a, 'c, 'f> App<'a, 'c, 'f> {
    pub fn banner(&mut self, banner: String) -> &mut Self {
        self.banner = banner;
        self
    }

    pub fn name(&mut self, name: &'a str) -> &mut Self {
        self.name = name;
        self
    }
    pub fn author(&mut self, author: &'a str) -> &mut Self {
        self.author = author;
        self
    }
    pub fn version(&mut self, version: &'a str) -> &mut Self {
        self.version = version;
        self
    }

    pub fn description(&mut self, description: &'a str) -> &mut Self {
        self.description = description;
        self
    }

    pub fn add_cmd(&mut self, cmd: Command<'c, 'f>) -> &mut Self {
        self.commands.insert(cmd.r#use.to_string(), cmd);
        self
    }

    pub fn commands(&mut self, cmd_list: Vec<Command<'c, 'f>>) {
        for v in cmd_list {
            self.commands.insert(v.r#use.to_string(), v);
        }
    }

    pub fn get_command_mut(&mut self, r#use: &str) -> Option<&Command<'c, 'f>> {
        self.commands.get(r#use)
    }



    pub fn run(&self) {
        let args = env::args().collect::<Vec<String>>();

        if args.len() <= 1 {
            self.print();
            return;
        }


        match self.commands.get(&*args.as_slice()[1]) {
            None => {
                println!();
                println!("You need this command ?");
                let cmd = &*args.as_slice()[1];
                if &*args.as_slice()[1] == "--help" {
                    self.print();
                    return;
                }
                for key in self.commands.keys() {
                    if key.contains(&cmd) {
                        println!("\t{}\n", key);
                    }
                }
                println!("{} : The corresponding command set was not found!", cmd);
            }
            Some(cmd) => {
                let mut arguments: Vec<String> = Vec::new();

                for argument in &args {
                    for x in argument.split("=") {
                        arguments.push(x.to_string());
                    }
                }

                if args.as_slice().len() == (3 as usize) {
                    if &*args.as_slice()[2] == "--help" {
                        println!();
                        println!("Help:");
                        println!("  {}", cmd.long);
                        println!();
                        println!("Usage:");
                        println!("  {} {} [flags]", self.name, &cmd.r#use);
                        println!();
                        println!("Flags:");
                        for x in &cmd.flags {
                            println!("  {}, {}", x.0, x.1.usages);
                        }
                        return;
                    }
                }
                match (cmd.run)(cmd.context(arguments)) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("{}", e);
                        process::exit(1);
                    }
                }
            }
        }
    }

    fn print(&self) {
        let mut commands = String::new();
        for (k, v) in self.commands.iter() {
            commands.push_str(format!(" {}\t{}\n\r", k, v.short).as_str());
        }
        println!("{}", self.banner);
        println!();
        println!("{}", self.description);
        println!("{name} {version} {author}", name = self.name, version = self.version, author = self.author);
        println!();
        println!("Usage:");
        println!("  {}  [command]", self.name);
        println!();
        println!("Available Commands:");
        println!("{}", commands);
        println!();
        println!("Flags:");
        println!("  --help   help for {}", self.name);
        println!();
        println!("Use \"{} [command] --help\" for more information about a command.", self.name);
    }
}