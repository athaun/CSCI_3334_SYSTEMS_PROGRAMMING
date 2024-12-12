mod site_config;
mod site_defs;
mod status;
mod threads;

use site_defs::get_websites;
use threads::SiteChecker;

fn main() {
    let checker = SiteChecker::new(50);
    
    let mut websites = get_websites();
    let results = checker.check_websites(&mut websites);

    println!("Website Status Results:");
    for result in results {
        println!(
            "URL: {},\t\t\tStatus: {:?},\t\tResponse Time: {:?},\t\tTimestamp: {}",
            result.url, 
            result.status, 
            result.response_time, 
            result.timestamp
        );
    }
}