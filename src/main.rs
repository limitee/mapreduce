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

fn main() {
    let dsn = "postgresql://postgres:1988lm@localhost/mapreduce";
    let my_pool:MyDbPool = MyDbPool::new(dsn, 1);
    let my_db = DataBase::new("main", Arc::new(my_pool));
}
