use std::rc::Rc;
use std::thread;
use std::sync::{Arc, Mutex};

extern crate easydb;
use self::easydb::Column;
use self::easydb::Table;
use self::easydb::DbPool;

use std::collections::BTreeMap;

extern crate rustc_serialize;
use self::rustc_serialize::json::Json;
use self::rustc_serialize::json::ToJson;

extern crate postgres;
use self::postgres::{Connection, SslMode};
use self::postgres::types::Type;

extern crate rand;
use self::rand::distributions::{IndependentSample, Range};

pub struct MyDbPool {
    dsn:String,
    conns:Vec<Mutex<Connection>>,
}

impl MyDbPool {

    pub fn new(dsn:&str, size:u32) -> MyDbPool {
        let mut conns = vec![];
        for i in 0..size {
            let conn = match Connection::connect(dsn, &SslMode::None) {
                Ok(conn) => conn,
                Err(e) => {
                    println!("Connection error: {}", e);
                    break;
                }
            };
            conns.push(Mutex::new(conn));
        }
        MyDbPool {
            dsn:dsn.to_string(),
            conns:conns,
        }
    }

}

impl DbPool for MyDbPool {

    fn execute(&self, sql:&str) -> Json {
        //println!("{}", sql);
        let between = Range::new(0, self.conns.len());
        let mut rng = rand::thread_rng();
        let rand_int = between.ind_sample(&mut rng);
        let conn = self.conns[rand_int].lock().unwrap();
        let stmt = conn.prepare(&sql).unwrap();
        let rows = stmt.query(&[]).unwrap();
        let mut back_obj = BTreeMap::new();
        let mut data:Vec<Json> = Vec::new();
        for row in &rows {
            let mut row_map = BTreeMap::new();
            let columns = row.columns();
            for column in columns {
                let name = column.name();
                match *column.type_() {
                    Type::Int4 => {
                        let value:i32 = row.get(name);
                        row_map.insert(name.to_string(), value.to_json());
                    },
                    Type::Int8 => {
                        let value:i64 = row.get(name);
                        row_map.insert(name.to_string(), value.to_json());
                    },
                    Type::Timestamp => {

                    },
                    _ => {
                        let value:String = row.get(name);
                        row_map.insert(name.to_string(), value.to_json());
                    },
                }
            }
            data.push(row_map.to_json());
        }
        back_obj.insert("data".to_string(), data.to_json());
        back_obj.insert("rows".to_string(), rows.len().to_json());
        back_obj.to_json()
    }

}

pub struct DataBase<T> {
    pub name:String,
    pub table_list:BTreeMap<String, Table<T>>,
    pub dc:Arc<T>,   //data center
}

impl<T:DbPool> DataBase<T> {

    fn get_table_define(name:&str, vec:Vec<Column>, dc:Arc<T>) -> Table<T>
    {
        let mut map = BTreeMap::new();
        for col in vec {
            map.insert(col.name.clone(), col);
        }
        Table::new(name, map, dc)
    }

