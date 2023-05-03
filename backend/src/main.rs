use surrealdb::{engine::remote::ws::Ws, opt::auth::Root};

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let db = surrealdb::Surreal::new::<Ws>("127.0.0.1").await.unwrap();
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .unwrap();
    db.use_ns("todo").use_db("todo").await.unwrap();
}
