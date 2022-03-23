#[derive(Eq, PartialEq, Clone, Debug)]
pub struct PaymentTypeId(pub i32);

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct PaymentType {
    pub id: PaymentTypeId,
    pub name: String,
}
