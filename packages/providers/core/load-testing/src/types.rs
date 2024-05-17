use std::sync::Arc;

use tokio::sync::Mutex;

pub type Value = i64;
pub type Dataset = Vec<Value>;
pub type DataCollection = (
    Arc<Mutex<Dataset>>,
    Arc<Mutex<Dataset>>,
    Arc<Mutex<Dataset>>,
);
