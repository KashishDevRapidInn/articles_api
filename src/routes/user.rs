use actix_web::{
    get, post,put, delete, web::{self, Json, Path}, HttpResponse, Responder
};

use serde::Deserialize;

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::sql_query;

use crate::schema::users::dsl as user_dsl; 
use crate::db_utils::PgPool;
use crate::db_models::User;

#[derive(Deserialize)]
pub struct CreateUserBody {
    pub first_name: String,
    pub last_name: String,
}

#[get("/test")]
pub async fn test_connection(pool: web::Data<PgPool>)->impl Responder{
    let pool = pool.clone();
    
    let result= web::block(move||{
        let mut conn: r2d2::PooledConnection<ConnectionManager<PgConnection>>= pool.get().expect("Couldn't get db connection from Pool");

        let _= sql_query("SELECT 1").execute(&mut *conn); 
        
        Ok::<_, diesel::result::Error>("DB Connection Successul".to_string())
    })  
    .await;

    match result{
        Ok(Ok(message)) => HttpResponse::Ok().body(message),
        Ok(Err(err)) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
        Err(err) => HttpResponse::InternalServerError().body(format!("Blocking error: {:?}", err)),
    }
}
#[get("/users")]
pub async fn fetch_users(pool: web::Data<PgPool>)-> impl Responder {
      let pool = pool.clone();
    
    let result = web::block(move || {
        let conn = pool.get().expect("Couldn't get db connection from Pool");
        let mut conn= conn;
        let user_list = user_dsl::users.load::<User>(&mut conn)?;
        Ok::<_, diesel::result::Error>(user_list)
    }).await;

    match result {
        Ok(Ok(user_list)) => HttpResponse::Ok().json(user_list), // Serialize as JSON
        Ok(Err(err)) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
        Err(err) => HttpResponse::InternalServerError().body(format!("Blocking error: {:?}", err)),
    }
}

#[post("/create_user")]
pub async fn create_user(pool: web::Data<PgPool>,req_user: web::Json<CreateUserBody>)-> impl Responder {
    let pool = pool.clone();
    let user_data = req_user.into_inner();

    let result = web::block(move || {
        let mut conn = pool.get().expect("Couldn't get db connection from Pool");

        diesel::insert_into(user_dsl::users)
            .values((
                user_dsl::first_name.eq(user_data.first_name),
                user_dsl::last_name.eq(user_data.last_name),
            ))
            .execute(&mut conn)?;

        Ok::<_, diesel::result::Error>("User created successfully".to_string())
    })
    .await;

    match result {
        Ok(Ok(message)) => HttpResponse::Created().body(message),
        Ok(Err(err)) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
        Err(err) => HttpResponse::InternalServerError().body(format!("Blocking error: {:?}", err)),
    }
}

#[put("/user/{id}")]
pub async fn update_user(pool: web::Data<PgPool>, path: web::Path<i32>, req_user: web::Json<CreateUserBody>)-> impl Responder {
    let pool = pool.clone();
    let user_id = path.into_inner();
    let user_data = req_user.into_inner();

    let result = web::block(move || {
        let mut conn = pool.get().expect("Couldn't get db connection from Pool");

        diesel::update(user_dsl::users.find(user_id))
            .set((
                user_dsl::first_name.eq(user_data.first_name),
                user_dsl::last_name.eq(user_data.last_name),
            ))
            .execute(&mut conn)?;

        Ok::<_, diesel::result::Error>("User updated successfully".to_string())
    })
    .await;

    match result {
        Ok(Ok(message)) => HttpResponse::Ok().body(message),
        Ok(Err(err)) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
        Err(err) => HttpResponse::InternalServerError().body(format!("Blocking error: {:?}", err)),
    }
}

#[delete("/user/{id}")]
pub async fn delete_user(pool: web::Data<PgPool>, path: web::Path<i32> )-> impl Responder {
    let pool = pool.clone();
    let user_id = path.into_inner();

    let result = web::block(move || {
        let mut conn = pool.get().expect("Couldn't get db connection from Pool");

        diesel::delete(user_dsl::users.find(user_id))
            .execute(&mut conn)?;

        Ok::<_, diesel::result::Error>("User deleted successfully".to_string())
    })
    .await;

    match result {
        Ok(Ok(message)) => HttpResponse::Ok().body(message),
        Ok(Err(err)) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
        Err(err) => HttpResponse::InternalServerError().body(format!("Blocking error: {:?}", err)),
    }
}