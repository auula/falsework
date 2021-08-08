pub mod cli {
    use crate::cmd::Command;

    #[derive(Debug)]
    pub struct App<'a, 'c_u> {
        name: &'a str,
        author: &'a str,
        version: &'a str,
        description: &'a str,
        commands: Vec<Command<'c_u>>,
    }

    pub fn new<'a, 'c_u>() -> App<'a, 'c_u> {
        App {
            name: "Falsework",
            author: "Author name <email@email.com>",
            version: "0.0.1",
            description: "A command line program built with Falsework.",
            commands: vec![],
        }
    }

    impl<'a, 'c_u> App<'a, 'c_u> {
        pub fn name(&mut self, name: &'a str) -> &mut App<'a, 'c_u> {
            self.name = name;
            self
        }
        pub fn author(&mut self, author: &'a str) -> &mut App<'a, 'c_u> {
            self.author = author;
            self
        }
        pub fn version(&mut self, version: &'a str) -> &mut App<'a, 'c_u> {
            self.version = version;
            self
        }

        pub fn description(&mut self, description: &'a str) -> &mut App<'a, 'c_u> {
            self.description = description;
            self
        }

        pub fn add_cmd(&mut self, cmd: Command) -> &mut App<'a, 'c_u> {
            self.commands.push(cmd);
            self
        }

        pub fn commands(&mut self, cmd_list: &[Command]) {
            for &v in cmd_list.into_iter() {
                self.commands.push(*v);
            }
        }
    }
}

pub mod cmd {
    use std::io;

    #[derive(Debug)]
    pub struct Command<'c_u> {
        run: fn() -> Result<String, io::Error>,
        r#use: &'c_u str,
    }
}