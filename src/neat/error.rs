
#[derive(thiserror::Error, Debug)]
pub enum NeatError {
    #[error("Attempted crossbreed between incompatible species")]
    IncompatibleSpeciesBreedError
}

pub type Result<T> = std::result::Result<T, NeatError>;