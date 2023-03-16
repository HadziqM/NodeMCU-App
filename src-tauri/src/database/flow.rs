use super::{DB,MyErr};
use serde::Serialize;
use chrono::{DateTime, Utc,NaiveDateTime};
use chrono::serde::ts_seconds;

#[derive(Debug,sqlx::FromRow,PartialEq)]
pub struct FlowSens {
    pub flow:f32,
    pub created_at:NaiveDateTime
}
#[derive(Debug,Serialize)]
struct FlowSerial{
    data:f32,
    #[serde(with = "ts_seconds")]
    date:DateTime<Utc>
}
impl FlowSens{
    pub fn to_serial(&self)->FlowSerial{
        let date = DateTime::<Utc>::from_utc(self.created_at, Utc);
        FlowSerial { data: self.flow, date }
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

#[derive(Debug,Serialize)]
pub struct FlowRate {
    pub rate: f32,
    pub total:f32
}

#[derive(Debug,Serialize)]
struct DummyData<'a> {
    vec: &'a [FlowSerial]
}

#[derive(Debug,Serialize)]
pub struct SerialOut {
    rise: String,
    fall:String,
    status:String,
    rate:String
}

impl SerialOut {
    pub async fn serialize()->Result<String,MyErr>{
        let raw_rise = crate::FLOW_RISE.lock().await.iter().map(|x|x.to_serial()).collect::<Vec<_>>();
        let raw_fall = crate::FLOW_FALL.lock().await.iter().map(|x|x.to_serial()).collect::<Vec<_>>();
        let raw_status = crate::FLOW_STATUS.lock().await;
        let raw_rate = crate::FLOW_TOTAL.lock().await;
        let rise = serde_json::to_string(&DummyData{vec:&raw_rise})?;
        let fall = serde_json::to_string(&DummyData{vec:&raw_fall})?;
        let status = serde_json::to_string(&*raw_status)?;
        let rate = serde_json::to_string(&*raw_rate)?;
        Ok(serde_json::to_string(&Self{rise,fall,status,rate})?)
    }
}


impl DB {
    async fn rise(&self)->Result<Vec<FlowSens>,MyErr>{
        Ok(sqlx::query_as::<_,FlowSens>("select flow,created_at from flow_Sens where sens_id = 1 limit 20 sort by created_at asc").fetch_all(&self.pool).await?)
    }
    async fn fall(&self)->Result<Vec<FlowSens>,MyErr>{
        Ok(sqlx::query_as::<_,FlowSens>("select flow,created_at from flow_Sens where sens_id = 2 limit 20").fetch_all(&self.pool).await?)
    }
    async fn rate(fall:&Vec<FlowSens>,rise:&Vec<FlowSens>)->Option<()>{
        let mut flow = crate::FLOW_TOTAL.lock().await;
        let sum = |x:&Vec<FlowSens>|x.iter().map(|x|x.flow).sum::<f32>();
        let vector = sum(fall) - sum(rise);
        let time = fall.last()?.created_at.timestamp() - fall.first()?.created_at.timestamp();
        let rate = vector / time as f32 * 60.0;
        *flow = Box::new(FlowRate{rate,total:flow.total + vector});
        Some(())
    }
    pub async fn paralel(&self)->Result<(),MyErr>{
        let mut status = crate::FLOW_STATUS.lock().await;
        let mut rise = crate::FLOW_RISE.lock().await;
        let mut fall = crate::FLOW_FALL.lock().await;
        let mut r_stat = true;
        let mut f_stat = true;
        let new_rise = self.rise().await?;
        let new_fall = self.fall().await?;
        if *rise == new_rise{
            r_stat = false
        }
        if *fall == new_fall{
            f_stat = false
        }
        Self::rate(&new_fall, &new_rise).await.ok_or(MyErr::Custom("the data is empty".to_owned()))?;
        *rise = new_rise;
        *fall = new_fall;
        *status = Box::new(FLowStatus { rise: r_stat, fall: f_stat });
        Ok(())
    }
}
