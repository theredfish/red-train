#[derive(Debug, Clone)]
pub struct Subdomain {
    pub domain: String,
    pub open_ports: Vec<Port>,
}

impl Subdomain {
    pub fn new(domain: String) -> Self {
        Self {
            domain,
            open_ports: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub struct Port {
    pub port: u16,
    pub is_open: bool,
}
