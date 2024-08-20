use crate::api::endpoints::ApiResponse;
use crate::r#const::BUILD_VERSION;

pub async fn build_version<'a>() -> ApiResponse<&'a str> {
    ApiResponse::Json(BUILD_VERSION)
}
