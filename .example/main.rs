use std::error::Error;
use falsework::{app, cmd};

fn main() {
    let mut app = falsework::app::new();

    app.name("calculator")
        .author("Leon Ding <ding@ibyte.me>")
        .version("0.0.1")
        .description("A calculator that only supports addition.");

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

    app.run();
}