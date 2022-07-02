use chrono::Utc;
use domain::errors::PaymentError;
use domain::interface::GetPaymentType;
use sqlx::PgPool;

pub async fn get_payment_types(pool: &PgPool) -> Result<Vec<GetPaymentType>, PaymentError> {
    let mut payment_types = vec![];
    let mut result = sqlx::query!(
        r#"
        SELECT id, name
        FROM payment_type
        "#,
    )
    .fetch_all(pool)
    .await
    .unwrap()
    .into_iter()
    .map(|rec| GetPaymentType {
        id: rec.id,
        name: rec.name,
    })
    .collect();
    payment_types.append(&mut result);
    Ok(payment_types)
}

pub struct FindPayment {
    pub id: i32,
    pub payment_type_id: i32,
    pub amount: i32,
    pub creation_date: String,
}

// pub async fn find_payment(pool: &PgPool, user_id: &i32) -> Result<Vec<FindPayment>, sqlx::Error> {
//     let mut payment = vec![];
//     let mut result = sqlx::query!(
//         r#"
//             SELECT id, payment_type_id, amount, to_char(created_at, 'YYYY-MM-dd') as creation_date FROM payment WHERE user_id = $1
//         "#,
//         user_id
//     ).fetch_all(pool)
//         .await
//         .unwrap()
//         .into_iter()
//         .map(|rec| FindPayment {
//             id: rec.id,
//             payment_type_id: rec.payment_type_id,
//             amount: rec.amount,
//             creation_date: rec.creation_date.unwrap(),
//         })
//         .collect();
//     payment.append(&mut result);
//     Ok(payment)
// }
