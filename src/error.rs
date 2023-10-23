use halo2_proofs::plonk;

#[derive(Debug)]
pub enum Error {
    Halo2Error(Box<plonk::Error>),
    StdError(Box<std::io::Error>),
    SerdeJsonError(Box<serde_json::Error>),
    InternalError(&'static str),
}

impl From<plonk::Error> for Error {
    fn from(err: plonk::Error) -> Self {
        Error::Halo2Error(Box::new(err))
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::StdError(Box::new(err))
    }
}
impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::SerdeJsonError(Box::new(err))
    }
}
