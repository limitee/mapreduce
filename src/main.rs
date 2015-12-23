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

extern crate time;

fn get_raw() -> Result<String, i32> {
    let app_id = 55406;
    let date = "2015-12-23";
    let device_id = RandUtil::get_int(1, 10000);
    let raw_type = 1;
    let media = RandUtil::get_int(1, 50);
    let placement = RandUtil::get_int(1, 50);
    let str = format!(r#"
        {{
            "app_id": {},
            "date": "{}",
            "device_id": {},
            "type": {},
            "media": {},
            "placement": {}
        }}
    "#, app_id, date, device_id, raw_type, media, placement);
    Result::Ok(str)
}

fn main() {
    let dsn = "postgresql://postgres:1988lm@localhost/mapreduce";
    let my_pool:MyDbPool = MyDbPool::new(dsn, 1);
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
    let table = my_db.get_table("emp").expect("table not exists.");
    //table.save_by_str(&json_str, "{}");

    let raw_table = my_db.get_table("raw").expect("raw table not exists");
    for _ in 0..1000000 {
        let raw = get_raw().unwrap();
        raw_table.save_by_str(&raw, "{}");
    }
}
