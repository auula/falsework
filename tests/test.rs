#[cfg(test)]
mod tests {
    use falsework::cmd;
    use std::env;
    use std::error::Error;
    use std::fs::OpenOptions;
    use std::io;
    use std::io::Write;
    use std::collections::HashMap;

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
            run: Option::Some(|ctx| -> Result<(), Box<dyn Error>> {
                println!("{:?}", ctx);
                let mut file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open("/Users/ding/Desktop/falsework/test.txt")?;
                file.write_all(ctx.args[1].as_bytes())?;
                Ok(())
            }),
            r#use: "cmd1",
            ..Default::default()
        });


        if let Some(cmd) = app.get_command("cmd1") {
            match cmd.run {
                None => {
                    panic!("no run func")
                }
                Some(func) => {
                    match func(
                        cmd::Context {
                            args: env::args().collect(),
                            flag: HashMap::new(),
                        }
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            panic!("running {}", err)
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_add_command_list() {
        let mut app = falsework::cli::new();

        app.commands(vec![
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
            },
        ]);

        println!("{:#?}", app);
        app.run();
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
        let root_command = cmd::Command {
            run: Option::None,
            long: "Short is the short description shown in the 'help' output.",
            short: "cmd short",
            r#use: "cmd",
            ..Default::default()
        };

        // root_command.flags()[1].bound_string(&mut host, "-h", "127.0.0.1", "主机参数");

        // assert_eq!(host, "127.0.0.1");
        println!("{:#?}", root_command)
    }

    #[test]
    fn test_try_error() {
        let mut app = falsework::cli::new();

        app.add_cmd(cmd::Command {
            run: Option::Some(|ctx| -> Result<(), Box<dyn Error>> {
                println!("{:?}", ctx);
                let mut file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open("/Users/ding/Desktop/falsework/test.txt")?;
                file.write_all(ctx.args[1].as_bytes())?;
                Ok(())
            }),
            r#use: "cmd1",
            ..Default::default()
        });
        if let Some(cmd) = app.get_command("cmd1") {
            match cmd.run {
                None => {
                    panic!("no run func")
                }
                Some(func) => {
                    match func(
                        cmd::Context {
                            args: env::args().collect(),
                            flag: HashMap::new(),
                        }
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            panic!("running {}", err)
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_run() {
        let mut app = falsework::cli::new();

        app.commands(vec![
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
            },
        ]);

        app.run();
    }

    #[test]
    fn test_flags() {
        let mut app = falsework::cli::new();

        let mut command = cmd::Command {
            run: Option::None,
            long: "cmd 1 long help",
            short: "cmd 1 short help",
            r#use: "cmd 1",
            ..Default::default()
        };


        command.bound_flag("flag", "-f", "测试flag参数绑定");

        println!("{:#?}", command.flags());

        println!("{:#?}", command.get_flag("flag").unwrap().get_value());

    }
}
