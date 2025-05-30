//  Simple shared in-memory storage using hashmap for user db

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use uuid::Uuid;

use crate::models::User;

pub type Db = Arc<Mutex<HashMap<Uuid, User>>>;

