use actix_web::{
    get, post,put, delete, web::{self, Json, Path}, HttpResponse, Responder
};

use serde::Deserialize;

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::sql_query;
use chrono::{NaiveDateTime};

use crate::schema::articles::dsl as article_dsl; 
use crate::db_utils::PgPool;
use crate::db_models::Article;


#[derive(Deserialize)]
pub struct CreateArticleBody{
    pub title: String,
    pub content: String,
    pub created_by: Option<i32>,
    pub created_on: Option<NaiveDateTime>,
}

#[derive(Deserialize)]
pub struct UpdateArticleBody{
    pub title: String,
    pub content: String,
}

#[post("/user/{id}/create_article")]
pub async fn create_article(pool: web::Data<PgPool>,req_user: web::Json<CreateArticleBody>, path:web::Path<i32>)-> impl Responder {
    let pool = pool.clone();
    let article_data = req_user.into_inner();
    let id:i32= path.into_inner();

    let result = web::block(move || {
        let mut conn = pool.get().expect("Couldn't get db connection from Pool");

        diesel::insert_into(article_dsl::articles)
            .values((
                article_dsl::user_id.eq(id), 
                article_dsl::title.eq(article_data.title),
                article_dsl::content.eq(article_data.content),
                article_dsl::created_by.eq(article_data.created_by),
                article_dsl::created_on.eq(article_data.created_on),
            ))
            .execute(&mut conn)?;  // was getting error on this because  to sql query diesel is not applicable to chrono so to enable it added chrono in features in cargo.toml

        Ok::<_, diesel::result::Error>("Article created successfully".to_string())
    })
    .await;

    match result {
        Ok(Ok(message)) => HttpResponse::Created().body(message),
        Ok(Err(err)) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
        Err(err) => HttpResponse::InternalServerError().body(format!("Blocking error: {:?}", err)),
    }
}

#[get("/user/{id}/articles")]
pub async fn fetch_articles(pool: web::Data<PgPool>, path: web::Path<i32>)-> impl Responder {
    let pool = pool.clone();
    let userid= path.into_inner();
    let result = web::block(move || {
        let conn = pool.get().expect("Couldn't get db connection from Pool");
        let mut conn= conn;
        let articles_list = article_dsl::articles
                            .filter(article_dsl::user_id.eq(userid))
                            .load::<Article>(&mut conn)?;
        Ok::<_, diesel::result::Error>(articles_list)
    }).await;

    match result {
        Ok(Ok(articles_list)) => HttpResponse::Ok().json(articles_list), // Serializing as JSON
        Ok(Err(err)) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
        Err(err) => HttpResponse::InternalServerError().body(format!("Blocking error: {:?}", err)),
    }
}

#[put("/article/{id}")]
pub async fn update_article(pool: web::Data<PgPool>, path: web::Path<i32>, req_article: web::Json<UpdateArticleBody> )-> impl Responder {
    let pool = pool.clone();
    let article_id = path.into_inner();
    let article_data = req_article.into_inner();

    let result = web::block(move || {
        let mut conn = pool.get().expect("Couldn't get db connection from Pool");

        diesel::update(article_dsl::articles.find(article_id))
            .set((
                article_dsl::title.eq(article_data.title),
                article_dsl::content.eq(article_data.content),
            ))
            .execute(&mut conn)?;

        Ok::<_, diesel::result::Error>("Article updated successfully".to_string())
    })
    .await;

    match result {
        Ok(Ok(message)) => HttpResponse::Ok().body(message),
        Ok(Err(err)) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
        Err(err) => HttpResponse::InternalServerError().body(format!("Blocking error: {:?}", err)),
    }
}

#[delete("/article/{id}")]
pub async fn delete_article(pool: web::Data<PgPool>, path: web::Path<i32>)-> impl Responder {
    let pool = pool.clone();
    let article_id = path.into_inner();

    let result = web::block(move || {
        let mut conn = pool.get().expect("Couldn't get db connection from Pool");

        diesel::delete(article_dsl::articles.find(article_id))
            .execute(&mut conn)?;

        Ok::<_, diesel::result::Error>("Article deleted successfully".to_string())
    })
    .await;

    match result {
        Ok(Ok(message)) => HttpResponse::Ok().body(message),
        Ok(Err(err)) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
        Err(err) => HttpResponse::InternalServerError().body(format!("Blocking error: {:?}", err)),
    }
}
