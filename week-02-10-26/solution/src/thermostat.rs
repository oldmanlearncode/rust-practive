use crate::measurement::*;

pub enum Mode {
    Off,
    Auto,
    Manual(bool),
}

pub enum Command {
    SetTarget(i32),
    Sample(Measurement),
    SetMode(Mode),
    Rename(String),
}

pub struct Thermostat {
    name: String,
    target: i32,
    last: Option<i32>,
    mode: Mode,
}

impl Thermostat {
    pub fn new(name: String) -> Self {
        Thermostat {
            name,
            target: 22,
            last: None,
            mode: Mode::Off,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn target(&self) -> i32 {
        self.target
    }

    pub fn last(&self) -> Option<i32> {
        self.last
    }

    pub fn mode(&self) -> &Mode {
        &self.mode
    }

    pub fn apply(&mut self, command: Command) -> bool {
        match command {
            Command::SetTarget(t) => {
                if !(16..=30).contains(&t) {
                    false
                } else {
                    self.target = t;
                    true
                }
            }
            Command::Sample(m) => {
                let sample = temperature(&m);
                match sample {
                    Some(t) => {
                        self.last = Some(t);
                        true
                    }
                    None => false,
                }
            }
            Command::SetMode(m) => {
                self.mode = m;
                true
            }
            Command::Rename(s) => {
                let new_name = s.trim();
                if new_name.is_empty() {
                    return false;
                }
                self.name = s;
                true
            }
        }
    }

    pub fn is_heating(&self) -> bool {
        match &self.mode {
            Mode::Off => false,
            Mode::Manual(on) => *on,
            Mode::Auto => match &self.last {
                None => false,
                Some(t) => *t < self.target,
            },
        }
    }
}
