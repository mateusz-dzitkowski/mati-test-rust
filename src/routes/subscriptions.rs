use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Debug, Deserialize)]
pub struct SubscribeFormData {
    name: String,
    email: String,
}

#[tracing::instrument(
    name="add subscriber",
    skip(form, pool),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name,
    ),
)]
pub async fn subscribe(
    form: web::Form<SubscribeFormData>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    match insert_subscriber(&form, &pool).await
    {
        Ok(_) => {
            HttpResponse::Ok().await
        }
        Err(e) => {
            tracing::error!("failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().await
        }
    }
}

#[tracing::instrument(
    name = "save subscriber",
    skip(form, pool),
)]
pub async fn insert_subscriber(
    form: &SubscribeFormData,
    pool: &PgPool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        insert into subscriptions (email, name)
        values ($1::text, $2::text)
        "#,
        form.email,
        form.name,
    )
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("failed to execute query {:?}", e);
            e
        })?;
    Ok(())
}
