use std::{collections::HashMap, error::Error};
use serde::{Deserialize, Serialize};
use serde_json::{Value, to_string};
mod methods;




type  HashmapData = HashMap<String, HashMap<String, String>>;



#[derive(Serialize, Deserialize, Debug)]

struct HashmapDataStruct {
    content:  HashMap<String, HashmapData> 
}
struct DataTypes {
    string:String,
    number:String,
    float:String,
    email:String
}

enum EnumDataTypes {
    String(String),
    Number(i64),
    Float(f32),
    Email(String)
}



impl HashmapDataStruct {

    fn new(data:HashMap<String, HashmapData>) ->  Self {
        let  data = HashmapDataStruct {
            content: data
        };
        data
    }

    fn loop_through(self) {
        for i  in self.content.iter() {
            println!("For I: {:?}", i.1);
            HashmapDataStruct::validate_items(&i.1);
        };
    }
    fn validate_items(items:&HashMap<String, HashMap<String, String>>) {
        
        match items["item"]["type"].as_str() {
            "string" => println!("hey string"),
            "number" => match items["item"]["value"].parse::<i64>().unwrap(){
                _i64 => { 
                    for method in items["methods"].iter() {
                        HashmapDataStruct::validate_methods(method, &items["item"])
                    }
                },
                _=>println!("{}", "bad value"),
            },
            "float" =>  match items["item"]["value"].parse::<f32>().unwrap(){
                            _f32 => println!("{}", items["item"]["value"]),
                            _=>println!("{}", "bad value"),
                        },
            "email" =>println!("hey email"),
            "" => println!("hey unknown"),
            _ =>println!("hey last option validate")
        }
 
    }
    fn validate_methods((method, method_value):(&String, &String),  item: &HashMap<String, String>) {
        match method.as_str() {
            "required" =>{
                            let result = require(&item["value"]);
                            match_result(result, );
                        }
            "max" => {
                        let result  = max(&item["value"], &method_value);
                        match_result(result);
                    },
            "min"=>{
                        let result  = min(&item["value"], &method_value);
                        
                        match_result(result);
                    },
            _=>println!("hey string what")
        }
        
        fn match_result(res:Result<bool, bool>) {
            match res {
                Ok(true) => println!("Yay"),
                Err(false) => println!("Nah"),
                _=> println!("Nah"),
            }
        }
        fn require(val: &str)-> Result<bool, bool>{
          let result =   match val {
               "" => Err(false),
               _ => Ok(true)
            };
            result
        }
        fn max(val: &str, max:&str) -> Result<bool, bool>{
            let val :i64 = val.len().try_into().unwrap();
            let max = max.parse::<i64>().unwrap();
            let result = if val <= max {
                println!("{} {}", max, val);
                Ok(true)
            } else  {
                Err(false)
            };
            result
        }
        fn min(val: &str, min:&str) -> Result<bool, bool>{
            let val :i64 = val.len().try_into().unwrap();
            let min = min.parse::<i64>().unwrap();
            let result = if val <= min {
                println!("{} {}", min, val);
                Err(false)
            } else  {
                Ok(true)
            };
            result
        }
    
    }


    
}

fn main() {
    let data1 = r#"
    {
        
        "name":[
                  {"item":{"name": "name","value": "John Doe", "type": "string"},  "methods":{"max": "100", "required": "true", "min":"10"}}
                ],       
        "password":[
                    {"item":{"name": "password", "value": "1234567", "type": "number"},  "methods":{"max": "100", "required": "true", "min":"10"}}
                  ]



    
    }"#;


    let data = r#"
    {
        
        "name":{"item":{"name": "name","value": "John Doe", "type": "string"},  "methods":{"max": "100", "required": "true", "min":"10"}},
        "password":{"item":{"name": "password", "value": "1234567", "type": "number"},  "methods":{"max": "100", "required": "true", "min":"10"}},  
        "phonenumber":{"item":{"name": "phonenumber", "value": "08133475878", "type": "number"},  "methods":{"max": "100", "required": "true", "min":"10"}}    
    
    }"#;


    let res = HashmapDataStruct::new(serde_json::from_str(data).unwrap());
    res.loop_through();
}
