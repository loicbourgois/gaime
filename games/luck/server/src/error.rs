

#[derive(Debug, Fail)]
pub enum MyError {
    #[fail(display = "invalid play")]
    InvalidPlay {
    },
    #[fail(display = "invalid code data pair")]
    InvalidCodeDataPair {
    },
    #[fail(display = "unknown api code : {}", code)]
    UnknownApiCode {
        code: String
    }
}
