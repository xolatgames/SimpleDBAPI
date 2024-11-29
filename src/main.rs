use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};

mod model;
use model::item::Item;

#[cfg(test)]
mod test;

const MONGODB_URI: &str = "mongodb://localhost:27017";
const DB_NAME: &str = "shoplist";
const ITEM_COLL: &str = "item";


#[post("/add_item")]
async fn add_item(client: web::Data<Client>, form: web::Form<Item>) -> HttpResponse {
    let collection = client.database(DB_NAME).collection(ITEM_COLL);
    let result = collection.insert_one(form.into_inner()).await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Item was added!"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}


#[get("/get_item/{item}")]
async fn get_item(client: web::Data<Client>, carrot: web::Path<String>) -> HttpResponse {
    let carrot = carrot.into_inner();
    let collection: Collection<Item> = client.database(DB_NAME).collection(ITEM_COLL);
    
    match collection.find_one(doc! {"carrot": &carrot }).await {
        Ok(Some(carrot)) => HttpResponse::Ok().json(carrot),
        Ok(None) => {
            HttpResponse::NotFound().body(format!("Here's no this item!"))
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}


async fn create_item_index(client: &Client) {
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc! { "item": 1 })
        .options(options)
        .build();
    client
        .database(DB_NAME)
        .collection::<Item>(ITEM_COLL)
        .create_index(model)
        .await
        .expect("Create an item index went successfully!");
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| MONGODB_URI.into());
    let client = Client::with_uri_str(uri).await.expect("Something went wrong...");
    create_item_index(&client).await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(add_item)
            .service(get_item)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
