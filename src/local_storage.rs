pub mod storage {

    use std::{fs::{self, File}, io::{Read, Write}};
    use serde_json::Value;

    pub fn get_user_stored_info(filename: String) -> Result<Value, ()> {
        match File::open(filename) {
            Ok(mut file) => {
                let mut json = String::new();
                file.read_to_string(&mut json).ok().unwrap();
                match serde_json::from_str(&json) {
                    Ok(res) => Ok(res),
                    Err(_) => {
                        println!("json error");
                        Err(())
                    }
                }
            }
            Err(_) => Err(())
        }
    }

    pub fn write_user_info_localy(json: Value, filename:String) -> Result<(), String> {
        match File::create(filename) {
            Ok(mut file) => {
                match file.write_all(json.to_string().as_bytes()){
                    Ok(_) => Ok(()),
                    Err(_) => Err("Cant write data".to_string())
                }
            }
            Err(_) => Err("Cant create file".to_string())
        }
    }

    pub fn is_acceptance_info_stored(filename: String) -> bool {
        File::open(filename).is_ok()
    }

    pub fn remove_acceptance_local_info(filename: String) {
        fs::remove_file(filename).unwrap_or_default();
    }
}
