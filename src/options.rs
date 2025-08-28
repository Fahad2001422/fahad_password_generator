pub struct Options {
    password_length: u16,
}


impl Options {
    pub fn new(passwd_len: u16) -> Options {
        Options {
            password_length: passwd_len,
        }
    }
    pub fn get_password_length(&self) -> u16 {
        self.password_length
    }
}