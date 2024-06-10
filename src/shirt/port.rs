use super::{Shirt, ShirtError};

pub trait Repository {
    fn create_shirt(&self, shirt: &Shirt) -> Result<(), ShirtError>;

    fn update_shirt(&self, id: &str, owner_id: i32, url: &str) -> Result<(), ShirtError>;

    fn get_shirt_by_id(&self, id: &str) -> Result<Shirt, ShirtError>;
    fn get_redirect_url(&self, id: &str) -> Result<String, ShirtError>;
    fn get_id_from_code(&self, reedem_code: &str) -> Result<String, ShirtError>;
    fn get_shirts_by_owner(&self, owner_id: i32) -> Result<Vec<Shirt>, ShirtError>;
}
