use actix_web::{test, web, App};
use rust_backend::{handlers, models};
use sqlx::postgres::PgPoolOptions;
use std::env;

#[actix_rt::test]
async fn test_crud_operations() {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    // Create a test database pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    // Create test app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/api")
                    .service(
                        web::resource("/items")
                            .route(web::post().to(handlers::create_item))
                            .route(web::get().to(handlers::get_items)),
                    )
                    .service(
                        web::resource("/items/{id}")
                            .route(web::get().to(handlers::get_item))
                            .route(web::put().to(handlers::update_item))
                            .route(web::delete().to(handlers::delete_item)),
                    ),
            ),
    )
    .await;

    // Test Create
    let create_item = models::CreateItem {
        name: "Test Item".to_string(),
        description: "Test Description".to_string(),
    };

    let resp = test::TestRequest::post()
        .uri("/api/items")
        .set_json(&create_item)
        .send_request(&app)
        .await;

    assert!(resp.status().is_success());
    let item: models::Item = test::read_body_json(resp).await;
    assert_eq!(item.name, "Test Item");
    assert_eq!(item.description, "Test Description");
    let item_id = item.id.unwrap();

    // Test Read
    let resp = test::TestRequest::get()
        .uri(&format!("/api/items/{}", item_id))
        .send_request(&app)
        .await;

    assert!(resp.status().is_success());
    let item: models::Item = test::read_body_json(resp).await;
    assert_eq!(item.name, "Test Item");

    // Test Update
    let update_item = models::UpdateItem {
        name: Some("Updated Item".to_string()),
        description: Some("Updated Description".to_string()),
    };

    let resp = test::TestRequest::put()
        .uri(&format!("/api/items/{}", item_id))
        .set_json(&update_item)
        .send_request(&app)
        .await;

    assert!(resp.status().is_success());
    let item: models::Item = test::read_body_json(resp).await;
    assert_eq!(item.name, "Updated Item");
    assert_eq!(item.description, "Updated Description");

    // Test Delete
    let resp = test::TestRequest::delete()
        .uri(&format!("/api/items/{}", item_id))
        .send_request(&app)
        .await;

    assert!(resp.status().is_success());

    // Verify deletion
    let resp = test::TestRequest::get()
        .uri(&format!("/api/items/{}", item_id))
        .send_request(&app)
        .await;

    assert!(resp.status().is_client_error());
} 