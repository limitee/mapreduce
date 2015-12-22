#[macro_use]
extern crate easy_util;

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

fn main() {
    let dsn = "postgresql://postgres:1988lm@localhost/mapreduce";
    let my_pool:MyDbPool = MyDbPool::new(dsn, 1);
    let my_db = DataBase::new("main", Arc::new(my_pool));
    let create_time = time::get_time().sec;
    let name = "liming1";
    let salary = 200;
    let json_str = format!(r#"
        {{
            "name":"{}",
            "salary": {},
            "create_time": {}
        }}
    "#, name, salary, create_time);
    let table = my_db.get_table("emp").expect("table not exists.");
    table.save_by_str(&json_str, "{}");
}
