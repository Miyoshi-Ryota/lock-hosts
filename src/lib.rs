use std::fs;

pub struct HostsWatcher {
    original: String,
}

impl HostsWatcher {
    pub fn new() -> Self {
        Self {
            original: Self::read_hosts(),
        }
    }

    fn read_hosts() -> String {
        fs::read_to_string("/etc/hosts").expect("cannot open /etc/hosts")
    }

    pub fn does_be_modified(&self) -> bool {
        Self::read_hosts() != self.original
    }

    pub fn overwrite_hosts_by_original(&self) {
        fs::write("/etc/hosts", &self.original).expect(&format!(
            "cannot write etc hosts. try to write content: {}",
            &self.original
        ))
    }
}
