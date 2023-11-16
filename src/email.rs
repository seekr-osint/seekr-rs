pub mod email {
    use serde_email::{Email, EmailError};

    pub enum EmailAdress {
        Valid(Email),
        Invalid(String),
    }
    impl EmailAdress {
        pub fn new(address: String) -> Self {
            let parsed = serde_email::Email::from_string(address.clone());
            match parsed {
                Ok(a) => Self::Valid(a),
                Err(_) => Self::Invalid(address.clone()),
            }
        }
    }
    /// email table in the database.
    /// Maps an email adress to information about the email.
    pub struct EmailInfo {
        address: EmailAdress,
    }
    impl EmailInfo {
        pub fn new(address: String) -> Self {
            Self {
                address: EmailAdress::new(address),
            }
        }
    }
}
