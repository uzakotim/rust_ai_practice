use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::{Item, CreateItem, UpdateItem};

pub async fn create_item(
    pool: web::Data<PgPool>,
    item: web::Json<CreateItem>,
) -> impl Responder {
    let result = sqlx::query!(
        r#"
        INSERT INTO items (name, description)
        VALUES ($1, $2)
        RETURNING id, name, description, created_at, updated_at
        "#,
        item.name,
        item.description
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(record) => {
            let item = Item {
                id: Some(record.id),
                name: record.name,
                description: record.description,
                created_at: Some(record.created_at),
                updated_at: Some(record.updated_at),
            };
            HttpResponse::Created().json(item)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_items(pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query!(
        r#"
        SELECT id, name, description, created_at, updated_at
        FROM items
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(records) => {
            let items: Vec<Item> = records
                .into_iter()
                .map(|record| Item {
                    id: Some(record.id),
                    name: record.name,
                    description: record.description,
                    created_at: Some(record.created_at),
                    updated_at: Some(record.updated_at),
                })
                .collect();
            HttpResponse::Ok().json(items)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_item(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
) -> impl Responder {
    let result = sqlx::query!(
        r#"
        SELECT id, name, description, created_at, updated_at
        FROM items
        WHERE id = $1
        "#,
        id.into_inner()
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(record) => {
            let item = Item {
                id: Some(record.id),
                name: record.name,
                description: record.description,
                created_at: Some(record.created_at),
                updated_at: Some(record.updated_at),
            };
            HttpResponse::Ok().json(item)
        }
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn update_item(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
    item: web::Json<UpdateItem>,
) -> impl Responder {
    let result = sqlx::query!(
        r#"
        UPDATE items
        SET name = COALESCE($1, name),
            description = COALESCE($2, description),
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $3
        RETURNING id, name, description, created_at, updated_at
        "#,
        item.name.as_deref(),
        item.description.as_deref(),
        id.into_inner()
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(record) => {
            let item = Item {
                id: Some(record.id),
                name: record.name,
                description: record.description,
                created_at: Some(record.created_at),
                updated_at: Some(record.updated_at),
            };
            HttpResponse::Ok().json(item)
        }
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn delete_item(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
) -> impl Responder {
    let result = sqlx::query!(
        r#"
        DELETE FROM items
        WHERE id = $1
        "#,
        id.into_inner()
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
} 