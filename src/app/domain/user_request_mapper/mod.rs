use super::models::user_request::UserRequest;

pub fn to_drom(user_request: &UserRequest) -> UserRequest {
    user_request.clone()
}

pub fn to_avito(user_request: &UserRequest) -> UserRequest {
    UserRequest {
        id: user_request.id,
        firm: avito_firm(user_request.firm.as_str()),
        model: user_request.model.clone(),
        avito_model_hash: user_request.avito_model_hash.clone(),
        avito_filter_hash: user_request.avito_filter_hash.clone(),
    }
}

fn avito_firm(original_firm: &str) -> String {
    match original_firm {
        "3-series" => "".to_string(),
        _default => original_firm.to_string()
    }
}

pub fn model_from_avito(original_firm: &str) -> String {
    match original_firm {
        "3_seriya" => "3-series".to_string(),
        _default => original_firm.to_string()
    }
}