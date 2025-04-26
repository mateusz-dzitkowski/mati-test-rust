use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Debug, Deserialize)]
pub struct SubscribeFormData {
    name: String,
    email: String,
}

pub async fn subscribe(
    form: web::Form<SubscribeFormData>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    match sqlx::query!(
        r#"
        insert into subscriptions (email, name)
        values ($1::text, $2::text)
        "#,
        form.email,
        form.name,
    )
        .execute(pool.get_ref())
        .await {
        Ok(_) => HttpResponse::Ok(),
        Err(e) => {
            println!("{}", e);
            HttpResponse::InternalServerError()
        }
    }
}
