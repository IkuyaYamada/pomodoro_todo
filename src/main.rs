use warp::Filter;

#[tokio::main]
async fn main() {
    // GET /hello に対して "Hello, world!" と応答するルートを作成します。
    let hello = warp::path("hello")
        .map(|| warp::reply::html("Hello, world!"));

    // Warpサーバーを起動します。このサーバーはlocalhostのポート3030でリクエストを待ち受けます。
    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}