use serde::Serialize;

#[derive(Serialize)]
pub struct PaginationMeta {
    pub total_items: i64,
    pub total_pages: i64,
    pub current_page: i64,
    pub page_size: i64,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum StandardResponse<T> {
    MessageOnly {
        status: bool,
        message: String,
    },
    WithData {
        status: bool,
        message: String,
        data: T,
    },
    WithDataAndPagination {
        status: bool,
        message: String,
        data: T,
        pagination: PaginationMeta,
    },
}

impl<T> StandardResponse<T> {
    pub fn success(message: &str) -> Self {
        StandardResponse::MessageOnly {
            status: true,
            message: message.to_string(),
        }
    }
    
    pub fn success_with_data(data: T, message: &str) -> Self {
        StandardResponse::WithData {
            status: true,
            message: message.to_string(),
            data,
        }
    }

    pub fn success_with_pagination(data: T, message: &str, pagination: PaginationMeta) -> Self {
        StandardResponse::WithDataAndPagination  {
            status: true,
            message: message.to_string(),
            data,
            pagination,
        }
    }

    pub fn error(message: &str) -> Self {
        StandardResponse::MessageOnly {
            status: false,
            message: message.to_string(),
        }
    }
}
