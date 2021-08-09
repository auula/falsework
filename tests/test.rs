#[cfg(test)]
mod tests {
    use falsework::cmd;
    use falsework::cmd::Types;

    #[test]
    fn test_build_new() {
        let mut app = falsework::cli::new();

        app.name("new name")
            .author("Leon Ding <ding@ibyte.me>")
            .version("0.0.2")
            .description("a command line program built with Falsework");

        println!("{:#?}", app);
    }

    #[test]
    fn test_add_command() {
        let mut app = falsework::cli::new();

        app.name("new name")
            .author("Leon Ding <ding@ibyte.me>")
            .version("0.0.2")
            .description("A command line program built with Falsework.");

        app.add_cmd(cmd::Command {
            run: || println!("run 1"),
            long: "Short is the short description shown in the 'help' output.",
            short: "cmd short",
            r#use: "cmd1",
        });
        println!("{:#?}", app);
    }

    #[test]
    fn test_add_command_list() {

        let mut app = falsework::cli::new();

        app.commands(
            vec![
                cmd::Command {
                    run: || println!("running function by cmd 1"),
                    long: "cmd 1 long help",
                    short: "cmd 1 short help",
                    r#use: "cmd 1",
                },
                cmd::Command {
                    run: || println!("running function by cmd 2"),
                    long: "cmd 2 long help",
                    short: "cmd 2 short help",
                    r#use: "cmd 2",
                }
            ]
        );
        println!("{:#?}", app);
    }

    #[test]
    fn test_get_command_and_mut() {
        let mut app = falsework::cli::new();

        app.add_cmd(cmd::Command {
            run: || println!("run 1"),
            long: "Short is the short description shown in the 'help' output.",
            short: "cmd short",
            r#use: "cmd",
        });

        if let Some(cmd) = app.get_command_mut("cmd") {
            cmd.r#use = "cmd not equal";
            assert_eq!(cmd.r#use, "cmd not equal")
        }

        if let Some(cmd) = app.get_command("cmd") {
            assert_eq!(cmd.r#use, "cmd")
        }
    }
}
