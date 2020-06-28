use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use anyhow::Result;
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{FromRow, PgPool, Row};
use chrono::{NaiveDateTime};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct NotificationUpdateRequest {
  pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct NotificationCreateRequest {
  pub user_id: Uuid,
  pub description: String,
}

#[derive(Serialize, FromRow)]
pub struct Notification {
  pub id: Uuid,
  pub user_id: Uuid,
  pub description: String,
  pub read_at: Option<NaiveDateTime>,
  pub created_at: NaiveDateTime,
}

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
        user_id: rec.user_id,
        description: rec.description,
        read_at: rec.read_at,
        created_at: rec.created_at
      });
    }

    Ok(notifications)
  }

  pub async fn find_by_id(id: Uuid, pool: &PgPool) -> Result<Notification> {
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
      user_id: rec.user_id,
      description: rec.description,
      read_at: rec.read_at,
      created_at: rec.created_at
    })
  }

  pub async fn create(request: NotificationCreateRequest, pool: &PgPool) -> Result<Notification> {
    let mut tx = pool.begin().await?;
    let notification = sqlx::query(
      r#"
        INSERT INTO Notifications (id, user_id, description, created_at) VALUES ($1, $2, $3, $4)
        RETURNING id, user_id, description, read_at, created_at
      "#,
    )
    .bind(Uuid::new_v4())
    .bind(&request.user_id)
    .bind(&request.description)
    .bind(chrono::Utc::now())
    .map(|row: PgRow| Notification {
      id: row.get(0),
      user_id: row.get(1),
      description: row.get(2),
      read_at: row.get(3),
      created_at: row.get(4),
    })
    .fetch_one(&mut tx)
    .await?;

    tx.commit().await?;
    Ok(notification)
  }

  pub async fn update(
    id: Uuid,
    request: NotificationUpdateRequest,
    pool: &PgPool,
  ) -> Result<Notification> {
    let mut tx = pool.begin().await.unwrap();
    let notification = sqlx::query("UPDATE notifications SET description = $1 WHERE id = $3 RETURNING id, user_id, description, read_at, created_at")
            .bind(&request.description)
            .bind(id)
            .map(|row: PgRow| {
                Notification {
                    id: row.get(0),
                    user_id: row.get(1),
                    description: row.get(2),
                    read_at: row.get(3),
                    created_at: row.get(4)
                }
            })
            .fetch_one(&mut tx)
            .await?;

    tx.commit().await.unwrap();
    Ok(notification)
  }

  pub async fn delete(id: Uuid, pool: &PgPool) -> Result<u64> {
    let mut tx = pool.begin().await?;
    let deleted = sqlx::query("DELETE FROM notifications WHERE id = $1")
      .bind(id)
      .execute(&mut tx)
      .await?;

    tx.commit().await?;
    Ok(deleted)
  }
}
