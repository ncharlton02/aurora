
use super::*;

pub trait Library{
    fn load(&self, &mut Interpreter);
}

pub struct AuroraStdLib{}

impl Library for AuroraStdLib{

    fn load(&self, interpreter: &mut Interpreter){
        interpreter.func_manager.register_func("print".to_string(), FunctionDef::Rust(|args, _| -> Result<Option<LuaData>, LuaError>{
            for arg in args{
                print!("{}\t", arg);
            }

            println!();
            Ok(None)
        }));

        interpreter.func_manager.register_func("fail".to_string(), FunctionDef::Rust(|args, interpreter| -> Result<Option<LuaData>, LuaError>{
            if args.len() != 1{
                return Err(interpreter.error(format!("Expected one argument, found {}", args.len())));
            }

            let message = match args.get(0).unwrap(){
                LuaData::Str(x) => x,
                x => return Err(interpreter.error(format!("Expected string, found {}", x)))
            };

            Err(interpreter.error(format!("{}", message)))
        }));

        interpreter.func_manager.register_func("require".to_string(), FunctionDef::Rust(|args, interpreter| -> Result<Option<LuaData>, LuaError>{
            if args.len() != 1{
                return Err(interpreter.error(format!("Expected one argument, found {}", args.len())));
            }

            let path = match args.get(0).unwrap(){
                LuaData::Str(x) => x,
                x => return Err(interpreter.error(format!("Expected string, found {}", x)))
            };

            let src = load_file(path)?;
            let module = load_module(path.to_string(), src, interpreter)?;
    
            Ok(Some(module))
        }));
    }

}

pub fn new_std() -> AuroraStdLib{
    AuroraStdLib{}
}