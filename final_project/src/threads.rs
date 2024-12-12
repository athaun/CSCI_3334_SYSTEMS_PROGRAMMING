use std::{sync::mpsc, thread, time::{Duration, Instant}};
use crate::status::SiteStatus;
use crate::site_defs::Site;

pub struct SiteChecker {
    max_threads: usize,
}

impl SiteChecker {
    pub fn new(max_threads: usize) -> Self {
        Self {
            max_threads: max_threads
        }
    }

    pub fn check_websites(&self, sites: &mut Vec<Site>) -> Vec<SiteStatus> {
        let (transmitter, receiver) = mpsc::channel();
        let mut threads: Vec<thread::JoinHandle<()>> = vec![];
        let mut active_threads = 0;

        for site in sites {
            while active_threads >= self.max_threads {
                // Pop a thread from the threads vector
                if let Some(t) = threads.pop() {
                    // Wait for the thread to finish execution
                    if let Err(e) = t.join() {
                        eprintln!("Thread join error: {:?}", e);
                        return receiver.iter().collect();
                    }
                    active_threads -= 1;
                }
            }

            let transmitter = transmitter.clone();
            let url = site.url.clone();
            let timeout = site.config.request_timeout;
            let max_retries = site.config.max_retries;

            let t = thread::spawn(move || {
                let mut retries = 0;
                loop {
                    let status = SiteChecker::check_website(&url, timeout);
                    transmitter.send(status.clone()).unwrap();
                    if status.status.is_ok() || retries >= max_retries {
                        break;
                    }
                    retries += 1;
                }
            });

            threads.push(t);
            active_threads += 1;
        }

        for t in threads {
            if let Err(e) = t.join() {
                eprintln!("Thread join error: {:?}", e);
                return receiver.iter().collect();
            }
        }

        drop(transmitter);

        return receiver.iter().collect();
    }

    fn check_website(url: &str, timeout: Duration) -> SiteStatus {
        let start = Instant::now();
        let response = ureq::get(url).timeout(timeout).call();
        let response_time = start.elapsed();

        let status = match response {
            Ok(resp) => Ok(resp.status()),
            Err(err) => Err(err.to_string()),
        };

        SiteStatus::new(url.to_string(), status, response_time)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use crate::site_config::SiteConfig;

    #[test]
    fn test_check_website_success() {
        let url = "https://www.example.com";
        let timeout = Duration::from_secs(5);
        let status = SiteChecker::check_website(url, timeout);
        assert!(status.status.is_ok());
    }

    #[test]
    fn test_check_website_failure() {
        let url = "https://invalid.website.url";
        let timeout = Duration::from_secs(5);
        let status = SiteChecker::check_website(url, timeout);
        assert!(status.status.is_err());
    }

    #[test]
    fn test_check_websites() {
        let mut sites = vec![
            Site {
                url: "https://www.example.com".to_string(),
                config: SiteConfig {
                    request_timeout: Duration::from_secs(5),
                    max_retries: 3,
                },
                status: SiteStatus::default()
            },
            Site {
                url: "https://www.rust-lang.org".to_string(),
                config: SiteConfig {
                    request_timeout: Duration::from_secs(5),
                    max_retries: 3,
                },
                status: SiteStatus::default()
            },
        ];

        let checker = SiteChecker::new(2);
        let expected_len = sites.len();
        let results = checker.check_websites(&mut sites);
        assert_eq!(results.len(), expected_len);
    }

    use crate::get_websites;

    #[test]
    fn test_check_websites_performance() {
        // Checks that the function can perform all tests concurrently and ensures that all responses are gathered.
        // 1 response per site minimum, and a maximum = sum of all site retries as configured in site_defs
        let mut sites = get_websites();

        let checker = SiteChecker::new(50);

        let expected_min_requests = sites.len();
        let expected_max_requests: usize = sites.iter().map(|site| (site.config.max_retries + 1) as usize).sum();

        let start = Instant::now();
        let results = checker.check_websites(&mut sites);
        let duration = start.elapsed();

        let total_requests = results.len();
        assert!(total_requests >= 50); // project requirement
        assert!(total_requests >= expected_min_requests);
        assert!(total_requests <= expected_max_requests);

        println!("Performance test completed in {:?}", duration);
    }

    #[test]
    fn test_check_website_timeout() {
        let url = "https://www.example.com";
        let timeout = Duration::from_millis(1); // Intentionally short timeout
        let status = SiteChecker::check_website(url, timeout);
        assert!(status.status.is_err());
    }   
}