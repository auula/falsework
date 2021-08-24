use falsework::cmd;
use falsework::cmd::Flag;
use std::collections::HashMap;
use std::error::Error;

fn main() {
    let mut app = falsework::app::new();

    app.name("Falsework")
        .author("Leon Ding <ding@ibyte.me>")
        .version("0.0.2")
        .description("A command line program built with Falsework.");

    let mut map: HashMap<String, Flag> = HashMap::new();
    提示
    map.insert(
        "--flag".to_string(),
        Flag {
            flag: "--flag",
            usages: "-f usages",
            value: "-f value".to_string(),
        },
    );

    map.insert(
        "--host".to_string(),
        Flag {
            flag: "--host",
            usages: "怎么使用这个--host",
            value: "-f value".to_string(),
        },
    );

    let com = cmd::Command {
        run: |ctx| -> Result<(), Box<dyn Error>> {
            println!("host={}", ctx.value_of("--host"));
            println!("flag={}", ctx.value_of("--flag"));
            Err(cmd::err_msg("运行时发送错误"))
        },
        long: "这是一条长的命令介绍信息",
        short: "短介绍",
        r#use: "cmd",
        flags: map,
    };
    app.add_cmd(com);

    app.run();
}
