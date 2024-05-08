
#[derive(thiserror::Error, Debug)]
pub enum NeatError {
    #[error("Attempted crossbreed between incompatible species")]
    IncompatibleSpeciesBreedError,
    #[error("Input size does not match network input size")]
    MismatchedInputSizeError
}

pub type Result<T> = std::result::Result<T, NeatError>;