use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use warp::body::json as body_json;
use warp::Filter;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TodoItem {
    id: u64,         // 一意の識別子
    title: String,   // ToDo項目のタイトル
    completed: bool, // 完了したかどうか
}

#[derive(Debug, Deserialize)]
struct UpdateTodoItem {
    title: String,
    completed: bool,
}

#[derive(Clone)]
struct AppState {
    todos: Arc<Mutex<Vec<TodoItem>>>,
}

async fn list_todos_handler(state: AppState) -> Result<impl warp::Reply, warp::Rejection> {
    let todos = state.todos.lock().unwrap();
    Ok(warp::reply::json(&*todos))
}

async fn add_todo_handler(
    todo: TodoItem,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut todos = state.todos.lock().unwrap();
    todos.push(todo);
    Ok(warp::reply::json(&*todos))
}

async fn update_todo_handler(
    id: u64,
    updated_todo: UpdateTodoItem,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut todos = state.todos.lock().unwrap();

    // ToDoリストから対象のToDoアイテムを検索
    if let Some(todo) = todos.iter_mut().find(|todo| todo.id == id) {
        // ToDoアイテムの内容を更新
        todo.title = updated_todo.title;
        todo.completed = updated_todo.completed;
        Ok(warp::reply::json(todo))
    } else {
        // 対象のToDoアイテムが見つからない場合はエラーを返す
        Err(warp::reject::not_found())
    }
}

async fn delete_todo_handler(
    id: u64,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut todos = state.todos.lock().unwrap();

    // 指定されたIDのアイテムを削除する。retainは条件に一致しない要素を削除します。
    let original_len = todos.len();
    todos.retain(|todo| todo.id != id);

    // 削除が行われたか確認し、結果に応じてレスポンスを返す。
    let num_deleted = original_len - todos.len();
    if num_deleted > 0 {
        Ok(warp::reply::with_status(
            "Item deleted",
            warp::http::StatusCode::OK,
        ))
    } else {
        Err(warp::reject::not_found())
    }
}

fn with_state(
    state: AppState,
) -> impl Filter<Extract = (AppState,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}

#[tokio::main]
async fn main() {
    let todos = Arc::new(Mutex::new(Vec::new()));
    let app_state = AppState {
        todos: todos.clone(),
    };

    let get_todos_route = warp::get()
        .and(warp::path("todos"))
        .and(with_state(app_state.clone()))
        .and_then(list_todos_handler);

    let update_todo_route = warp::put()
        .and(warp::path("todos")) // '/todos' パス
        .and(warp::path::param()) // URLパスからIDを取得
        .and(body_json()) // リクエストボディをJSONとしてパース
        .and(with_state(app_state.clone())) // アプリケーション状態を渡す
        .and_then(update_todo_handler);

    let post_todo_route = warp::post()
        .and(warp::path("todos"))
        .and(body_json())
        .and(with_state(app_state.clone()))
        .and_then(add_todo_handler);

    let delete_todo_route = warp::delete()
        .and(warp::path("todos")) // 'todos' パスを指定
        .and(warp::path::param::<u64>()) // URLパスからIDを取得
        .and(with_state(app_state.clone()))
        .and_then(delete_todo_handler);

    // ルート結合
    let routes = get_todos_route
        .or(post_todo_route)
        .or(update_todo_route)
        .or(delete_todo_route);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
