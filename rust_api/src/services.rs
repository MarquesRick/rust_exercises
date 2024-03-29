use actix::Message;
use actix_web::{
    delete, get, patch, post,
    web::{scope, Data, Json, Path, Query, ServiceConfig},
    HttpResponse, Responder,
};

use serde_json::json;
use uuid::{uuid, Uuid};

use crate::{
    model::TaskModel,
    schema::{CreateTaskSchema, FilterOptions, UpdateTaskSchema},
    AppState,
};

#[get("/healthchecker")]
async fn health_checker() -> impl Responder {
    const MESSAGE: &str = "Health Check: API is up and running smoothly";

    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": MESSAGE
    }))
}

#[post("/task")]
async fn create_task(body: Json<CreateTaskSchema>, data: Data<AppState>) -> impl Responder {
    match sqlx::query_as!(
        TaskModel,
        "INSERT INTO tasks(title, content) VALUES($1, $2) RETURNING * ",
        body.title.to_string(),
        body.content.to_string()
    )
    .fetch_one(&data.db)
    .await
    {
        Ok(task) => {
            let note_response = json!({
                "status": "success",
                "task": json!({
                     "task": task,
                })
            });

            return HttpResponse::Ok().json(note_response);
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(json!({
                "status": "",
                "message": format!("{:?}", error)
            }));
        }
    }
}

#[get("/tasks")]
async fn get_all_tasks(opts: Query<FilterOptions>, data: Data<AppState>) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    match sqlx::query_as!(
        TaskModel,
        "SELECT * FROM tasks ORDER BY id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await
    {
        Ok(tasks) => {
            let json_response = json!({
                "status": "success",
                "result": tasks.len(),
                "tasks": tasks
            });

            return HttpResponse::Ok().json(json_response);
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(json!({
                "status": "",
                "message": format!("{:?}", error)
            }));
        }
    }
}

#[get("/task/{id}")]
async fn get_task_by_id(path: Path<Uuid>, data: Data<AppState>) -> impl Responder {
    let task_id = path.into_inner();

    match sqlx::query_as!(TaskModel, "SELECT * FROM tasks WHERE id = $1", task_id)
        .fetch_one(&data.db)
        .await
    {
        Ok(task) => {
            let json_response = json!({
                "status": "success",
                "task": task
            });

            return HttpResponse::Ok().json(json_response);
        }
        Err(sqlx::Error::RowNotFound) => {
            // Handle the case where no row is found
            println!("No row found for the given query");
            return HttpResponse::NotFound().json(json!({
                "status": "",
                "message": "No row found for the given query".to_string()
            }));
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(json!({
                "status": "",
                "message": format!("{:?}", error)
            }));
        }
    }
}

#[delete("/task/{id}")]
async fn delete_task_by_id(path: Path<Uuid>, data: Data<AppState>) -> impl Responder {
    let task_id = path.into_inner();

    match sqlx::query_as!(TaskModel, "DELETE FROM tasks WHERE id = $1", task_id)
        .execute(&data.db)
        .await
    {
        Ok(_) => {
            return HttpResponse::NoContent().finish();
        }
        Err(sqlx::Error::RowNotFound) => {
            // Handle the case where no row is found
            println!("No row found for the given query");
            return HttpResponse::NotFound().json(json!({
                "status": "",
                "message": "No row found for the given query".to_string()
            }));
        }
        Err(error) => {
            println!("{:?}", error);
            return HttpResponse::InternalServerError().json(json!({
                "status": "",
                "message": format!("{:?}", error)
            }));
        }
    }
}

#[patch("/task/{id}")]
async fn update_task_by_id(
    path: Path<Uuid>,
    body: Json<UpdateTaskSchema>,
    data: Data<AppState>,
) -> impl Responder {
    let task_id = path.into_inner();

    match sqlx::query_as!(TaskModel, "SELECT * FROM tasks WHERE id = $1", task_id)
        .fetch_one(&data.db)
        .await
    {
        Ok(tasks) => {
            match sqlx::query_as!(
                TaskModel,
                "UPDATE tasks SET content = $1, title = $2 WHERE id = $3 RETURNING *",
                body.content.to_owned().unwrap_or(tasks.content),
                body.title.to_owned().unwrap_or(tasks.title),
                task_id
            )
            .fetch_one(&data.db)
            .await
            {
                Ok(task) => {
                    let task_respnse = json!({
                        "status": "success",
                        "task": task
                    });

                    return HttpResponse::Ok().json(task_respnse);
                }
                Err(error) => {
                    let message = format!("{:?}", error);
                    return HttpResponse::InternalServerError().json(json!({
                        "status": "error",
                        "message": message
                    }));
                }
            }
        }
        Err(error) => {
            let message = format!("{:?}", error);
            return HttpResponse::NotFound().json(json!({
                "status": "not found",
                "message": message
            }));
        }
    }
}

pub fn config(conf: &mut ServiceConfig) {
    let scope = scope("/api")
        .service(health_checker)
        .service(create_task)
        .service(get_all_tasks)
        .service(get_task_by_id)
        .service(delete_task_by_id)
        .service(update_task_by_id);
    conf.service(scope);
}
