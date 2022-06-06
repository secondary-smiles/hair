pub const VERSION: &'static str = "0.2.1";

pub struct Arg {
    pub name: &'static str,
    pub short: Option<char>,
    pub long: Option<&'static str>,
    pub help: &'static str,
}

impl std::fmt::Debug for Arg {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Arg {{ name: {}, short: {:?}, long: {:?}, help: {} }}", self.name, self.short, self.long, self.help)
    }
}

impl Copy for Arg { }

impl Clone for Arg {
    fn clone(&self) -> Arg {
        *self
    }
}
