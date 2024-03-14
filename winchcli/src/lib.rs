#[derive(Clone)]
pub struct Argument {
    pub short: Vec<char>,
    pub long: Vec<String>,
    pub help: String, 
}

#[derive(Debug)]
pub struct Event {
    pub arguments_passed: Vec<Argument>
}

impl std::fmt::Debug for Argument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Argument {{ short: {:?}, long: {:?}, help: {:?} }}", self.short, self.long, self.help)
    }
}

pub fn short(short: char) -> Argument {
    Argument {
        short: vec![short],
        long: vec![],
        help: "".to_string(),
    }
}

pub fn long(long: &'static str) -> Argument {
    Argument {
        short: vec![],
        long: vec![long.to_string()],
        help: "".to_string(),
    }
}

// short::<&str>('c').long("config").help("Config file").handler(|arg| {
//     println!("Config file: {}", arg);
// })

impl Argument {
    pub fn short(mut self, short: char) -> Self {
        self.short.push(short);
        self
    }

    pub fn long(mut self, long: &'static str) -> Self {
        self.long.push(long.to_string());
        self
    }

    pub fn help(mut self, help: &'static str) -> Self {
        self.help = help.to_string();
        self
    }
}

pub struct OptionsParser {
    pub handler: Option<&'static dyn Fn(&Event) -> ()>,
}

impl IntoIterator for Argument {
    type Item = Argument;
    type IntoIter = std::vec::IntoIter<Argument>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self].into_iter()
    }
}

impl OptionsParser {
    pub fn execute(&self, args: Vec<Argument>) {
        let mut arguments_passed: Vec<Argument> = vec![];
        std::env::args().for_each(|arg: String| {
            // see if we have valid arguments
            for argument in &args {
                let arg_chars: Vec<char> = arg.chars().collect(); 
                if arg_chars == argument.short || argument.long.contains(&arg) {
                    // find valid handler for that argument
                    let values_args_pair: Vec<&str> = arg.split("=").collect();
                    for (arg, value) in values_args_pair.iter().enumerate() {
                        if arg == 0 {
                            arguments_passed.push(argument.clone());
                        } else {
                            let mut argument = argument.clone();
                            argument.help = value.to_string();
                            arguments_passed.push(argument);
                        }
                    }
                    arguments_passed.push(argument.clone());
                }
            }
        });
        self.handler.unwrap()(&Event { arguments_passed });
    }

    pub fn set_handler(mut self, handler: &'static dyn Fn(&Event) -> ()) -> Self {
        self.handler = Some(handler);
        self
    }
}

