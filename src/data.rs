
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum LuaData{
    Str(String),
    Number(f64),
    Bool(bool),
    Nil,
}

impl LuaData{

    pub fn to_num(&self) -> f64{
        match self{
            LuaData::Number(x) => *x,
            LuaData::Bool(false) => 0.0,
            LuaData::Bool(true) => 1.0,
            LuaData::Str(_) => unimplemented!("String to number conversion isn't implemented"),
            LuaData::Nil => 0.0,
        }
    }

    pub fn to_string(&self) -> String{
        match self{
            LuaData::Str(x) => (*x).to_string(),
            LuaData::Bool(x) => format!("{}", x),
            LuaData::Number(x) => format!("{}", x),
            LuaData::Nil => "nil".to_string(),
        }
    }

    pub fn to_bool(&self) -> bool{
        match self{
            LuaData::Nil => false,
            LuaData::Str(x) if x == "false" => false,
            LuaData::Bool(x) => *x,
            _ => true
        }
    }
}

impl fmt::Display for LuaData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self{
            LuaData::Str(string) => write!(f, "{}", string),
            LuaData::Number(number) => write!(f, "{}", number),
            LuaData::Bool(b) => write!(f, "{}", b),
            LuaData::Nil => write!(f, "nil"),
        }
    }
}

#[cfg(test)]
mod data_tests{

    use super::LuaData;

    #[test]
    fn to_num_test(){
        let start_vec = vec![LuaData::Number(5.0), LuaData::Nil, LuaData::Bool(false), LuaData::Bool(true)];
        let expected_vec = vec![5.0, 0.0, 0.0, 1.0];    

        for index in 0..start_vec.len(){            
            let actual = start_vec[index].clone();
            let expected = expected_vec[index].clone();

            assert_eq!(actual.to_num(), expected, "{:?} != {:?}", start_vec[index], expected);
        }
    }

    #[test]
    fn to_str_test(){
        let start_vec = vec![LuaData::Str("foo".to_string()), LuaData::Bool(true), 
            LuaData::Bool(false), LuaData::Number(12.34), LuaData::Nil];
        let expected_vec: Vec<String> = vec!["foo", "true", "false", "12.34", "nil"]
            .iter().map(|x| x.to_string()).collect();    

        for index in 0..start_vec.len(){            
            let actual = start_vec[index].clone();
            let expected = expected_vec[index].clone();

            assert_eq!(actual.to_string(), expected, "{:?} != {:?}", start_vec[index], expected);
        }
    }

    #[test]
    fn to_bool_test(){
        let start_vec = vec![LuaData::Bool(false), LuaData::Nil, LuaData::Str("false".to_string()), LuaData::Number(0.0), 
            LuaData::Str("foo".to_string()), LuaData::Bool(true)];
        let expected_vec = vec![false, false, false, true, true, true];   

        for index in 0..start_vec.len(){            
            let actual = start_vec[index].clone();
            let expected = expected_vec[index].clone();

            assert_eq!(actual.to_bool(), expected, "{:?} != {:?}", start_vec[index], expected);
        }
    }

}