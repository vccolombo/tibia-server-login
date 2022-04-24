use crate::server::Server;

pub struct Character {
    pub nickname: String,
    pub server: Server,
}

impl Character {
    pub fn new(name: String, server: Server) -> Self {
        return Self { nickname: name, server };
    }
}
