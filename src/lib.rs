/// 自定义类型简写
pub type R<T> = Result<T, Box<dyn std::error::Error>>;
