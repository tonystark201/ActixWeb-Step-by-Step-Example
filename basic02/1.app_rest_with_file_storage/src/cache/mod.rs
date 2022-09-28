use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use serde_json::{json, Map, Value};

pub trait RW {

    fn read(&self,path:&str) -> Map<String, Value> {
        let mut string = String::new();
        if !Path::new(path).exists() {
            File::create(path);
            Map::new()
        } else {
            let mut file = File::open(path).unwrap();
            file.read_to_string(&mut string);
            match serde_json::from_str(&string[..]){
                Ok(value)=>{value},
                Err(error)=>{Map::new()}
            }
        }
    }

    fn write(&self, path:&str, data: &mut  Map<String, Value>){
        let new_data = json!(data);
        fs::write(
            path,
            new_data.to_string()
        ).expect("Unable to write data");
    }
}

pub struct Cache{
    path: String
}

impl Cache {

    pub fn new(path:String) -> Self {
        Cache{path:path}
    }

    pub fn list(&self) -> Map<String, Value> {
        let mut data = self.read(&self.path[..]);
        data
    }

    pub fn get(&self, key:&str)->Value{
        let mut data = self.read(&self.path[..]);
        let result = data.get(key);
        match result {
            Some(value) => { value.clone() },
            None => { Value::Null }
        }
    }

    pub fn create(&self,id:&str,name:&str,age:u8){
        let mut data = self.read(&self.path[..]);
        data.insert(
            id.to_string(),
            json!(
                {
                    "id":id,
                    "name":name,
                    "age":age
                }
            )
        );
        self.write(&self.path[..], &mut data)
    }

    pub fn update(&self,id:&str,name:&str,age:u8){
        let mut data = self.read(&self.path[..]);
        data.remove(id);
        data.insert(
            id.to_string(),
            json!(
                {
                    "id":id,
                    "name":name,
                    "age":age
                }
            )
        );
        self.write(&self.path[..], &mut data)
    }

    pub fn delete(&self,key:&str){
        let mut data = self.read(&self.path[..]);
        data.remove(key);
        self.write(&self.path[..], &mut data)
    }
}

impl RW for Cache{}