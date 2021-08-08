#[cfg(test)]
mod tests {
    use falsework::cmd;

    /// Adds one to the number given.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = 5;
    /// let y = 5;
    ///
    /// assert_eq!(x + y, 10);
    /// ```
    #[test]
    fn it_works() {
        assert_eq!(5 + 5, 10);
    }

    #[test]
    fn test_new() {
        let mut app = falsework::cli::new();

        println!("{:#?}", app);

        app.name("new name")
            .author("Leon Ding <ding@ibyte.me>")
            .version("0.0.2")
            .description("a command line program built with Falsework");

        app.add_cmd(cmd::Command {
            run: || println!("run 1"),
            r#use: "cmd1",
        });

        let cmd_list = &[
            cmd::Command {
                run: || println!("run 2"),
                r#use: "cmd2",
            },
            cmd::Command {
                run: || println!("run 3"),
                r#use: "cmd3",
            }
        ];

        app.commands(cmd_list);

        println!("{:#?}", app);
    }
}
