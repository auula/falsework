# Falsework 

![https://img.shields.io/badge/falsework-Rust%20CLI-brightgreen](https://img.shields.io/badge/falsework-Rust%20Crates-yellow)
[![Go](https://github.com/higker/falsework/actions/workflows/coverage.yml/badge.svg?event=push)](https://github.com/auula/falsework/actions/workflows/coverage.yml)
[![codecov](https://codecov.io/gh/auula/falsework/branch/main/graph/badge.svg?token=22QKRI2IFE)](https://codecov.io/gh/auula/falsework)
![https://img.shields.io/github/repo-size/auula/falsework](https://img.shields.io/github/repo-size/auula/falsework)
![https://img.shields.io/crates/v/falsework](https://img.shields.io/crates/v/falsework)
[![License](https://img.shields.io/badge/license-MIT-db5149.svg)](https://github.com/higker/falsework/blob/master/LICENSE)

A tool crate to quickly build rust Command line application.


## 导入依赖

在你的项目中添加依赖如下：

```toml
[dependencies]
falsework = "0.1.0"
```


## 快速构建

```rust
use std::error::Error;
use falsework::cmd;

fn main() {
    
    // 通过falsework创建一个骨架
    let mut app = falsework::app::new();
    
    
    // 应用元数据信息
    app.name("calculator")
        .author("Leon Ding <ding@ibyte.me>")
        .version("0.0.1")
        .description("A calculator that only supports addition.");

    // 构建命令行项
    let mut command = cmd::CommandItem {
        // run add命令所对应命令行逻辑代码
        run: |ctx| -> Result<(), Box<dyn Error>> {
            // 通过上下文获取flag绑定的数据
            let x = ctx.value_of("--x").parse::<i32>().unwrap();
            let y = ctx.value_of("--y").parse::<i32>().unwrap();
            println!("{} + {} = {}", x, y, x + y);
            // 如果处理发生了错误则调用 cmd::err_msg 会优雅的退出
            // Err(cmd::err_msg("Application produce error！"));
            Ok(())
        },
        // 命令帮助信息
        long: "这是一个加法计算程序需要两个flag参数 --x --y",
        // 命令介绍
        short: "加法计算",
        // 通过add激活命令
        r#use: "add",
    }.build();
    
    // 给add命令绑定flag
    command.bound_flag("--x", "加数");
    command.bound_flag("--y", "被加数");
    
    // 往app里面添加一个命令集
    app.add_cmd(command);
    
    // 最后run 程序开始监听命令行输入
    app.run();
}
```

上面这个例子运行起来结果:

```shell
$: ./calculator add --x=10 --y=10
10 + 10 = 20
```

到此为止你就快速构建一个命令行计算器了，你只需要写你核心逻辑，其他操作`falsework`帮助你完成。

1. 例如如果我不记得了命令了，只记得一个单词或者字母，程序会帮助你修复：

```shell
$: ./calculator a

You need this command ?
	add
a : The corresponding command set was not found!
```
2. 可以看到程序提示你有一个对应的`add`命令可以使用，如果不知道`add`有啥参数，在后面
加上`--help`即可获得帮助信息：

```shell
$: ./calculator add --help

Help:
  这是一个加法计算程序需要两个flag参数 --x --y

Usage:
  calculator add [flags]

Flags:
  --y, 被加数
  --x, 加数
```
构建出来主程序预览：

```shell
$: ./calculator


A calculator that only supports addition.
calculator 0.0.1 Leon Ding <ding@ibyte.me>

Usage:
  calculator  [command]

Available Commands:
  add	加法计算

Flags:
  --help   help for calculator

Use "calculator [command] --help" for more information about a command.
```

## 其他操作
有多种构建方式，例如下面的：

```rust
    #[test]
    fn test_add_commands() {
        let mut app = falsework::app::new();

        app.name("calculator")
            .author("Leon Ding <ding@ibyte.me>")
            .version("0.0.2")
            .description("A command line program built with Falsework.");


        let command_list = vec![
            cmd::CommandItem {
                run: |_ctx| -> Result<(), Box<dyn Error>> {
                    // _ctx.args 获取命令行参数
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
```

## 其他
本项目也是笔者一个练手写的`crate`，看了半个月的`Rust`书，学了点基础，不知道写什么所以撸一个这个玩玩。。
如果对你有帮助记得`star`，`falsework`目前构建一些`command line application`问题不大，不过源代码写的有点挫。。。后面再改进吧。