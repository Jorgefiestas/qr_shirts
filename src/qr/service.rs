use std::collections::HashMap;

use askama::Template;
use uuid::Uuid;

use super::port::Repository;
use super::templates;
use super::{PageType, QrError, QrPage};

const DEFAULT_TEMPLATE: i32 = 1;

#[derive(Clone)]
pub struct Service<R: Repository> {
    repo: R,
}

impl<R: Repository> Service<R> {
    pub fn new(repo: R) -> Self {
        Service { repo }
    }

    pub async fn generate_qr(&self, id: &str) -> Result<(), QrError> {
        let default_page = QrPage {
            id: id.to_string(),
            template_id: DEFAULT_TEMPLATE,
            parameters: HashMap::new(),
        };
        let secret = Uuid::new_v4();

        self.repo.create_page(&default_page).await?;
        self.repo.create_secret(&id, secret).await?;
        Ok(())
    }

    pub async fn get_page(&self, id: &str) -> Result<PageType, QrError> {
        let qr_page = self.repo.get_page(id).await?;
        let parameters = &qr_page.parameters;

        if qr_page.template_id == 0 {
            let redirect_url = match parameters.get("redirect_url") {
                Some(url) => url.clone(),
                None => return Err(QrError::TemplateParameterNotFound),
            };
            return Ok(PageType::Redirect(redirect_url));
        }

        let html = match qr_page.template_id {
            //1 => templates::Profile::new(id, parameters)?.render()?,
            1 => templates::Quote::new_page(id, parameters)?.render()?,
            2 => templates::Profile::new_page(id, parameters)?.render()?,
            _ => return Err(QrError::InvalidTemplateID),
        };

        Ok(PageType::Html(html))
    }

    pub async fn get_edit_page(&self, secret: Uuid) -> Result<String, QrError> {
        let id = self.repo.get_id_from_secret(secret).await?;

        let qr_page = self.repo.get_page(&id).await?;
        let parameters = &qr_page.parameters;

        let html = match qr_page.template_id {
            1 => templates::Quote::new_edit(&id, parameters)?.render()?,
            2 => templates::Profile::new_edit(&id, parameters)?.render()?,
            _ => return Err(QrError::InvalidTemplateID),
        };

        Ok(html)
    }

    pub async fn update_page(&self, secret: Uuid, page: &QrPage) -> Result<(), QrError> {
        match self.repo.get_id_from_secret(secret).await? {
            id if id == page.id => self.repo.update_page(page).await,
            _ => Err(QrError::Unauthorized),
        }
    }
}
