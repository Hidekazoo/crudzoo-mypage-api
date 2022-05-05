#[derive(Eq, PartialEq, Clone, Debug)]
pub struct BookId(pub i32);

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Book {
    pub id: BookId,
    pub name: String,
}
