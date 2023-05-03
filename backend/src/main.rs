use models::{Tribe, User};
use serde::Deserialize;
use surrealdb::{
    engine::remote::ws::Ws,
    opt::auth::Root,
    sql::{Id, Thing},
};

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let db = surrealdb::Surreal::new::<Ws>("127.0.0.1:8000")
        .await
        .unwrap();
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .unwrap();
    db.use_ns("todo").use_db("todo").await.unwrap();

    let user: User<Thing> = User::new("user name".to_string(), "user@example.com".to_string());
    println!("User: {user:?}");
    let user: User<Thing> = db.create("user").content(user).await.unwrap();
    println!("User: {user:?}");
    let tribe: Tribe<Thing, Thing> = Tribe::new("Lalani".to_string(), user.id);
    println!("Tribe: {tribe:?}");
    let tribe: Tribe<Thing, Thing> = db.create("tribe").content(tribe).await.unwrap();
    println!("Tribe: {tribe:?}");
    let mut result = db.query("SELECT * from tribe").await.unwrap();
    let result: Vec<Tribe<Thing, Thing>> = result.take(0).unwrap();
    println!("Result: {result:?}");
    #[derive(Deserialize, Debug)]
    #[allow(dead_code)]
    struct Response {
        leader: User<Thing>,
    }
    let mut result = db.query("SELECT leader.* from tribe").await.unwrap();
    let result: Vec<Response> = result.take(0).unwrap();
    println!("Result: {result:?}");
}
