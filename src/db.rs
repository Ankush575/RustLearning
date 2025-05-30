use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use uuid::Uuid;

use crate::models::User;

pub type Db = Arc<Mutex<HashMap<Uuid, User>>>;