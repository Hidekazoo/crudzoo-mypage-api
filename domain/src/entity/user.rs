#[derive(Eq, PartialEq, Clone, Debug)]
pub struct UserId(pub i32);

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct User {
    id: i32,
    email: String,
}
