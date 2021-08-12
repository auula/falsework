use std::collections::HashMap;
use crate::cmd::Command;
use std::env;

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

    pub fn get_command(&self, r#use: &str) -> Option<&Command<'c, 'f>> {
        for (k, v) in self.commands.iter() {
            if k == r#use {
                return Some(v);
            }
        }
        None
    }

    pub fn get_command_mut(&mut self, r#use: &str) -> Option<&mut Command<'c, 'f>> {
        for (k, v) in &mut self.commands.iter_mut() {
            if k == r#use {
                return Some(v);
            }
        }
        None
    }
    // 1. 除了cmd 还需要外面的flag
    pub fn run(&self) {

        let args = env::args().collect::<Vec<String>>();

        if args.len() <= 1 {
            self.print();
            return;
        }

        match self.commands.get(&*args.as_slice()[1]) {
            None => { self.print(); }
            Some(cmd) => {
                let mut arguments: Vec<String> = Vec::new();

                for argument in args {
                    for x in argument.split("=") {
                        arguments.push(x.to_string());
                    }
                }
                (cmd.run)(cmd.context(arguments));
            }
        }
    }

    fn print(&self) {
        let mut commands = String::new();
        for (k, v) in self.commands.iter() {
            commands.push_str(format!("{}\t{}\n", k, v.long).as_str());
        }
        println!("\
             {banner} \n\
             {description} \n\n\
             {name} {version} {author}\n\n\
             Usage:\n
  {name}  [command] \n\n\
             Available Commands:\n
  {commands}\n\n\
             Flags:\n
  --help   help for {name}\n
  --debug  debug for {name}\n\n\
             Use \"{name} [command] --help\" for more information about a command.",
                 name = self.name,
                 banner = self.banner,
                 version = self.version,
                 author = self.author,
                 commands = commands,
                 description = self.description);
    }
}