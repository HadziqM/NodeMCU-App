use super::{DB,MyErr};
use serde::Serialize;
use chrono::{DateTime, Utc,NaiveDateTime};
use chrono::serde::ts_seconds;

#[derive(Debug,sqlx::FromRow,PartialEq)]
pub struct FlowSens {
    pub data:f32,
    pub date:NaiveDateTime
}
#[derive(Serialize)]
struct FlowSerial{
    data:f32,
    #[serde(with = "ts_seconds")]
    date:DateTime<Utc>
}
impl FlowSens{
    pub fn serialize(&self)->String{
        let date = DateTime::<Utc>::from_utc(self.date, Utc);
        let ser = FlowSerial{data:self.data,date};
        serde_json::to_string(&ser).unwrap()
    }
}


#[derive(Debug,Serialize)]
pub struct FLowStatus {
    pub rise: bool,
    pub fall:bool
}

impl Default for FLowStatus {
    fn default() -> Self {
        Self { rise: false, fall: false }
    }
}

impl DB {
    pub async fn rise(&self)->Result<Vec<FlowSens>,MyErr>{
        Ok(sqlx::query_as::<_,FlowSens>("").fetch_all(&self.pool).await?)
    }
    pub async fn fall(&self)->Result<Vec<FlowSens>,MyErr>{
        Ok(sqlx::query_as::<_,FlowSens>("").fetch_all(&self.pool).await?)
    }
}
