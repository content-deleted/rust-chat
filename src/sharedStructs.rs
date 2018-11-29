pub struct User {
    pub username: String,
    pub password: String,
}

pub enum MessageType {
    ClientLogin,
    LoginSuccess,
    LoginFailure,
    ClientLogout,
    Continue,
    End,
}
