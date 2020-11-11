use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub count: i64,
    pub data: T,
}
