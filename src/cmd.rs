//! The Command module
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct Flag<'f> {
    pub flag: &'f str,
    pub usages: &'f str,
    pub value: String,
}

impl<'f> Flag<'f> {}

/// # Command struct
/// Used to build the command execution body
#[derive(Debug)]
pub struct Command<'c, 'f> {
    /// implement this for it to work properly.
    pub run: fn(Context<'f>) -> Result<(), Box<dyn Error>>,
    /// Long is the long message shown in the 'help <this-command>' output.
    pub long: &'c str,
    /// Short is the short description shown in the 'help' output.
    pub short: &'c str,
    /// Using the command `./falsework use`
    pub r#use: &'c str,
    pub flags: HashMap<String, Flag<'f>>,
}

/// # Context struct
/// Command Context
#[derive(Debug)]
pub struct Context<'f> {
    flag: HashMap<String, Flag<'f>>,
    pub args: Vec<String>,
}

impl<'f> Context<'f> {
    pub fn flag(&self, flag: &str) -> Option<&Flag> {
        self.flag.get(flag)
    }

    pub fn value_of(&self, flag: &str) -> &str {
        match self.flag.get(flag) {
            None => "",
            Some(v) => v.value.as_str(),
        }
    }
}


pub fn err_msg(msg: &str) -> Box<StrError> {
    Box::new(StrError(msg))
}

#[derive(Debug)]
pub struct StrError<'a>(&'a str);

// Error doesn't require you to implement any methods, but
// your type must also implement Debug and Display.
impl<'a> Error for StrError<'a> {}

impl<'a> fmt::Display for StrError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Delegate to the Display impl for `&str`:
        self.0.fmt(f)
    }
}

impl<'c, 'f> Command<'c, 'f> {
    pub fn help(&self) {
        println!(
            "\t{long}\n\n\
            Flags:\n\n\
            \t-h, --help   help for {name}",
            long = self.long,
            name = self.r#use,
        )
    }

    pub fn bound_flag(&mut self, flag: &'f str, usages: &'f str) {
        self.flags.insert(
            flag.to_string(),
            Flag {
                flag,
                usages,
                value: "".to_string(),
            },
        );
    }

    /// Return context
    pub fn context(&self, args: Vec<String>) -> Context<'f> {
        let mut ctx = Context {
            flag: HashMap::new(),
            args: args.clone(),
        };

        if args.len() <= 1 {
            return ctx;
        }

        let args = &args[1..];

        let mut index = 0;
        for v in args.iter() {
            // 变量到倒数第二个就不需要遍历
            if index == args.len() - 1 {
                break;
            }

            for f in self.flags.iter() {
                if v == f.0 {
                    ctx.flag.insert(
                        f.0.clone(),
                        Flag {
                            flag: f.1.flag,
                            usages: f.1.usages,
                            value: args[index + 1..][0].clone(),
                        },
                    );
                }
            }
            index += 1;
        }
        ctx
    }
}
