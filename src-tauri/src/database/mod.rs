pub mod flow;

#[derive(Debug)]
pub enum MyErr{
    Sqlx(sqlx::Error),
    Tokio(tokio::io::Error),
    Tauri(tauri::Error),
    Serde(serde_json::Error),
    Custom(String),
}

impl std::error::Error for MyErr {}

impl From<sqlx::Error> for MyErr {
    fn from(value: sqlx::Error) -> Self {
        MyErr::Sqlx(value)
    }
}
impl From<tokio::io::Error> for MyErr {
    fn from(value: tokio::io::Error) -> Self {
        MyErr::Tokio(value)
    }
}
impl From<tauri::Error> for MyErr {
    fn from(value: tauri::Error) -> Self {
        MyErr::Tauri(value)
    }
}
impl From<serde_json::Error> for MyErr {
    fn from(value: serde_json::Error) -> Self {
        MyErr::Serde(value)
    }
}
impl std::fmt::Display for MyErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Self::Sqlx(x)=>x.fmt(f),
            Self::Tokio(x)=>x.fmt(f),
            Self::Tauri(x)=>x.fmt(f),
            Self::Custom(x)=>x.fmt(f),
            Self::Serde(x)=>x.fmt(f)
        }
    }
}
#[derive(Clone)]
pub struct DB{
    pub pool:sqlx::Pool<sqlx::postgres::Postgres>
}
impl DB {
    pub async fn new()->Result<Self,MyErr>{
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            //read only user in my vps for demo
            .connect("postgres://postgres:Qwerty333@103.67.186.22/development").await?;
        Ok(Self { pool })
    }
    pub async fn close(&mut self){
        self.pool.close().await
    }
}
