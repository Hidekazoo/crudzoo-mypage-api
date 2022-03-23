use sqlx::PgPool;

pub async fn create_user(pool: &PgPool, email: &str) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
            INSERT INTO users (email) VALUES ($1)
        "#,
        email.to_string()
    )
    .execute(pool)
    .await?;
    Ok(())
}
