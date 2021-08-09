pub mod cli {
    use crate::cmd::{Command};

    #[derive(Debug)]
    pub struct App<'a, 'c> {
        name: &'a str,
        author: &'a str,
        version: &'a str,
        description: &'a str,
        commands: Vec<Command<'c>>,
    }

    pub fn new<'a, 'c>() -> App<'a, 'c> {
        App {
            name: "Falsework",
            author: "Author name <email@email.com>",
            version: "0.0.1",
            description: "A command line program built with Falsework.",
            commands: vec![],
        }
    }

    impl<'a, 'c> App<'a, 'c> {
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

        pub fn add_cmd(&mut self, cmd: Command<'c>) -> &mut Self {
            self.commands.push(cmd);
            self
        }

        pub fn commands(&mut self, cmd_list: Vec<Command<'c>>) {
            for v in cmd_list {
                self.commands.push(v);
            }
        }

        pub fn get_command(&self, r#use: &str) -> Option<&Command<'c>> {
            for v in &self.commands {
                if v.r#use == r#use {
                    return Some(v);
                }
            }
            None
        }

        pub fn get_command_mut(&mut self, r#use: &str) -> Option<&mut Command<'c>> {
            for v in &mut self.commands {
                if v.r#use == r#use {
                    return Some(v);
                }
            }
            None
        }
        pub fn run(&self) {}
    }
}

pub mod cmd {
    use std::collections::HashMap;
    use std::error::Error;
    use std::env::Args;
    use std::io;

    pub type RunFunc = fn(
        ctx: Context, args: Vec<String>,
    ) -> Result<(), Box<dyn Error>>;

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
        I64,
        F64,
        BOOL,
        STRING,
    }

    #[derive(Debug)]
    pub struct Flag {
        pub flag: String,
        pub default: String,
        pub r#type: Types,
        pub usages: String,
        // pub value: Box<T>,
    }

    impl Flag {
        // s2s add -x 10 -y 10 = 20
        // -x i64 default= 10 usages 加数

        pub fn bound_string(&mut self, value: &mut String, flag: &str, default: &str, usages: &str) -> Result<(), Box<dyn Error>> {
            self.r#type = Types::STRING;
            self.usages = usages.parse().unwrap();
            self.flag = flag.parse().unwrap();
            // value = xx;
            Ok(())
        }

        // pub fn string(&mut self, name: &str, default: T, usages: &str) {
        //     self.name = name.parse().unwrap();
        //     self.value = Box::new(default);
        //     self.r#type = Types::STRING;
        //     self.usages = usages.parse().unwrap()
        // }
        //
        // pub fn int(&mut self, name: &str, default: T, usages: &str) {
        //     self.name = name.parse().unwrap();
        //     self.value = Box::new(default);
        //     self.r#type = Types::I64;
        //     self.usages = usages.parse().unwrap()
        // }
        //
        // pub fn bool(&mut self, name: &str, default: T, usages: &str) {
        //     self.name = name.parse().unwrap();
        //     self.value = Box::new(default);
        //     self.r#type = Types::BOOL;
        //     self.usages = usages.parse().unwrap()
        // }
        //
        // pub fn float(&mut self, name: &str, default: T, usages: &str) {
        //     self.name = name.parse().unwrap();
        //     self.value = Box::new(default);
        //     self.r#type = Types::F64;
        //     self.usages = usages.parse().unwrap()
        // }
    }
}

