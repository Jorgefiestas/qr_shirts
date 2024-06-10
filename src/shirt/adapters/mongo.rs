use crate::shirt::{port::Repository, Shirt, ShirtError};

pub struct Mongo {}

impl Mongo {
    pub fn new() -> Self {
        Mongo {}
    }
}

impl Repository for Mongo {
    fn create_shirt(&self, shirt: &Shirt) -> Result<(), ShirtError> {
        todo!()
    }

    fn update_shirt(&self, id: &str, owner_id: i32, url: &str) -> Result<(), ShirtError> {
        todo!()
    }

    fn get_shirt_by_id(&self, id: &str) -> Result<Shirt, ShirtError> {
        todo!()
    }

    fn get_shirts_by_owner(&self, owner_id: i32) -> Result<Vec<Shirt>, ShirtError> {
        todo!()
    }

    fn get_id_from_code(&self, reedem_code: &str) -> Result<String, ShirtError> {
        todo!()
    }
}
