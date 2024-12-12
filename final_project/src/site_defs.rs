use crate::site_config::SiteConfig;
use crate::status::SiteStatus;

#[derive(PartialEq)]
pub struct Site {
    pub url: String,
    pub config: SiteConfig,
    pub status: SiteStatus,
}

impl Site {
    pub fn new(url: &str, config: SiteConfig) -> Site {
        Site {
            url: url.to_string(),
            config,
            status: SiteStatus::default(),
        }
    }

    pub fn default(url: &str) -> Site {
        Site::new(url, SiteConfig::default())
    }
}

pub fn get_websites() -> Vec<Site> {
    vec![
        Site::new(
            "https://www.github.com", 
            SiteConfig::custom(3.0, 2)
        ),
        Site::new(
            "https://www.wikipedia.org", 
            SiteConfig::custom(10.0, 4)
        ),
        Site::new(
            "https://www.youtube.com", 
            SiteConfig::custom(7.0, 4)
        ),
        Site::new(
            "https://www.netflix.com", 
            SiteConfig::custom(5.5, 3)
        ),
        Site::new(
            "https://www.twitch.tv", 
            SiteConfig::custom(6.0, 4)
        ),
        Site::new(
            "https://www.aws.amazon.com", 
            SiteConfig::custom(12.0, 5)
        ),
        Site::new(
            "https://www.cloudflare.com", 
            SiteConfig::custom(4.5, 3)
        ),
        Site::new(
            "https://www.heroku.com", 
            SiteConfig::custom(4.0, 2)
        ),
        Site::new(
            "https://www.docker.com", 
            SiteConfig::custom(5.0, 3)
        ),
        Site::new(
            "https://www.kubernetes.io", 
            SiteConfig::custom(6.5, 4)
        ),

        Site::default("https://www.smorgiscorp.com"),
        Site::default("https://www.athaun.tech"),
        Site::default("https://www.google.com"),
        Site::default("https://www.facebook.com"),
        Site::default("https://www.twitter.com"),
        Site::default("https://www.instagram.com"),
        Site::default("https://www.linkedin.com"),
        Site::default("https://www.reddit.com"),
        Site::default("https://www.microsoft.com"),
        Site::default("https://www.apple.com"),
        Site::default("https://www.amazon.com"),
        Site::default("https://www.spotify.com"),
        Site::default("https://www.dropbox.com"),
        Site::default("https://www.slack.com"),
        Site::default("https://www.zoom.us"),
        Site::default("https://www.trello.com"),
        Site::default("https://www.notion.so"),
        Site::default("https://www.ebay.com"),
        Site::default("https://www.paypal.com"),
        Site::default("https://www.salesforce.com"),
        Site::default("https://www.atlassian.com"),
        Site::default("https://www.gitlab.com"),
        Site::default("https://www.bitbucket.org"),
        Site::default("https://www.digitalocean.com"),
        Site::default("https://www.stripe.com"),
        Site::default("https://www.figma.com"),
        Site::default("https://www.canva.com"),
        Site::default("https://www.adobe.com"),
        Site::default("https://www.okta.com"),
        Site::default("https://www.mailchimp.com"),
        Site::default("https://www.asana.com"),
        Site::default("https://www.airbnb.com"),
        Site::default("https://www.uber.com"),
        Site::default("https://www.lyft.com"),
        Site::default("https://www.pinterest.com"),
        Site::default("https://www.medium.com"),
        Site::default("https://www.snapchat.com"),
        Site::default("https://www.tiktok.com"),
        Site::default("https://www.reddit.com/r/rust"),
        Site::default("https://www.stackoverflow.com"),
        Site::default("https://www.github.io"),
        Site::default("https://www.git-scm.com"),
        Site::default("https://www.rust-lang.org"),
        Site::default("https://www.coursera.org"),
        Site::default("https://www.edx.org"),
        Site::default("https://www.udacity.com"),
        Site::default("https://www.udemy.com"),
        Site::default("https://www.pluralsight.com"),
        Site::default("https://www.codecademy.com"),
        Site::default("https://www.khanacademy.org"),
    ]
}

mod tests {
    use super::*;

    #[test]
    fn test_site_new() {
        let config = SiteConfig::custom(3.0, 2);
        let site = Site::new("https://www.example.com", config.clone());
        assert_eq!(site.url, "https://www.example.com");
        assert_eq!(site.config, config);
    }

    #[test]
    fn test_site_default() {
        let site = Site::default("https://www.example.com");
        assert_eq!(site.url, "https://www.example.com");
        assert_eq!(site.config, SiteConfig::default());
    }

    #[test]
    fn test_get_websites() {
        let websites = get_websites();
        assert!(!websites.is_empty());
        assert_eq!(websites[0].url, "https://www.github.com");
        assert_eq!(websites[0].config, SiteConfig::custom(3.0, 2));

        assert_eq!(websites.len(), 60);
    }
}