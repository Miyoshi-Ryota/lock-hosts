use std::{thread::sleep, time::Duration};

use lock_hosts::HostsWatcher;

fn main() {
    let watcher = HostsWatcher::new();
    loop {
        if watcher.does_be_modified() {
            watcher.overwrite_hosts_by_original()
        }
        sleep(Duration::from_secs(30));
    }
}
