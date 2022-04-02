#[derive(Clone, PartialEq)]
pub enum PaymentError {
    // #[error(transparent)]
    // UnexpectedError(#[from] anyhow::Error),
    PaymentCreationError,
    UnexpectedError,
}
impl std::fmt::Debug for PaymentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, PartialEq)]
pub enum UserError {
    UnexpectedError,
}

impl std::fmt::Debug for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
