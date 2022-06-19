use thiserror::Error;

#[derive(Clone, PartialEq)]
pub enum PaymentError {
    // #[error(transparent)]
    // UnexpectedError(#[from] anyhow::Error),
    PaymentCreationError,
    FindPaymentError,
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

#[derive(Clone, PartialEq)]
pub enum BookError {
    UnexpectedError,
}

impl std::fmt::Debug for BookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[derive(Clone, PartialEq, Error, Debug)]
pub enum DailyConditionError {
    #[error("daily condition unexpected error")]
    UnexpectedError,
}
