#[cfg(test)]
mod tests {
    use falsework::{cmd,app};
    use std::error::Error;
    use falsework::cmd::{Command, Flag};
    use std::env;
    use std::collections::HashMap;

    #[test]
    fn test_command() {
        let mut map: HashMap<String, Flag> = HashMap::new();

        map.insert("-Z".to_string(), Flag {
            flag: "-Z",
            short: "-f short",
            usages: "-f usages",
            value: "-f value".to_string(),
        });

        let com = cmd::Command {
            run: |ctx| -> Result<(), Box<dyn Error>> {
                println!("{}",ctx.value_of("--host"));
                Ok(())
            },
            long: "cmd 2 long help",
            short: "cmd 2 short help",
            r#use: "cmd 2",
            flags: map,
        };

        let mut arguments: Vec<String> = Vec::new();

        for argument in env::args() {
            for x in argument.split("=") {
                arguments.push(x.to_string());
            }
        }

        println!("{:?}", arguments);

        let context = com.context(arguments);

        println!("{:#?}", context);
    }

    #[test]
    fn test_new() {
        let mut app = falsework::app::new();

        app.name("new name")
            .author("Leon Ding <ding@ibyte.me>")
            .version("0.0.2")
            .description("A command line program built with Falsework.");

        let mut map: HashMap<String, Flag> = HashMap::new();

        map.insert("--flag".to_string(), Flag {
            flag: "--flag",
            short: "-f short",
            usages: "-f usages",
            value: "-f value".to_string(),
        });

        map.insert("--host".to_string(), Flag {
            flag: "--host",
            short: "-f short",
            usages: "-f usages",
            value: "-f value".to_string(),
        });

        let com = cmd::Command {
            run: |ctx| -> Result<(), Box<dyn Error>> {
                Ok(())
            },
            long: "cmd 2 long help",
            short: "cmd 2 short help",
            r#use: "cmd 2",
            flags: map,
        };

        app.add_cmd(com);

        println!("{:#?}", app);
    }
}
