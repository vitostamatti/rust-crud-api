use actix_web::{
    delete, get, post, put,
    web::{self},
    HttpResponse, Responder,
};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
// use uuid::Uuid;

use crate::{
    contracts::{CreateNoteRequest, GetNotesQuery, UpdateNoteRequest},
    models::Note,
};

#[get("/healthcheck")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[post("/notes")]
pub async fn create_note(
    body: web::Json<CreateNoteRequest>,
    db: web::Data<PgPool>,
) -> impl Responder {
    let query_result = sqlx::query_as!(
        Note,
        r#"
        INSERT INTO notes (title, content, category)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
        body.title.to_string(),
        body.content.to_string(),
        body.category.to_owned().unwrap_or("".into())
    )
    .fetch_one(db.as_ref())
    .await;
    // TODO: add error types with message
    match query_result {
        Err(_) => HttpResponse::BadRequest().finish(),
        Ok(note) => HttpResponse::Ok().json(note),
    }
}

#[put("/notes/{id}")]
pub async fn update_note(
    path: web::Path<Uuid>,
    body: web::Json<UpdateNoteRequest>,
    db: web::Data<PgPool>,
) -> impl Responder {
    let id = path.into_inner();

    let query_result = sqlx::query_as!(
        Note,
        r#"
        SELECT *
        FROM notes
        WHERE id = $1
        "#,
        id
    )
    .fetch_one(db.as_ref())
    .await;

    match query_result {
        Err(_) => HttpResponse::NotFound().finish(),
        Ok(note) => {
            let now = Utc::now();
            let query_result = sqlx::query_as!(
                Note,
                r#"
                UPDATE notes
                SET title = $1, content = $2, category = $3, published = $4, updated_at = $5  
                WHERE id = $6 
                RETURNING *
                "#,
                body.title.to_owned().unwrap_or(note.title),
                body.content.to_owned().unwrap_or(note.content),
                body.category.to_owned().unwrap_or(note.category.unwrap()),
                body.published.unwrap_or(note.published.unwrap()),
                now,
                id
            )
            .fetch_one(db.as_ref())
            .await;

            match query_result {
                Ok(note) => HttpResponse::Ok().json(note),
                Err(_) => HttpResponse::BadRequest().finish(),
            }
        }
    }
}

#[delete("/notes/{id}")]
pub async fn delete_note(path: web::Path<Uuid>, db: web::Data<PgPool>) -> impl Responder {
    let id = path.into_inner();

    let rows_affected = sqlx::query!("DELETE FROM notes WHERE id = $1", id)
        .execute(db.as_ref())
        .await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        HttpResponse::NotFound().finish()
    } else {
        HttpResponse::Ok().finish()
    }
}

#[get("/notes/{id}")]
pub async fn get_note_by_id(path: web::Path<Uuid>, db: web::Data<PgPool>) -> impl Responder {
    let id = path.into_inner();

    let query_result = sqlx::query_as!(
        Note,
        r#"
        SELECT * FROM notes 
        WHERE id = $1
        "#,
        id
    )
    .fetch_one(db.as_ref())
    .await;

    match query_result {
        Err(_) => HttpResponse::NotFound().finish(),
        Ok(note) => HttpResponse::Ok().json(note),
    }
}

#[get("/notes")]
pub async fn get_notes(query: web::Query<GetNotesQuery>, db: web::Data<PgPool>) -> impl Responder {
    let take = query.take.unwrap_or(10);
    let skip = query.skip.unwrap_or(0);
    let query_result = sqlx::query_as!(
        Note,
        r#"
        SELECT * 
        FROM notes 
        ORDER BY id 
        LIMIT $1 
        OFFSET $2
        "#,
        take as i32,
        skip as i32
    )
    .fetch_all(db.as_ref())
    .await;

    match query_result {
        Err(_) => HttpResponse::InternalServerError().finish(),
        Ok(notes) => HttpResponse::Ok().json(notes),
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_check)
        .service(get_notes)
        .service(get_note_by_id)
        .service(update_note)
        .service(delete_note)
        .service(create_note);

    conf.service(scope);
}
