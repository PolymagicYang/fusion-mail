use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String
}

pub async fn subscribe(
    form: web::Form<FormData>,
    connection: web::Data<PgPool>
) -> HttpResponse {
    let _ = sqlx::query!(
        r#"
            INSERT INTO subscriptions (id, email, name, subscribed_at)
            VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(connection.get_ref())
    .await;
    
    HttpResponse::Ok().finish()
}