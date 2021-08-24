#[cfg(test)]
mod tests {
    use falsework::cmd;
    use std::error::Error;

    #[test]
    fn test_command_build() {

        let mut command = cmd::CommandItem {
            run: |ctx| -> Result<(), Box<dyn Error>> {
                let x = ctx.value_of("--x").parse::<i32>().unwrap();
                let y = ctx.value_of("--y").parse::<i32>().unwrap();
                println!("{} + {} = {}", x, y, x + y);
                Ok(())
            },
            long: "这是一个加法计算程序需要两个flag参数 --x --y",
            short: "加法计算",
            r#use: "add",
        }.build();

        command.bound_flag("--x", "加数");

        command.bound_flag("--y", "被加数");

        println!("{:#?}", command);
    }


    #[test]
    fn test_command_add() {
        let mut app = falsework::app::new();

        app.name("Calculator")
            .author("Leon Ding <ding@ibyte.me>")
            .version("0.0.2")
            .description("A command line program built with Falsework.");

        let mut command = cmd::CommandItem {
            run: |ctx| -> Result<(), Box<dyn Error>> {
                let x = ctx.value_of("--x").parse::<i32>().unwrap();
                let y = ctx.value_of("--y").parse::<i32>().unwrap();
                println!("{} + {} = {}", x, y, x + y);
                Ok(())
            },
            long: "这是一个加法计算程序需要两个flag参数 --x --y",
            short: "加法计算",
            r#use: "add",
        }.build();

        command.bound_flag("--x", "加数");
        command.bound_flag("--y", "被加数");

        app.add_cmd(command);

        println!("{:#?}", app);
    }

    #[test]
    fn test_add_commands() {
        let mut app = falsework::app::new();

        app.name("Calculator")
            .author("Leon Ding <ding@ibyte.me>")
            .version("0.0.2")
            .description("A command line program built with Falsework.");


        let command_list = vec![
            cmd::CommandItem {
                run: |_ctx| -> Result<(), Box<dyn Error>> {
                    println!("call foo command.");
                    Ok(())
                },
                long: "这是一个测试命令，使用foo将调用foo命令。",
                short: "foo命令",
                r#use: "foo",
            },
            cmd::CommandItem {
                run: |_ctx| -> Result<(), Box<dyn Error>> {
                    println!("call bar command.");
                    Ok(())
                },
                long: "这是一个测试命令，使用bar将调用bar命令。",
                short: "bar命令",
                r#use: "bar",
            },
        ].iter().map(|c| c.build()).collect();

        app.commands(command_list);

        println!("{:#?}", app);
    }

}
