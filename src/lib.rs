pub mod cli {
    use crate::cmd::Command;

    #[derive(Debug)]
    pub struct App<'a> {
        name: &'a str,
        author: &'a str,
        version: &'a str,
        description: &'a str,
        commands: Vec<Command>,
    }
    pub fn new<'a>() -> App<'a> {
        App{
            name: "Application",
            author: "",
            version: "",
            description: "",
            commands: vec![]
        }
    }
}

pub mod cmd {
    use std::io;

    #[derive(Debug)]
    pub struct Command {
        run: fn() -> Result<String, io::Error>,
    }
}