    pub fn new(name:&str, dc:Arc<T>) -> DataBase<T>
    {
        let mut table_list = BTreeMap::new();
        {
            let dc = dc.clone();
            let vec = vec![
                Column::new("id", "serial", -1, "primary key", false),
                Column::new("name", "varchar", 80, "not null", true),
                Column::new("salary", "integer", -1, "default 0", false),
                Column::new("create_time", "bigint", -1, "default -1", false),
                Column::new("type", "integer", -1, "default -1", false),
                Column::new("version", "integer", -1, "default -1", false),
            ];
            let table = DataBase::get_table_define("emp", vec, dc);
            table_list.insert(table.name.clone(), table);
        }
        {
            let dc = dc.clone();
            let vec = vec![
                Column::new("id", "serial", -1, "primary key", false),
                Column::new("app_id", "integer", -1, "default 0", false),
                Column::new("device_id", "integer", -1, "default 0", false),
                Column::new("date", "integer", -1, "", false),
                Column::new("type", "integer", -1, "default 0", false),
                Column::new("rtype", "integer", -1, "default 0", false),
                Column::new("media", "integer", -1, "default 0", false),
                Column::new("placement", "integer", -1, "default 0", false),
                Column::new("type_times", "integer", -1, "default 0", false),
                Column::new("event_interval", "integer", -1, "default 0", false),
                Column::new("uid", "varchar", 20, "default ''", false),
                Column::new("created_at", "integer", -1, "default 0", false),
                Column::new("accessed_at", "integer", -1, "default 0", false),
                Column::new("it", "integer", -1, "default 0", false),
                Column::new("at", "integer", -1, "default 0", false),
                Column::new("tkStamp", "integer", -1, "default 0", false),
                Column::new("ch", "varchar", 30, "default ''", false),
                Column::new("sdkv", "varchar", 20, "default ''", false),
                Column::new("appv", "varchar", 20, "default ''", false),
                Column::new("osv", "varchar", 10, "default ''", false),
                Column::new("os", "varchar", 20, "default ''", false),
                Column::new("brand", "varchar", 20, "default ''", false),
                Column::new("model", "varchar", 20, "default ''", false),
                Column::new("net", "varchar", 20, "default ''", false),
                Column::new("mcc", "varchar", 10, "default ''", false),
                Column::new("mnc", "varchar", 10, "default ''", false),
                Column::new("ap_mac", "varchar", 20, "default ''", false),
                Column::new("is_jailbreak", "integer", -1, "default 0", false),
                Column::new("is_root", "integer", -1, "default 0", false),
                Column::new("ad_tracked", "integer", -1, "default 0", false),
                Column::new("dev_name", "varchar", 100, "default ''", false),
                Column::new("package_name", "varchar", 100, "default ''", false),
                Column::new("keyword", "varchar", 50, "default ''", false),
                Column::new("campaign", "varchar", 20, "default ''", false),
                Column::new("creative", "varchar", 50, "default ''", false),
                Column::new("media_appkey", "varchar", 255, "default ''", false),
                Column::new("media_advertiserid", "varchar", 255, "default ''", false),
                Column::new("media_appid", "varchar", 255, "default ''", false),
                Column::new("media_custom", "varchar", 255, "default ''", false),
                Column::new("site_ip", "integer", -1, "default 0", false),
                Column::new("track_ip", "integer", -1, "default 0", false),
                Column::new("region_id", "integer", -1, "default 0", false),
                Column::new("site_ua", "varchar", 50, "default ''", false),
                Column::new("track_ua", "varchar", 50, "default ''", false),
                Column::new("source_type", "integer", -1, "default 0", false),
                Column::new("cheat_type", "integer", -1, "default 0", false),
                Column::new("ascription_type", "integer", -1, "default 0", false),
                Column::new("match_type", "integer", -1, "default 0", false),
                Column::new("purchase_currency", "integer", -1, "default 0", false),
                Column::new("purchase_value", "integer", -1, "default 0", false),
                Column::new("is_last_click", "integer", -1, "default 0", false),
                Column::new("ua", "varchar", 255, "default ''", false),
                Column::new("subchannel", "varchar", 255, "default ''", false),
                Column::new("extra", "integer", -1, "default 0", false),
            ];
            let table = DataBase::get_table_define("raw", vec, dc);
            table_list.insert(table.name.clone(), table);
        }
        {
            let dc = dc.clone();
            let vec = vec![
                Column::new("id", "serial", -1, "primary key", false),
                Column::new("app_id", "integer", -1, "default 0", false),
                Column::new("device_id", "integer", -1, "default 0", false),
                Column::new("count", "integer", -1, "default 0", false),
            ];
            let table = DataBase::get_table_define("raw_tmp", vec, dc);
            table_list.insert(table.name.clone(), table);
        }

        for (name, table) in table_list.iter() {
            println!("{}", table.to_ddl_string());
        }
        DataBase {
            name:name.to_string(),
            table_list:table_list,
            dc:dc,
        }
    }

    pub fn get_table(&self, name:&str) -> Option<&Table<T>>
    {
        self.table_list.get(name)
    }

    pub fn execute(&self, sql:&str) -> Result<Json, i32> {
        Result::Ok(self.dc.execute(sql)) 
    }
}
