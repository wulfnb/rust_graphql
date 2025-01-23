use async_graphql::{Context, EmptySubscription, Object, Schema, SimpleObject};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use actix_web::{guard, web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// Define the User struct with the required GraphQL trait
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
struct User {
    id: String,
    name: String,
    email: String,
}

// In-memory data store
type UserStore = Arc<Mutex<Vec<User>>>;

// Define the Query object
struct Query;

#[Object]
impl Query {
    // Fetch all users
    async fn users(&self, ctx: &Context<'_>) -> Vec<User> {
        let store = ctx.data_unchecked::<UserStore>();
        let users = store.lock().unwrap();
        users.clone()
    }

    // Fetch a single user by ID
    async fn user(&self, ctx: &Context<'_>, id: String) -> Option<User> {
        let store = ctx.data_unchecked::<UserStore>();
        let users = store.lock().unwrap();
        users.iter().find(|user| user.id == id).cloned()
    }
}

// Define the Mutation object
struct Mutation;

#[Object]
impl Mutation {
    // Create a new user
    async fn create_user(&self, ctx: &Context<'_>, id: String, name: String, email: String) -> User {
        let store = ctx.data_unchecked::<UserStore>();
        let mut users = store.lock().unwrap();
        let user = User { id, name, email };
        users.push(user.clone());
        user
    }

    // Update an existing user
    async fn update_user(
        &self,
        ctx: &Context<'_>,
        id: String,
        name: Option<String>,
        email: Option<String>,
    ) -> Option<User> {
        let store = ctx.data_unchecked::<UserStore>();
        let mut users = store.lock().unwrap();
        if let Some(user) = users.iter_mut().find(|user| user.id == id) {
            if let Some(new_name) = name {
                user.name = new_name;
            }
            if let Some(new_email) = email {
                user.email = new_email;
            }
            return Some(user.clone());
        }
        None
    }

    // Delete a user by ID
    async fn delete_user(&self, ctx: &Context<'_>, id: String) -> Option<User> {
        let store = ctx.data_unchecked::<UserStore>();
        let mut users = store.lock().unwrap();
        if let Some(index) = users.iter().position(|user| user.id == id) {
            return Some(users.remove(index));
        }
        None
    }
}

// GraphQL Playground handler
async fn graphql_playground() -> impl Responder {
    use async_graphql::http::GraphQLPlaygroundConfig;

    actix_web::HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(async_graphql::http::playground_source(
            GraphQLPlaygroundConfig::new("/graphql"),
        ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the in-memory data store
    let store = Arc::new(Mutex::new(vec![
        User {
            id: "1".to_string(),
            name: "John Doe".to_string(),
            email: "john.doe@example.com".to_string(),
        },
        User {
            id: "2".to_string(),
            name: "Jane Doe".to_string(),
            email: "jane.doe@example.com".to_string(),
        },
    ]));

    // Create the schema
    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(store.clone())
        .finish();

    // Start the Actix Web server
    println!("GraphQL server running at http://localhost:8080/graphql");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .service(
                web::resource("/graphql")
                    .guard(guard::Post())
                    .to(|schema: web::Data<Schema<Query, Mutation, EmptySubscription>>, req: GraphQLRequest| async move {
                        GraphQLResponse::from(schema.execute(req.into_inner()).await)
                    }),
            )
            .service(
                web::resource("/graphql").guard(guard::Get()).to(graphql_playground),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
