use std::sync::Arc;

use crate::{
    models::{NoteModel, NoteModelResponse},
    schema::CreateNoteSchema,
    AppState,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use sqlx::Error;

fn to_note_response(note: &NoteModel) -> NoteModelResponse {
    NoteModelResponse {
        id: note.id.to_owned(),
        title: note.title.to_owned(),
        content: note.content.to_owned(),
        is_published: note.is_published,
        created_at: note.created_at,
        updated_at: note.updated_at,
    }
}

fn to_note_responses(notes: &Vec<NoteModel>) -> Vec<NoteModelResponse> {
    notes.iter().map(|note| to_note_response(note)).collect()
}

pub async fn create_note_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateNoteSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Insert
    let id = uuid::Uuid::new_v4().to_string();
    let query_result = sqlx::query(r#"INSERT INTO notes (id, title, content) VALUES ($1, $2, $3)"#)
        .bind(&id)
        .bind(&body.title)
        .bind(&body.content)
        .execute(&data.db)
        .await
        .map_err(|err: sqlx::Error| err.to_string());

    // Duplicate err check
    if let Err(err) = query_result {
        if err.contains("Duplicate entry") {
            let error_response = serde_json::json!({
                "status": "error",
                "message": "Note already exists",
            });
            return Err((StatusCode::CONFLICT, Json(error_response)));
        }

        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error","message": format!("{:?}", err)})),
        ));
    }

    let note = sqlx::query_as!(
        NoteModel,
        r#"SELECT id, title, content, is_published, created_at, updated_at FROM notes WHERE id = $1"#,
        &id
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error","message": format!("{:?}", e)})),
        )
    })?;

    let note_response = serde_json::json!({
            "status": "success",
            "data": serde_json::json!({
                "note": to_note_response(&note)
        })
    });

    Ok(Json(note_response))
}

pub async fn find_many_note_handler(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let notes = sqlx::query_as!(
        NoteModel,
        r#"SELECT id, title, content, is_published, created_at, updated_at FROM notes"#
    )
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error","message": format!("{:?}", e)})),
        )
    })?;

    let note_reponse = serde_json::json!({
        "status": "success",
        "data": serde_json::json!({
            "notes": to_note_responses(&notes)
        })
    });

    Ok(Json(note_reponse))
}

pub async fn find_one_note_handler(
    State(data): State<Arc<AppState>>,
    note_id: String,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let note = sqlx::query_as!(
        NoteModel,
        r#"SELECT id, title, content, is_published, created_at, updated_at FROM notes WHERE id = $1"#,
        &note_id
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error","message": format!("{:?}", e)})),
        )
    })?;

    let note_response = serde_json::json!({
        "status": "success",
        "data": serde_json::json!({
            "note": to_note_response(&note)
        })
    });

    Ok(Json(note_response))
}

pub async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "API";

    let json_response = serde_json::json!({
        "message": MESSAGE,
        "status": "OK"
    });

    return Json(json_response);
}
