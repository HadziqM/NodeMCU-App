// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code)]

pub mod database;

use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayMenuItem};
use tauri::{Manager, SystemTrayEvent};
use async_mutex::Mutex;
use database::{flow::{FlowSens,FLowStatus,FlowRate,SerialOut},DB};
use lazy_static::lazy_static;

lazy_static!{
    static ref FLOW_RISE:Mutex<Vec<FlowSens>> = Mutex::new(Vec::new());
    static ref FLOW_FALL:Mutex<Vec<FlowSens>> = Mutex::new(Vec::new());
    static ref FLOW_STATUS:Mutex<Box<FLowStatus>> = Mutex::new(Box::new(FLowStatus::default()));
    static ref FLOW_TOTAL:Mutex<Box<FlowRate>> = Mutex::new(Box::new(FlowRate::default()));
    static ref DBASE:Mutex<Option<DB>> = Mutex::new(None);
}

#[tauri::command]
async fn database() -> String {
    let mut dbase = DBASE.lock().await;
    let db = match DB::new().await{
        Err(why)=>{return format!("Error When Connecting : {why:?}");}
        Ok(x)=>x
    };
    *dbase = Some(db.clone());
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
#[tauri::command]
async fn data() ->String {
    match SerialOut::serialize().await{
        Ok(x) =>x,
        Err(why)=>{
            println!("error on serialize data: {why:?}");
            return format!("error with message: {why:?}");
        }
    }
}
#[tauri::command]
async fn search(start:i32)->String{
    let naive = chrono::NaiveDateTime::from_timestamp_millis(start as i64 * 1000).unwrap();
    let db = DBASE.lock().await.clone().unwrap();
    match db.data(Some(naive)).await{
        Ok(x) => {
            match SerialOut::serial_data(&x){
                Ok(y)=>y,
                Err(why)=>format!("error deserializing: {why:?}")
            }
        }
        Err(why)=>format!("error deserializing: {why:?}")
    }
}

#[tauri::command]
async fn interval(start:i32,stop:i32)->String{
    let db = DBASE.lock().await.clone().unwrap();
    match db.interval(start, stop).await{
        Ok(x) => {
            match SerialOut::interval_data(&x){
                Ok(y)=>y,
                Err(why)=>format!("error deserializing: {why:?}")
            }
        }
        Err(why)=>format!("error deserializing: {why:?}")
    }
}

fn main() {
     let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("hide".to_string(), "Hide/Show"))
        .add_item(CustomMenuItem::new(
            "window".to_string(),
            "Fullscreen/Window",
        ));
    let system_tray = SystemTray::new().with_menu(tray_menu);
    tauri::Builder::default()
        .system_tray(system_tray)
        .invoke_handler(tauri::generate_handler![database,data,search,interval])
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                match app.get_window("main") {
                    Some(window) => match window.is_visible().expect("winvis") {
                        true => {
                            // hide the window instead of closing due to processes not closing memory leak: https://github.com/tauri-apps/wry/issues/590
                            window.hide().expect("winhide");
                            // window.close().expect("winclose");
                            return;
                        }
                        false => window.show().expect("error"),
                    },
                    None => return,
                };
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "hide" => {
                    match app.get_window("main") {
                        Some(window) => match window.is_visible().expect("winvis") {
                            true => {
                                window.hide().expect("winhide");
                            }
                            false => window.show().expect("error"),
                        },
                        None => return,
                    };
                }
                "window" => {
                    let window = app.get_window("main").unwrap();
                    match window.is_fullscreen().expect("idk") {
                        true => window
                            .set_fullscreen(false)
                            .expect("windows cant be fullscreen"),
                        false => window
                            .set_fullscreen(true)
                            .expect("windows cant be fullscreen"),
                    }
                }
                "resize" => {
                    let window = app.get_window("main").unwrap();
                    match window.is_maximized().expect("idk") {
                        true => window.maximize().expect("window isnt resizable"),
                        false => window.unmaximize().expect("window isnt resizable"),
                    }
                }
                _ => {}
            },
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
