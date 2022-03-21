// #[derive(Eq, PartialEq, Clone, Debug)]

// #[derive(Clone, thiserror::Error)]
#[derive(Clone, PartialEq)]
pub enum paymentError {
    // #[error(transparent)]
    // UnexpectedError(#[from] anyhow::Error),
    UnexpectedError
}
impl std::fmt::Debug for paymentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
