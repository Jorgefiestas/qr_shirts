use std::collections::HashMap;

use askama::Template;

use super::QrError;

fn get_with_error(map: &HashMap<String, String>, key: &str) -> Result<String, QrError> {
    match map.get(key) {
        Some(v) => Ok(v.clone()),
        None => Err(QrError::TemplateParameterNotFound),
    }
}

#[derive(Template)]
#[template(path = "page_templates/quote.html")]
pub struct Quote {
    shirt_id: String,
    edit_flag: bool,
    quote: String,
}

impl Quote {
    pub fn new_page(shirt_id: &str, parameters: &HashMap<String, String>) -> Result<Self, QrError> {
        Ok(Quote {
            shirt_id: shirt_id.to_string(),
            quote: get_with_error(parameters, "quote")?,
            edit_flag: false,
        })
    }

    pub fn new_edit(shirt_id: &str, parameters: &HashMap<String, String>) -> Result<Self, QrError> {
        let mut quote = Self::new_page(shirt_id, parameters)?;
        quote.edit_flag = true;
        Ok(quote)
    }
}

// TODO: Make macro to avoid redundant edit/page code

#[derive(Template)]
#[template(path = "page_templates/profile.html")]
pub struct Profile {
    shirt_id: String,
    edit_flag: bool,
    image_url: String,
    name: String,
    title: String,
    email: String,
    phone: String,
}

impl Profile {
    pub fn new_page(shirt_id: &str, parameters: &HashMap<String, String>) -> Result<Self, QrError> {
        Ok(Profile {
            shirt_id: shirt_id.to_string(),
            image_url: get_with_error(parameters, "image_url")?,
            name: get_with_error(parameters, "name")?,
            title: get_with_error(parameters, "title")?,
            email: get_with_error(parameters, "email")?,
            phone: get_with_error(parameters, "phone")?,
            edit_flag: false,
        })
    }

    pub fn new_edit(shirt_id: &str, parameters: &HashMap<String, String>) -> Result<Self, QrError> {
        let mut profile = Self::new_page(shirt_id, parameters)?;
        profile.edit_flag = true;
        Ok(profile)
    }
}
