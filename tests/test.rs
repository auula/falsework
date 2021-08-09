#[cfg(test)]
mod tests {
    use falsework::cmd;
    use falsework::cmd::{
        Types, Context,
    };
    use std::error::Error;
    use std::io::Write;
    use std::io;
    use std::env;

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
            run: Option::Some(
                |c, args| -> Result<(), Box<dyn Error>> {
                    let mut handle = c.stdout.lock();
                    handle.write_all(b"running");
                    Ok(())
                }
            ),
            long: "Short is the short description shown in the 'help' output.",
            short: "cmd short",
            r#use: "cmd1",
            ..Default::default()
        });

        if let Some(cmd) = app.get_command("cmd1") {
            match cmd.run {
                None => {
                    panic!("no run func")
                }
                Some(func) => {
                    func(
                        cmd::Context {
                            stdout: io::stdout(),
                            stderr: io::stderr(),
                        },
                        env::args().collect(),
                    );
                }
            }
        }
    }

    #[test]
    fn test_add_command_list() {
        let mut app = falsework::cli::new();

        app.commands(
            vec![
                cmd::Command {
                    run: Option::None,
                    long: "cmd 1 long help",
                    short: "cmd 1 short help",
                    r#use: "cmd 1",
                    ..Default::default()
                },
                cmd::Command {
                    run: Option::None,
                    long: "cmd 2 long help",
                    short: "cmd 2 short help",
                    r#use: "cmd 2",
                    ..Default::default()
                }
            ]
        );
        println!("{:#?}", app);
    }

    #[test]
    fn test_get_command_and_mut() {
        let mut app = falsework::cli::new();


        app.add_cmd(cmd::Command {
            run: None,
            long: "Short is the short description shown in the 'help' output.",
            short: "cmd short",
            r#use: "cmd",
            ..Default::default()
        });

        if let Some(cmd) = app.get_command_mut("cmd") {
            cmd.r#use = "cmd not equal";
            assert_eq!(cmd.r#use, "cmd not equal")
        }

        if let Some(cmd) = app.get_command("cmd") {
            assert_eq!(cmd.r#use, "cmd")
        }
    }

    #[test]
    fn test_command_flags() {
        let mut host: String;
        let mut root_command = cmd::Command {
            run: Option::None,
            long: "Short is the short description shown in the 'help' output.",
            short: "cmd short",
            r#use: "cmd",
            ..Default::default()
        };

        // root_command.flags()[1].bound_string(&mut host, "-h", "127.0.0.1", "主机参数");

        // assert_eq!(host, "127.0.0.1");
    }
}
