// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code)]

pub mod database;

use async_mutex::Mutex;
use database::{flow::{FlowSens,FLowStatus},DB,MyErr};
use lazy_static::lazy_static;

lazy_static!{
    static ref FLOW_RISE:Mutex<Vec<FlowSens>> = Mutex::new(Vec::new());
    static ref FLOW_FALL:Mutex<Vec<FlowSens>> = Mutex::new(Vec::new());
    static ref FLOW_STATUS:Mutex<Box<FLowStatus>> = Mutex::new(Box::new(FLowStatus::default()));
}
async fn paralel(db:&DB)->Result<(),MyErr>{
    let mut status = FLOW_STATUS.lock().await;
    let mut rise = FLOW_RISE.lock().await;
    let mut fall = FLOW_FALL.lock().await;
    let mut r_stat = true;
    let mut f_stat = true;
    let new_rise = db.rise().await?;
    let new_fall = db.fall().await?;
    if *rise == new_rise{
        r_stat = false
    }
    if *fall == new_fall{
        f_stat = false
    }
    *rise = new_rise;
    *fall = new_fall;
    *status = Box::new(FLowStatus { rise: r_stat, fall: f_stat });
    Ok(())
}
#[tauri::command]
async fn database() -> String {
    let db = match DB::new().await{
        Err(why)=>{return format!("Error When Connecting : {why:?}");}
        Ok(x)=>x
    };
    tokio::spawn(async move{
        loop {
            tokio::time::sleep(std::time::Duration::new(10, 0)).await;
            if let Err(why) = paralel(&db).await{
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
