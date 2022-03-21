use sqlx::PgPool;
use domain::errors::paymentError;
// use domain::interface::PaymentType;
use domain::interface::GetPaymentType;

// #[derive(Debug,Clone)]
// pub struct GetPaymentTypes {
//     id: i32,
//     name: String
// }

pub async fn get_payment_types(pool: &PgPool) -> Result<Vec<GetPaymentType>, paymentError> {
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