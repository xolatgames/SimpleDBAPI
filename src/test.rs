use actix_web::{test::{call_and_read_body, init_service, TestRequest}, web::Bytes};

use super::*;


#[actix_web::test]
#[ignore = "MongoDB isn't started!"]
async fn test() {
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| MONGODB_URI.into());
    let client = Client::with_uri_str(uri).await.expect("Something went wrong...");

    let app = init_service(
        App::new()
            .app_data(web::Data::new(client))
            .service(add_item)
            .service(get_item)
    )
    .await;

    let item = Item {
        name: "Scissors".into(),
        amount: 1.into()
    };

    let req = TestRequest::post()
        .uri("/add_item")
        .set_form(&item)
        .to_request();

    let response = call_and_read_body(&app, req).await;
    assert_eq!(response, Bytes::from_static(b"Test item was added!"));
}