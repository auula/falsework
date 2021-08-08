#[cfg(test)]
mod tests {
    use falsework::cmd;
    use falsework::cmd::Types;

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
            long: "Short is the short description shown in the 'help' output.",
            short: "cmd short",
            r#use: "cmd1",
            // aliases: vec![]
            flags: vec![
                cmd::Flag {
                    name: "-d".to_string(),
                    value: "a".to_string(),
                    usages: "被加的数字".to_string(),
                    r#type: Types::I64,
                }
            ],
        });

        // let cmd_list = vec![
        //     cmd::Command {
        //         run: || println!("run 2"),
        //         long: "",
        //         short: "SSSS2",
        //         r#use: "cmd2",
        //         // flags: vec![],
        //         // aliases: vec![]
        //         flags: &[
        //
        //         ]
        //     },
        //     cmd::Command {
        //         run: || println!("run 3"),
        //         long: "",
        //         short: "",
        //         r#use: "cmd3",
        //         // flags: vec![],
        //         // aliases: vec![]
        //     }
        // ];
        //
        // if let Some(elem) = cmd_list.get(0){
        //     let func = elem.run;
        //     func()
        // }
        // app.commands(cmd_list);
        println!("{:#?}", app);
    }
}
