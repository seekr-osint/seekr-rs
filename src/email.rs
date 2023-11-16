pub mod email {
    use serde_email::Email;

    pub enum EmailAdress {
        Valid(Email),
        Invalid(String),
    }
    impl EmailAdress {
        pub fn new(address: &str) -> Self {
            let parsed = serde_email::Email::from_string(address.to_string());
            match parsed {
                Ok(a) => Self::Valid(a),
                Err(_) => Self::Invalid(address.to_string()),
            }
        }
    }
    /// email table in the database.
    /// Maps an email adress to information about the email.
    pub struct EmailInfo {
        pub address: EmailAdress,
    }
    impl EmailInfo {
        pub fn new(address: &str) -> Self {
            Self {
                address: EmailAdress::new(address),
            }
        }
    }
}
