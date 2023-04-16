use uuid::{Builder, Uuid};
use rand::prelude::*;
pub fn new_random_uuid_v4() -> Uuid {
    let mut data = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut data);
    let uuid = Builder::from_random_bytes(data).into_uuid();
    uuid
}