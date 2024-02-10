use serde_yaml::{self, Mapping, Sequence, Value};
use std::{error::Error, io::{self, Read}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let args: Sequence = serde_yaml::from_str(&buffer)?;
    let args_string = val_vec_to_arg_string(args)?;

    println!("{}", args_string.join(" "));
    Ok(())
}

fn val_vec_to_arg_string(args: Vec<Value>) -> Result<Vec<String>, Box<dyn Error>> {
    return args.iter().map(|arg: &Value| -> Result<String, Box<dyn std::error::Error>> {
        let a = arg.as_mapping().ok_or("Not a mapping")?;
        return stringify_arg(a)
    }).collect();
}

fn stringify_arg(a: &Mapping) -> Result<String, Box<dyn Error>> {
    let arg_type = get_value(a, "type")?;
    match arg_type.as_str() {
        "option" => {
            let key = get_value(a, "key")?;
            let value = get_value(a, "value")?;
            Ok(format!("{} {}", key, value))
        },
        "flag" => {
            let key = get_value(a, "key")?;
            Ok(format!("{}", key))
        },
        "arg" => {
            let value = get_value(a, "value")?;
            Ok(format!("{}", value))
        },
        _ => Err("Unknown type".to_string().into())
    }
}

fn get_value(arg: &Mapping, key: &str) -> Result<String, Box<dyn std::error::Error>> {
    let value = arg.get(&Value::String(key.to_string())).ok_or("Key not found")?;
    let str_value = value.as_str().ok_or("Value is not a string")?;
    Ok(str_value.to_string())
}
