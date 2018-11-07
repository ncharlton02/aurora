use std::collections::{HashMap};
use super::super::data::LuaData;

#[derive(Clone, Debug, PartialEq)]
pub struct Table{
    vars: HashMap<String, LuaData>,
}

impl Table{

    pub fn new() -> Table{
        let data = Table{vars : HashMap::new()};
        
        data
    }

    pub fn assign_variable(&mut self, name: String, data: LuaData){
        self.vars.insert(name, data);
    }

    pub fn get_variable(&self, name: String) -> Option<&LuaData>{
        self.vars.get(&name)
    }

    pub fn get_variable_mut(&mut self, name: String) -> Option<&mut LuaData>{
        self.vars.get_mut(&name)
    }
}