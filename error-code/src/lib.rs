pub struct ErrorInfo<T> {
  pub app_code: T,
  pub code: String,
  pub client_msg: String,
  pub server_msg: String,
}

pub trait ToErrorInfo {
  type T;
  fn to_error_info(&self) -> ErrorInfo<Self::T>;
}