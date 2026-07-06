#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RabbitMqSettings {
    pub url: String,
    pub queue: String,
    pub prefetch: u16,
}
