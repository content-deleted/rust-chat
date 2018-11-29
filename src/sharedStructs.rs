
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


pub fn messageStr (messageType : MessageType) -> String {
    match messageType {
        MessageType::ClientLogin => String::from("ClientLogin"),
        MessageType::LoginSuccess  =>  String::from("LoginSuccess"),
        MessageType::LoginFailure  =>  String::from("LoginFailure"),
        MessageType::ClientLogout =>  String::from("ClientLogout"),
        MessageType::Continue=>  String::from("Continue"),
        MessageType::End =>  String::from("End"),
    }
}
