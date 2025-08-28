pub struct Options {
    password_length: usize,
}


impl Options {
    fn new(passwd_len: usize) -> Options {
        Options {
            password_length: passwd_len,
        }
    }
    fn get_password_length(&self) -> usize {
        self.password_length
    }
}