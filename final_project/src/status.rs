use serde::{Serialize, Deserialize};
use std::time::Duration;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SiteStatus {
    pub url: String,
    pub status: Result<u16, String>,
    pub response_time: Duration,
    pub timestamp: DateTime<Utc>,
}

impl SiteStatus {
    pub fn new(url: String, status: Result<u16, String>, response_time: Duration) -> Self {
        Self {
            url,
            status,
            response_time,
            timestamp: Utc::now(),
        }
    }

    pub fn default() -> Self {
        Self {
            url: String::new(),
            status: Err("Not checked".to_string()),
            response_time: Duration::new(0, 0),
            timestamp: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_site_status_new() {
        let url = "http://example.com".to_string();
        let status = Ok(200);
        let response_time = Duration::new(1, 0);
        let site_status = SiteStatus::new(url.clone(), status.clone(), response_time);

        assert_eq!(site_status.url, url);
        assert_eq!(site_status.status, status);
        assert_eq!(site_status.response_time, response_time);
    }

    #[test]
    fn test_site_status_default() {
        let site_status = SiteStatus::default();

        assert_eq!(site_status.url, "");
        assert_eq!(site_status.status, Err("Not checked".to_string()));
        assert_eq!(site_status.response_time, Duration::new(0, 0));
    }

    #[test]
    fn test_site_status_timestamp() {
        let url = "http://example.com".to_string();
        let status = Ok(200);
        let response_time = Duration::new(1, 0);
        let site_status = SiteStatus::new(url, status, response_time);

        assert!(site_status.timestamp <= Utc::now());
    }
}

