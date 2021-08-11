pub mod cli {
    use crate::cmd::{Command, Flag, Types};
    use std::collections::HashMap;

    #[derive(Debug)]
    pub struct App<'a, 'c> {
        name: &'a str,
        author: &'a str,
        version: &'a str,
        description: &'a str,
        banner: String,
        flags: Flag,
        commands: HashMap<String, Command<'c>>,
    }

    pub fn new<'a, 'c>() -> App<'a, 'c> {
        App {
            name: "Falsework",
            author: "Author name <email@email.com>",
            version: "0.0.1",
            description: "A command line program built with Falsework.",
            banner: "".to_string(),
            flags: Flag { command: "root".to_string(), item: Default::default() },
            commands: HashMap::new(),
        }
    }

    impl<'a, 'c> App<'a, 'c> {
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

        pub fn flags(&mut self) -> &mut Flag {
            &mut self.flags
        }
        pub fn add_cmd(&mut self, cmd: Command<'c>) -> &mut Self {
            self.commands.insert(cmd.r#use.to_string(), cmd);
            self
        }

        pub fn commands(&mut self, cmd_list: Vec<Command<'c>>) {
            for v in cmd_list {
                self.commands.insert(v.r#use.to_string(), v);
            }
        }

        pub fn get_command(&self, r#use: &str) -> Option<&Command<'c>> {
            for (k, v) in self.commands.iter() {
                if k == r#use {
                    return Some(v);
                }
            }
            None
        }

        pub fn get_command_mut(&mut self, r#use: &str) -> Option<&mut Command<'c>> {
            for (k, v) in &mut self.commands.iter_mut() {
                if k == r#use {
                    return Some(v);
                }
            }
            None
        }
        // 1. 除了cmd 还需要外面的flag
        pub fn run(&self) {
            self.print();
        }

        fn print(&self) {
            let mut commands = String::new();
            for (k, v) in self.commands.iter() {
                commands.push_str(format!("\t{}\t{}\n", k, v.long).as_str());
            }
            println!("\
             {banner} \n\
             {description} \n\
             {name} {version} {author}\n\n\
             Usage:\n\
             \t{name}  [command] \n\n\
             Available Commands:\n\
             {commands}\n\n\
             Flags:\n\
             \t-h, --help   help for {name}\n\n\
             Use \"{name} [command] --help\" for more information about a command.",
                     name = self.name,
                     banner = self.banner,
                     version = self.version,
                     author = self.author,
                     commands = commands,
                     description = self.description);
        }
    }
}

pub mod cmd {
    use std::collections::HashMap;
    use std::error::Error;
    use std::env::Args;
    use std::io;
    use std::rc::Rc;
    use std::ptr::null;
    use std::option::Option::Some;

    pub type RunResult = Result<(), Box<dyn Error>>;

    pub type RunFunc = fn(
        ctx: Context, args: Vec<String>,
    ) -> RunResult;

    #[derive(Debug)]
    pub struct Context {
        pub stdout: io::Stdout,
        pub stderr: io::Stderr,
    }


    #[derive(Debug, Default)]
    pub struct Command<'c> {
        pub run: Option<RunFunc>,
        // Long is the long message shown in the 'help <this-command>' output.
        pub long: &'c str,
        // Short is the short description shown in the 'help' output.
        pub short: &'c str,
        pub r#use: &'c str,
        pub flags: Vec<Flag>,
        // pub aliases: Vec<&'c str>,
    }

    impl<'c> Command<'c> {
        // pub fn flags(&mut self) -> &mut Vec<Flag<T>> {
        //     &mut self.flags
        // }
        pub fn flags(&mut self) -> &mut Vec<Flag> {
            &mut self.flags
        }
    }

    #[derive(Debug)]
    pub enum Types {
        Float(i32),
        Bool(bool),
        String(String),
    }

    //Paola
    #[derive(Debug)]
    pub struct Flag {
        pub command: String,
        pub item: HashMap<String, FlagItem>,
    }

    #[derive(Debug)]
    pub struct FlagItem {
        pub flag: String,
        pub short: String,
        pub usages: String,
        pub default: Types,
        pub value: Box<Types>,
    }

    impl Flag {
        // s2s add -x 10 -y 10 = 20
        // -x i64 default= 10 usages 加数

        pub fn bound(&mut self, mut boundValue: Option<Types>, flag: &str, short: &str, defValue: Types, usages: &str) -> RunResult {
            if boundValue.is_none() {
                match &defValue {
                    Types::Float(v) => {
                        boundValue = Option::from(Types::Float(*v));
                    }
                    Types::Bool(v) => {
                        boundValue = Option::from(Types::Bool(*v));
                    }
                    Types::String(v) => {
                        boundValue = Option::from(Types::String(v.clone()));
                    }
                }
            }

            self.item.insert(flag.to_string(), FlagItem {
                flag: flag.to_string(),
                short: short.to_string(),
                usages: usages.to_string(),
                default: defValue,
                value: Box::from(boundValue.unwrap()),
            });
            Ok(())
        }
    }
}

