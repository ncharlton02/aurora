#[derive(Debug, PartialEq)]
pub enum LogLevel{
    Quiet, Normal, Verbose 
}

#[derive(Debug, PartialEq)]
pub struct Config{
    pub log_level: LogLevel,
}

impl Config{

    pub fn new(level: LogLevel) -> Config{
        Config{log_level: level}
    }

}