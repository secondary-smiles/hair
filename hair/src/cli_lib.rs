pub struct Arg {
    pub name: &'static str,
    pub short: Option<char>,
    pub long: Option<&'static str>,
    pub help: &'static str,
}


pub trait Run {
    fn run(&self);
}

impl Copy for Arg { }

impl Clone for Arg {
    fn clone(&self) -> Arg {
        *self
    }
}
