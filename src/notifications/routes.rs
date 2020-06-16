use crate::notifications::{Notification, NotificationRequest};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use sqlx::PgPool;

#[get("/notifications")]
async fn find_all(db_pool: web::Data<PgPool>) -> impl Responder {
  let result = Notification::find_all(db_pool.get_ref()).await;
  match result {
    Ok(notifications) => HttpResponse::Ok().json(notifications),
    _ => HttpResponse::BadRequest().body("Error trying to read all notifications from database"),
  }
}

#[get("/notifications/{id}")]
async fn find(id: web::Path<i32>, db_pool: web::Data<PgPool>) -> impl Responder {
  let result = Notification::find_by_id(id.into_inner(), db_pool.get_ref()).await;
  match result {
    Ok(notification) => HttpResponse::Ok().json(notification),
    _ => HttpResponse::BadRequest().body("Todo not found"),
  }
}

#[post("/notifications")]
async fn create(
  notification: web::Json<NotificationRequest>,
  db_pool: web::Data<PgPool>,
) -> impl Responder {
  let result = Notification::create(notification.into_inner(), db_pool.get_ref()).await;
  match result {
    Ok(notification) => HttpResponse::Ok().json(notification),
    _ => HttpResponse::BadRequest().body("Error trying to create new notification"),
  }
}

#[put("/notifications/{id}")]
async fn update(
  id: web::Path<i32>,
  notification: web::Json<NotificationRequest>,
  db_pool: web::Data<PgPool>,
) -> impl Responder {
  let result = Notification::update(
    id.into_inner(),
    notification.into_inner(),
    db_pool.get_ref(),
  )
  .await;
  match result {
    Ok(notification) => HttpResponse::Ok().json(notification),
    _ => HttpResponse::BadRequest().body("Notification not found"),
  }
}

#[delete("/notifications/{id}")]
async fn delete(id: web::Path<i32>, db_pool: web::Data<PgPool>) -> impl Responder {
  let result = Notification::delete(id.into_inner(), db_pool.get_ref()).await;
  match result {
    Ok(rows) => {
      if rows > 0 {
        HttpResponse::Ok().body(format!("Successfully deleted {} record(s)", rows))
      } else {
        HttpResponse::BadRequest().body("Notification not found")
      }
    }
    _ => HttpResponse::BadRequest().body("Notification not found"),
  }
}

// function that will be called on new Application to configure routes for this module
pub fn init(cfg: &mut web::ServiceConfig) {
  cfg.service(find_all);
  cfg.service(find);
  cfg.service(create);
  cfg.service(update);
  cfg.service(delete);
}
