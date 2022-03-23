use crate::entity::{PaymentTypeId, UserId};
pub struct PaymentId(pub i32);

pub struct Payment {
    id: PaymentTypeId,
    user_id: UserId,
    payment_type_id: PaymentTypeId,
}