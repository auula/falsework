pub mod cli {
    use crate::cmd::Command;


    #[derive(Debug)]
    pub struct App<'a, 'c, T> {
        name: &'a str,
        author: &'a str,
        version: &'a str,
        description: &'a str,
        commands: Vec<Command<'c, T>>,
    }

    pub fn new<'a, 'c, T>() -> App<'a, 'c, T> {
        App {
            name: "Falsework",
            author: "Author name <email@email.com>",
            version: "0.0.1",
            description: "A command line program built with Falsework.",
            commands: vec![],
        }
    }

    impl<'a, 'c, T> App<'a, 'c, T> {
        pub fn name(&mut self, name: &'a str) -> &mut App<'a, 'c, T> {
            self.name = name;
            self
        }
        pub fn author(&mut self, author: &'a str) -> &mut App<'a, 'c, T> {
            self.author = author;
            self
        }
        pub fn version(&mut self, version: &'a str) -> &mut App<'a, 'c, T> {
            self.version = version;
            self
        }

        pub fn description(&mut self, description: &'a str) -> &mut App<'a, 'c, T> {
            self.description = description;
            self
        }

        pub fn add_cmd(&mut self, cmd: Command<'c, T>) -> &mut App<'a, 'c, T> {
            self.commands.push(cmd);
            self
        }

        pub fn commands(&mut self, cmd_list: Vec<Command<'c, T>>) {
            for v in cmd_list {
                self.commands.push(v);
            }
        }

        pub fn run(&self) {}
    }
}

pub mod cmd {
    #[derive(Debug)]
    pub struct Command<'c, T> {
        pub run: fn(),
        // Long is the long message shown in the 'help <this-command>' output.
        pub long: &'c str,
        // Short is the short description shown in the 'help' output.
        pub short: &'c str,
        pub r#use: &'c str,
        pub flags: Vec<Flag<T>>,
        // pub aliases: Vec<&'c str>,
    }

    impl<'c, T> Command<'c, T> {
        pub fn flags(&mut self) -> &mut Vec<Flag<T>> {
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
    pub struct Flag<T> {
        pub name: String,
        pub r#type: Types,
        pub usages: String,
        pub value: Box<T>,
    }

    impl<T> Flag<T> {
        // s2s add -x 10 -y 10 = 20
        // -x i64 default= 10 usages 加数
        pub fn string(&mut self, name: &str, default: T, usages: &str) {
            self.name = name.parse().unwrap();
            self.value = Box::new(default);
            self.r#type = Types::STRING;
            self.usages = usages.parse().unwrap()
        }

        pub fn int(&mut self, name: &str, default: T, usages: &str) {
            self.name = name.parse().unwrap();
            self.value = Box::new(default as i64);
            self.r#type = Types::I64;
            self.usages = usages.parse().unwrap()
        }

        pub fn bool(&mut self, name: &str, default: T, usages: &str) {
            self.name = name.parse().unwrap();
            self.value = Box::new(default as bool);
            self.r#type = Types::BOOL;
            self.usages = usages.parse().unwrap()
        }

        pub fn float(&mut self, name: &str, default: T, usages: &str) {
            self.name = name.parse().unwrap();
            self.value = Box::new(default as f64);
            self.r#type = Types::F64;
            self.usages = usages.parse().unwrap()
        }
    }
}

