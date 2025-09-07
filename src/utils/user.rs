use identicon_rs::error::IdenticonError;
use identicon_rs::Identicon;

use crate::result::CarbonResult;

pub fn generate_avatar(username: &str) -> CarbonResult<Vec<u8>> {
    let avatar = Identicon::new(username);
    avatar.export_png_data().map_err(|e| match e {
        IdenticonError::EncodeImageError => crate::result::CarbonError::SerializerError,
        IdenticonError::ThemeError(_) => crate::result::CarbonError::UserError {
            message: "Failed to generate avatar from username".to_string(), 
            code: 400
        },
        _ => crate::result::CarbonError::ExternalError { 
            message: "Failed to generate avatar".to_string(), 
            service: "IDENTICON_RS".to_string()
        }
    })
}