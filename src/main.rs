#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate easy_util;
use easy_util::RandUtil;

use std::rc::Rc;

use std::sync::Mutex;
use std::sync::Arc;
use std::sync::mpsc::{channel, Sender};
use std::collections::BTreeMap;

use std::io;
use std::io::prelude::*;
use std::fs::File;

extern crate rustc_serialize;
use self::rustc_serialize::json::Json;
use self::rustc_serialize::json::ToJson;
use std::str::FromStr;

extern crate mapreduce;
use mapreduce::dc::MyDbPool;
use mapreduce::dc::DataBase;

use std::thread;

extern crate time;

lazy_static! {
    pub static ref VEC_KEYWORD:Vec<String> = vec!["test".to_string(), "测试".to_string()];
    pub static ref VEC_APPV:Vec<String> = vec!["a".to_string(), "b".to_string()];
    pub static ref VEC_SDKV:Vec<String> = vec!["sdk1.1".to_string(), "sdk1.2".to_string()];
    pub static ref VEC_SUBCHANNEL:Vec<String> = vec!["s1".to_string(), "s2".to_string()];
    pub static ref VEC_CREA:Vec<String> = vec!["c1".to_string(), "c2".to_string()];
    pub static ref VEC_MODEL:Vec<String> = vec!["model_a".to_string(), "model_b".to_string()];
    pub static ref VEC_NET:Vec<String> = vec!["net_a".to_string(), "net_b".to_string()];
    pub static ref VEC_BRAND:Vec<String> = vec!["xiaomi".to_string(), "huawei".to_string()];
}

fn get_string(vec:&Vec<String>) -> String {
    let length = vec.len() as i32;
    let random = RandUtil::get_int(0, length) as usize;
    vec.get(random).unwrap().clone()
}

pub fn get_time() -> i64 {
    let now = time::get_time();
    now.sec*1000 + (now.nsec/1000000) as i64
}

pub fn get_raw() -> Result<String, i32> {
    let app_id = 55406;
    let date = 20151224;
    let device_id = RandUtil::get_int(1, 100);
    let raw_type = RandUtil::get_int(0, 14);
    let mut p_value = 0;
    if raw_type == 12 || raw_type == 13 {
        p_value = RandUtil::get_int(1, 50);
    }
    let mut rtype = raw_type;
    if rtype == 2 {
        if RandUtil::get_int(0, 2) == 0 {
            rtype = 0;
        }
    }
    let media = RandUtil::get_int(1, 6);
    let placement = RandUtil::get_int(1, 6);
    let create_time = time::get_time().sec;
    let tkstamp = create_time - RandUtil::get_int(10, 30) as i64;
    let keyword = get_string(&VEC_KEYWORD);
    let source_type = RandUtil::get_int(0, 2);
    let appv = get_string(&VEC_APPV);
    let subchannel = get_string(&VEC_SUBCHANNEL);
    let crea = get_string(&VEC_CREA);
    let region_id = RandUtil::get_int(1, 100);
    let brand = get_string(&VEC_BRAND);
    let sdkv = get_string(&VEC_SDKV);
    let net = get_string(&VEC_NET);
    let model = get_string(&VEC_MODEL);
    let extra = RandUtil::get_int(0, 2);
    let str = format!(r#"
        {{
            "app_id": {},
            "date": {},
            "device_id": {},
            "type": {},
            "rtype": {},
            "media": {},
            "placement": {},
            "created_at": {},
            "tkStamp": {},
            "accessed_at": {},
            "keyword": "{}",
            "source_type": {},
            "appv": "{}",
            "subchannel":"{}",
            "creative":"{}",
            "brand":"{}",
            "sdkv":"{}",
            "net":"{}",
            "model":"{}",
            "region_id": {},
            "extra": {},
            "purchase_value": {}
        }}
    "#, app_id, date, device_id, raw_type, rtype, media, placement, create_time, tkstamp, create_time, keyword, source_type, appv, subchannel, crea, brand, sdkv, net, model, region_id, extra, p_value);
    Result::Ok(str)
}

fn main() {
    let dsn = "postgresql://postgres:1988lm@localhost/mapreduce";
    let my_pool:MyDbPool = MyDbPool::new(dsn, 25);
    let my_db = DataBase::new("main", Arc::new(my_pool));
    let create_time = time::get_time().sec;
    let name = "liming";
    let salary = 1200;
    let emp_type = 1;
    let json_str = format!(r#"
        {{
            "name":"{}",
            "salary": {},
            "create_time": {},
            "type": {}
        }}
    "#, name, salary, create_time, emp_type);
    //let table = my_db.get_table("emp").expect("table not exists.");
    //table.save_by_str(&json_str, "{}");
    let thread_count = 20;
    let save_count = 100000;
    let start = get_time();
    let arc_db = Arc::new(my_db);
    let threads: Vec<_> = (0..thread_count).map(|i| {
        let arc_db = arc_db.clone();
        thread::spawn(move || {
            let raw_table = arc_db.get_table("raw").unwrap();
            for _ in 0..save_count {
                let raw = get_raw().unwrap();
                raw_table.save_by_str(&raw, "{}");
            }
        })
    }).collect();
    let _: Vec<_> = threads.into_iter().map(|thread| {
        thread.join()
    }).collect();
    let end = get_time();
    let used = (end - start)/1000;
    let average = (thread_count*save_count)/used;
    println!("{} data per s. used {} s. {} threads.", average, used, thread_count);
}
