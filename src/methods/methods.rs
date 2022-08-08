use std::collections::HashMap;

trait Helper {
    fn convert_string_to_i32(required_string:&str) -> i32;

    fn return_string_length_to_i32(required_string:&str) -> i32;
}


struct Method<'a> {
    method: &'static str,
    method_value: &'static str,
    items: &'a HashMap<String, HashMap<String, String>>
}

impl<'a> Helper for Method<'a> {
    fn convert_string_to_i32(required_string:&str) -> i32 {
        let return_str = required_string.parse::<i32>().unwrap_or_else(|e| panic!("{}", e));
        return_str
    }

    fn return_string_length_to_i32(required_string:&str) -> i32 {
        let return_string :i32 = required_string.len().try_into().unwrap_or_else(|e| panic!("{}", e));
        return_string
    }
}

impl<'a>  Method<'a> {

    fn new((method, method_value):&(&'static str, &'static str), items:&'a HashMap<String, HashMap<String, String>>) -> Self{
            let method_wrapper = Method {
                method,
                method_value,
                items
            };
            method_wrapper
    }

    fn verify_method(&self) {
        let item = &self.items["item"]["value"];
        match self.method {
            "required" =>{
                            let result = Method::required(&item);
                            Method::match_result(result);
                        }
            "max" => {
                        let result  = Method::max(&item, self.method_value);
                        Method::match_result(result);
                    },
            "min"=>{
                        let result  = Method::min(&item, self.method_value);
                        
                        Method::match_result(result);
                    },
            _=>println!("hey string what")
        }
    }
    fn match_result(res:Result<bool, bool>){
        match res {
            Ok(true) => println!("Yay"),
            Err(false) => println!("Nah"),
            _=> println!("Nah"),
        }
    }
    fn required(val: &str)-> Result<bool, bool>{
        let result =   match val {
             "" => Err(false),
             _ => Ok(true)
          };
          result
    }
    fn max(val: &str, max:&str) -> Result<bool, bool>{
        let max = Method::convert_string_to_i32(max);
        let val :i32 = Method::return_string_length_to_i32(val);
       // let val :i32 = val.len().try_into().unwrap();
       // let max = max.parse::<i32>().unwrap();
        let result = if val <= max {
            println!("{} {}", max, val);
            Ok(true)
        } else  {
            Err(false)
        };
        result
    }
    fn min(val: &str, min:&str) -> Result<bool, bool>{
        
        let min = Method::convert_string_to_i32(min);
        let val  = Method::return_string_length_to_i32(val);
       // let val :i64 = val.len().try_into().unwrap();
        //let min = min.parse::<i64>().unwrap();
        let result = if val <= min {
            println!("{} {}", min, val);
            Err(false)
        } else  {
            Ok(true)
        };
        result
    }

    fn send_err() {

    }
}

#[cfg(test)]
mod method_test {
    use  super::*;
    #[test]
    fn method() {
       let mut items = HashMap::new();
       let mut vals = HashMap::new();
       vals.insert("name".to_string(), "name".to_string());
       vals.insert("value".to_string(), "John Doe".to_string() );
       vals.insert("type".to_string(), "string".to_string());
       items.insert("Item".to_string(), vals);
       let methods = ("max", "100");

       let method_wrapper = Method::new(&methods, &items);
        assert_eq!(method_wrapper.method_value, methods.1);
    }   
    #[test]
    fn max_should_pass() {    
        let result = Method::max("John Doe", "15");
            let match_result = match result {
                Ok(true) => true,
                Err(false)=> false,
                Ok(false)=> false,
                Err(true) =>true,
            };
            

        assert_eq!(match_result, true);
    }
    #[test]
    fn max_should_fail() {
        
        let result = Method::max("John Doe", "5");
            let match_result = match result {
                Ok(true) => true,
                Err(false)=> false,
                Ok(false)=> false,
                Err(true) =>true,
            };
            

        assert_eq!(match_result, false);
    }
    #[test]
    fn min_should_pass() {    
        let result = Method::min("John Doe", "5");
            let match_result = match result {
                Ok(true) => true,
                Err(false)=> false,
                Ok(false)=> false,
                Err(true) =>true,
            };
            

        assert_eq!(match_result, true);
    }
    #[test]
    fn min_should_fail() {
        
        let result = Method::min("John Doe", "10");
            let match_result = match result {
                Ok(true) => true,
                Err(false)=> false,
                Ok(false)=> false,
                Err(true) =>true,
            };
            

        assert_eq!(match_result, false);
    }

    
    #[test]
    fn required_should_pass() {
        
        let result = Method::required("John Doe");
            let match_result = match result {
                Ok(true) => true,
                Err(false)=> false,
                Ok(false)=> false,
                Err(true) =>true,
            };
            

        assert_eq!(match_result, true);
    }
    
    #[test]
    fn required_should_fail() {
        
        let result = Method::required("");
            let match_result = match result {
                Ok(true) => true,
                Err(false)=> false,
                Ok(false)=> false,
                Err(true) =>true,
            };
            

        assert_eq!(match_result, false);
    }
}