#[derive(Clone, PartialEq, Debug)]
pub struct SiteConfig {
    pub request_timeout: std::time::Duration,
    pub max_retries: u32,
}

impl SiteConfig {
    pub fn default() -> Self {
        Self {
            request_timeout: std::time::Duration::from_secs(5),
            max_retries: 3,
        }
    }

    pub fn custom(request_timeout: f32, max_retries: u32) -> Self {
        Self {
            request_timeout: std::time::Duration::from_secs(request_timeout as u64),
            max_retries,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = SiteConfig::default();
        assert_eq!(config.request_timeout, std::time::Duration::from_secs(5));
        assert_eq!(config.max_retries, 3);
    }

    #[test]
    fn test_custom_config() {
        let config = SiteConfig::custom(10.0, 5);
        assert_eq!(config.request_timeout, std::time::Duration::from_secs(10));
        assert_eq!(config.max_retries, 5);
    }
}
