use std::fmt::Display;

#[derive(Clone, Copy)]
pub struct Config<'a> {
    pub ip: &'a String,
}

impl<'a> Config<'a> {
    pub fn new(args: &[String]) -> Result<Config, impl Display> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let ip = &args[1];
        Ok( Config{ ip })
    }
}