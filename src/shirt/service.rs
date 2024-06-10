use super::port::Repository;
use super::{Shirt, ShirtError};

const LANDING_PAGE_URL: &str = "https://google.com";

pub struct Service<R: Repository> {
    repo: R,
}

impl<R: Repository> Service<R> {
    pub fn new(repo: R) -> Self {
        Service { repo }
    }

    pub fn claim_shirt(&self, id: &str, user_id: i32) -> Result<(), ShirtError> {
        let shirt = self.repo.get_shirt_by_id(id)?;

        match shirt.owner_id {
            None => self.repo.update_shirt(id, user_id, LANDING_PAGE_URL),
            Some(_) => Err(ShirtError::AlreadyOwned),
        }
    }

    pub fn update_url(&self, id: &str, user_id: i32, new_url: &str) -> Result<(), ShirtError> {
        let shirt = self.repo.get_shirt_by_id(id)?;

        match shirt.owner_id {
            Some(owner_id) if user_id == owner_id => self.repo.update_shirt(id, user_id, new_url),
            Some(_) => Err(ShirtError::Unauthorized),
            None => Err(ShirtError::HasNoOwner),
        }
    }

    pub fn get_id_from_code(&self, reedem_code: &str) -> Result<String, ShirtError> {
        self.repo.get_id_from_code(reedem_code)
    }

    pub fn get_shirts_by_owner(&self, owner_id: i32) -> Result<Vec<Shirt>, ShirtError> {
        self.repo.get_shirts_by_owner(owner_id)
    }
}
