// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code)]

pub mod database;

use async_mutex::Mutex;
use database::{flow::{FlowSens,FLowStatus,FlowRate},DB};
use lazy_static::lazy_static;

lazy_static!{
    static ref FLOW_RISE:Mutex<Vec<FlowSens>> = Mutex::new(Vec::new());
    static ref FLOW_FALL:Mutex<Vec<FlowSens>> = Mutex::new(Vec::new());
    static ref FLOW_STATUS:Mutex<Box<FLowStatus>> = Mutex::new(Box::new(FLowStatus::default()));
    static ref FLOW_TOTAL:Mutex<Box<FlowRate>> = Mutex::new(Box::new(FlowRate { rate: 0, total: 0.0 }));
}
#[tauri::command]
async fn database() -> String {
    let db = match DB::new().await{
        Err(why)=>{return format!("Error When Connecting : {why:?}");}
        Ok(x)=>x
    };
    tokio::spawn(async move{
        loop {
            tokio::time::sleep(std::time::Duration::new(3, 0)).await;
            if let Err(why) = db.paralel().await{
                println!("error on paralel thread: {why:?}")
            }
        }
    });
    return "success".to_owned();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![database])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
