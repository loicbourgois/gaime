pub enum ClientErrorCode {
    CouldNotParseRequest,
    InvalidInputCode,
    BinaryNotAllowed,
}

pub struct ClientError {
    code: String,
    message: String,
}

impl ClientError {
    pub fn code(&self) -> String {
        self.code.clone()
    }

    pub fn message(&self) -> String {
        self.message.clone()
    }

    pub fn new(error_code: ClientErrorCode) -> ClientError {
        match error_code {
            ClientErrorCode::CouldNotParseRequest => ClientError {
                code: "e01".to_owned(),
                message: "Could not parse request".to_owned(),
            },
            ClientErrorCode::InvalidInputCode => ClientError {
                code: "e02".to_owned(),
                message: "Invalid input code".to_owned(),
            },
            ClientErrorCode::BinaryNotAllowed => ClientError {
                code: "e03".to_owned(),
                message: "Binary not allowed".to_owned(),
            },
        }
    }
}
