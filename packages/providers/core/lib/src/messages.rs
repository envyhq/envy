use std::collections::VecDeque;
use std::sync::{Arc, RwLock};

pub type Message = Vec<u8>;
pub type Messages = Arc<RwLock<VecDeque<Message>>>;
