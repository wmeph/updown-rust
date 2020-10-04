pub(crate) mod check;
pub(crate) mod downtime;
pub(crate) mod metric;

quick_error! {
    #[derive(Debug)]
    pub enum MessageError {
        RequestFailed( cause : reqwest::Error){from()}
        JsonFailed( cause : serde_json::Error){from()}
    }
}
