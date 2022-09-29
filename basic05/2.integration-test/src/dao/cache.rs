use std::collections::HashMap;
use std::sync::Arc;
use failure::Error;
use log::error;
use r2d2::Pool;
use r2d2_redis::redis::{Commands, RedisError,RedisResult};
use r2d2_redis::RedisConnectionManager;
use serde::{Deserialize, Serialize};

pub struct Cache<T>{
    uid: Option<String>,
    person: Option<T>,
    pool: Arc<Pool<RedisConnectionManager>>
}

impl <T> Cache<T> {

    pub fn new(uid:Option<String>,person: Option<T>,pool: Arc<Pool<RedisConnectionManager>>)->Self{
        Cache{
            uid:uid,
            person: person,
            pool: pool
        }
    }

    pub fn list<'de>(&self, key: &str,) -> Result<HashMap<String, String>,RedisError>
    {
        let mut conn = self.pool.get().unwrap();
        let result = conn.hgetall(key);
        result
    }

    pub fn get(
        &self,
        key:&str,
    )->Result<String, RedisError>{
        let field = self.uid.as_ref().unwrap().as_str();
        let mut conn = self.pool.get().unwrap();
        let result = conn.hget(key,field);
        result
    }

    pub fn create<'de>(
        &self,
        key: &str,
    ) ->Result<(),Error>
        where T: Serialize + Deserialize<'de>{
        let mut conn = self.pool.get().unwrap();
        if let Some(person) = &self.person {
            let field = self.uid.as_ref().unwrap().as_str();
            let value:String = serde_json::to_string(person).unwrap();
            let result:RedisResult<i32>= conn.hset(key, field, &value);
            if let Err(_) = result {
                error!("Redis hset Error");
                panic!("Redis hset Error");
            }
        };
        Ok(())
    }

    pub fn update<'de>(&self,key:&str)->Result<(), RedisError>
        where T: Serialize + Deserialize<'de>{
        let mut conn = self.pool.get().unwrap();

        if let Some(person) = &self.person {
            let field = self.uid.as_ref().unwrap().as_str();
            let _:RedisResult<i32> = conn.hdel(key,field);
            let value:String = serde_json::to_string(person).unwrap();
            let result:RedisResult<i32>= conn.hset(key, field, &value);
            if let Err(_) = result {
                panic!("Redis hset Error");
            }
        };
        Ok(())
    }

    pub fn delete(&self, key:&str)->Result<(),RedisError>{
        let field = self.uid.as_ref().unwrap().as_str();
        let mut conn = self.pool.get().unwrap();
        conn.hdel(key,field)
    }
}