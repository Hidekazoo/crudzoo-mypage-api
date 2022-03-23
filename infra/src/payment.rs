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
