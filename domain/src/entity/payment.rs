use crate::entity::{PaymentTypeId, UserId};

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct PaymentId(pub i32);

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Payment {
    pub id: PaymentId,
    pub user_id: UserId,
    pub payment_type_id: PaymentTypeId,
    pub amount: i32,
    pub creation_date: String
}
