use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use anyhow::Result;
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{FromRow, PgPool, Row};
use chrono::{NaiveDateTime};

// this struct will use to receive user input
#[derive(Serialize, Deserialize)]
pub struct NotificationRequest {
  pub description: String,
  pub done: bool,
  pub created_at: NaiveDateTime,
}

// this struct will be used to represent database record
#[derive(Serialize, FromRow)]
pub struct Notification {
  pub id: i32,
  pub description: String,
  pub done: bool,
  pub created_at: NaiveDateTime,
}

// implementation of Actix Responder for Notification struct so we can return Notification from action handler
impl Responder for Notification {
  type Error = Error;
  type Future = Ready<Result<HttpResponse, Error>>;

  fn respond_to(self, _req: &HttpRequest) -> Self::Future {
    let body = serde_json::to_string(&self).unwrap();
    // create response and set content type
    ready(Ok(
      HttpResponse::Ok()
        .content_type("application/json")
        .body(body),
    ))
  }
}

// Implementation for Notification struct, functions for read/write/update and delete Notification from database
impl Notification {
  pub async fn find_all(pool: &PgPool) -> Result<Vec<Notification>> {
    let mut notifications = vec![];
    let recs = sqlx::query!(
      r#"
        SELECT *
        FROM notifications
        ORDER BY id
      "#
    )
    .fetch_all(pool)
    .await?;

    for rec in recs {
      notifications.push(Notification {
        id: rec.id,
        description: rec.description,
        done: rec.done,
        created_at: rec.created_at
      });
    }

    Ok(notifications)
  }

  pub async fn find_by_id(id: i32, pool: &PgPool) -> Result<Notification> {
    let rec = sqlx::query!(
      r#"
        SELECT * FROM notifications WHERE id = $1
      "#,
      id
    )
    .fetch_one(&*pool)
    .await?;

    Ok(Notification {
      id: rec.id,
      description: rec.description,
      done: rec.done,
      created_at: rec.created_at
    })
  }

  pub async fn create(request: NotificationRequest, pool: &PgPool) -> Result<Notification> {
    let mut tx = pool.begin().await?;
    let notification = sqlx::query(
      r#"
        INSERT INTO Notifications (description, done, created_at) VALUES ($1, $2, $3)
        RETURNING id, description, done, created_at
      "#,
    )
    .bind(&request.description)
    .bind(request.done)
    .bind(chrono::Utc::now())
    .map(|row: PgRow| Notification {
      id: row.get(0),
      description: row.get(1),
      done: row.get(2),
      created_at: row.get(3),
    })
    .fetch_one(&mut tx)
    .await?;

    tx.commit().await?;
    Ok(notification)
  }

  pub async fn update(
    id: i32,
    request: NotificationRequest,
    pool: &PgPool,
  ) -> Result<Notification> {
    let mut tx = pool.begin().await.unwrap();
    let notification = sqlx::query("UPDATE notifications SET description = $1, done = $2 WHERE id = $3 RETURNING id, description, done, created_at")
            .bind(&request.description)
            .bind(request.done)
            .bind(id)
            .map(|row: PgRow| {
                Notification {
                    id: row.get(0),
                    description: row.get(1),
                    done: row.get(2),
                    created_at: row.get(3)
                }
            })
            .fetch_one(&mut tx)
            .await?;

    tx.commit().await.unwrap();
    Ok(notification)
  }

  pub async fn delete(id: i32, pool: &PgPool) -> Result<u64> {
    let mut tx = pool.begin().await?;
    let deleted = sqlx::query("DELETE FROM notifications WHERE id = $1")
      .bind(id)
      .execute(&mut tx)
      .await?;

    tx.commit().await?;
    Ok(deleted)
  }
}
