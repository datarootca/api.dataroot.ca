use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::api::{
    config::get_config, 
    resources::state::dto::ResponseState,
    resources::categories::dto::ResponseCategory,
    resources::city::dto::ResponseCity,
    resources::group::dto::ResponseGroup,
    resources::article::dto::ResponseArticle,
    resources::event::dto::ResponseEvent,
};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Meta {
    pub count: u32,
    pub page: u32,
    pub pages: u32,
}
impl Meta {
    pub fn new(page: Option<u32>, count: Option<u32>, page_size: Option<u32>) -> Self {
        let config = get_config();

        let page = page.unwrap_or(1);
        let count = count.unwrap_or(1);
        let page_size = page_size.unwrap_or(config.page_size_default);

        let pages = ((count as f32) / (page_size as f32)).ceil() as u32;

        Self { count, page, pages }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[aliases(
    ApiResponseCategory = ApiResponse<ResponseCategory>,
    ApiResponseState = ApiResponse<ResponseState>,
    ApiResponseArticle = ApiResponse<ResponseArticle>,
    ApiResponseEvent = ApiResponse<ResponseEvent>,
    ApiResponseCity = ApiResponse<ResponseCity>,
    ApiResponseGroup = ApiResponse<ResponseGroup>,
)]
pub struct ApiResponse<T> {
    pub meta: Meta,
    pub records: Vec<T>,
}
impl<T> ApiResponse<T> {
    pub fn new(
        records: Vec<T>,
        page: Option<u32>,
        count: Option<u32>,
        page_size: Option<u32>,
    ) -> Self {
        Self {
            meta: Meta::new(page, count, page_size),
            records,
        }
    }
}
