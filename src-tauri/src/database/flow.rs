use super::{DB,MyErr};
use serde::Serialize;
use chrono::{DateTime, Utc,NaiveDateTime};
use chrono::serde::ts_seconds;

#[derive(Debug,sqlx::FromRow,PartialEq,Clone)]
pub struct FlowSens {
    pub flow:f32,
    pub created_at:NaiveDateTime
}
#[derive(Debug,Serialize,Clone)]
struct FlowSerial{
    data:f32,
    #[serde(with = "ts_seconds")]
    date:DateTime<Utc>
}
impl FlowSens{
    fn to_serial(&self)->FlowSerial{
        let date = DateTime::<Utc>::from_utc(self.created_at, Utc);
        FlowSerial { data: self.flow, date }
    }
}


#[derive(Debug,Serialize,Clone)]
pub struct FLowStatus {
    pub rise: bool,
    pub fall:bool
}

impl Default for FLowStatus {
    fn default() -> Self {
        Self { rise: false, fall: false }
    }
}

#[derive(Debug,Serialize,Clone)]
pub struct FlowRate {
    pub rate: f32,
    pub total:f32
}

impl Default for FlowRate {
    fn default() -> Self {
        Self { rate: 0.0, total: 0.0 }
    }
}
#[derive(Debug,Serialize,Clone)]
struct DummyData {
    vec: Vec<FlowSerial>
}

#[derive(Debug,Serialize)]
pub struct SerialOut {
    rise: DummyData,
    fall: DummyData,
    status:FLowStatus,
    rate:FlowRate
}

impl SerialOut {
    fn sensor_data(data:&Vec<FlowSens>)->Result<DummyData,MyErr>{
        let sens = data.iter().map(|x|x.to_serial()).collect::<Vec<_>>();
        Ok(DummyData{vec:sens})
    }
    pub async fn serialize()->Result<String,MyErr>{
        let rise = Self::sensor_data(&*crate::FLOW_RISE.lock().await)?;
        let fall = Self::sensor_data(&*crate::FLOW_FALL.lock().await)?;
        let status = *crate::FLOW_STATUS.lock().await.clone();
        let rate = *crate::FLOW_TOTAL.lock().await.clone();
        Ok(serde_json::to_string(&Self{rise,fall,status,rate})?)
    }
    pub fn serial_data(data:&[Vec<FlowSens> ;2])->Result<String,MyErr>{
        let rise = Self::sensor_data(&data[0])?;
        let fall = Self::sensor_data(&data[1])?;
        let status = FLowStatus::default();
        let rate = FlowRate::default();
        Ok(serde_json::to_string(&Self{rise,fall,status,rate})?)
    }
    pub fn interval_data(data:&[Vec<FlowSens> ;2])->Result<String,MyErr>{
        let rise = Self::sensor_data(&vec![])?;
        let fall = rise.clone();
        let status = FLowStatus::default();
        let rate = DB::flowrate(data).ok_or(MyErr::Custom("no data found".to_owned()))?;
        Ok(serde_json::to_string(&Self{rise,fall,status,rate})?)
    }
}


impl DB {
    fn data_clone(data:&Vec<FlowSens>)->Vec<FlowSens>{
        data.iter().map(|x|FlowSens{flow:0.0,created_at:x.created_at}).collect::<Vec<_>>()
    }
    pub async fn data(&self,start:Option<NaiveDateTime>)->Result<[Vec<FlowSens> ;2],MyErr>{
        let end;
        let star;
        let fall = if let Some(x) = start{
            star = x;
            end = NaiveDateTime::from_timestamp_millis((x.timestamp() + 60)*1000)
                .ok_or(MyErr::Custom("invalid_timestamp".to_owned()))?;
            sqlx::query_as::<_,FlowSens>("select flow,created_at from flow_sens where sens_id = 1 and created_at >= $1 and created_at <= $2 
            order by created_at desc limit 20")
                .bind(x).bind(end).fetch_all(&self.pool).await?
        }else {
            let second = sqlx::query_as::<_,FlowSens>("select flow,created_at from flow_sens where sens_id = 1 order by created_at desc limit 20")
                .fetch_all(&self.pool).await?;
            star = second.last()
                .ok_or(MyErr::Custom("data is empty".to_owned()))?.created_at;
            end = second.last()
                .ok_or(MyErr::Custom("data is empty".to_owned()))?.created_at;
            second
        };
        let rise = match sqlx::query_as::<_,FlowSens>("select flow,created_at from flow_sens where sens_id = 1 and created_at >= $1 and created_at <= $2 order by created_at desc limit 20")
                .bind(star).bind(end).fetch_all(&self.pool).await{
                Ok(x)=>x,
                Err(_)=>DB::data_clone(&fall)
    };
        Ok([rise,fall])
    }
    pub async fn interval(&self,start:i32,stop:i32)->Result<[Vec<FlowSens> ;2],MyErr>{
        let end = NaiveDateTime::from_timestamp_millis(stop as i64 *1000)
            .ok_or(MyErr::Custom("invalid timestamp".to_owned()))?;
        let star = NaiveDateTime::from_timestamp_millis(start as i64 *1000)
            .ok_or(MyErr::Custom("invalid timestamp".to_owned()))?;
        let fall = sqlx::query_as::<_,FlowSens>("select flow,created_at from flow_sens where sens_id = 1 and created_at >= $1 and created_at <= $2 order by created_at desc limit 20")
                .bind(star).bind(end).fetch_all(&self.pool).await?;
        let rise = match sqlx::query_as::<_,FlowSens>("select flow,created_at from flow_sens where sens_id = 1 and created_at >= $1 and created_at <= $2 order by created_at desc limit 20")
                .bind(star).bind(end).fetch_all(&self.pool).await{
                Ok(x)=>x,
                Err(_)=>DB::data_clone(&fall)
            };
        Ok([rise,fall])
    }
    fn flowrate(data:&[Vec<FlowSens> ;2])-> Option<FlowRate>{
        let sum = |x:&Vec<FlowSens>|x.iter().map(|x|x.flow).sum::<f32>();
        let vector = sum(&data[1]) - sum(&data[0]);
        let time = data[1].first()?.created_at.timestamp() - data[1].last()?.created_at.timestamp();
        let rate = vector / time as f32 * 60.0;
        Some(FlowRate { rate, total: vector })
    }
    async fn rate(data:&[Vec<FlowSens> ;2])->Option<()>{
        let rate = Self::flowrate(data)?;
        let mut flow = crate::FLOW_TOTAL.lock().await;
        *flow = Box::new(FlowRate{rate:rate.rate,total:flow.total + rate.total});
        Some(())
    }
    pub async fn paralel(&self)->Result<(),MyErr>{
        let mut status = crate::FLOW_STATUS.lock().await;
        let mut rise = crate::FLOW_RISE.lock().await;
        let mut fall = crate::FLOW_FALL.lock().await;
        let mut r_stat = true;
        let mut f_stat = true;
        let data = self.data(None).await?;
        if *rise == data[0]{
            r_stat = false
        }
        if *fall == data[1]{
            f_stat = false
        }
        *rise = data[0].clone();
        *fall = data[1].clone();
        *status = Box::new(FLowStatus { rise: r_stat, fall: f_stat });
        Self::rate(&data).await.ok_or(MyErr::Custom("the data is empty".to_owned()))?;
        Ok(())
    }
}
