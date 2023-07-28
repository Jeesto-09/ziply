use crate::model::{Domain, Url};
use crate::utils::validate_url;
use actix_web::{
    post,
    web::{self},
    HttpResponse,
};
use mongodb::{bson::doc, Client, Collection};
const DB_NAME: &str = "ziply";
const COLL_NAME: &str = "urls";

// Handler function to handle the GET request with url:id
// Redirect to orig_url
pub async fn handle_get(client: web::Data<Client>, req: web::Path<(String,)>) -> HttpResponse {
    // Using actix-web extractor to get the request
    let url_id = &req.into_inner().0;
    log::info!("Original url_id: {:?}", &url_id);
    let collection: Collection<Url> = client.database(DB_NAME).collection(COLL_NAME);
    let url = collection
        .find_one(Some(doc! {"url_id":url_id}), None)
        .await;
    if let Ok(Some(url)) = url {
        // Replace by redirecting to original url
        let orig_url = collection
            .find_one(Some(doc! {"orig_url":url.orig_url}), None)
            .await;
        log::info!("Redirecting to original url: {:?}", Some(&orig_url));
        let stored_result = match orig_url {
            Ok(Some(url)) => HttpResponse::Found()
                .append_header(("Location", url.orig_url))
                .finish(),
            Ok(None) => {
                HttpResponse::InternalServerError().body("Invalid Original Url".to_string())
            }
            Err(_) => HttpResponse::InternalServerError().body("Server Error".to_string()),
        };
        stored_result
    } else {
        log::warn!("Url not found");
        HttpResponse::InternalServerError()
            .body("you've clicked on a bad link or entered an invalid URL".to_string())
    }
}

#[post("/api/shorten_url")]
async fn shorten_url(client: web::Data<Client>, form: web::Form<Domain>) -> HttpResponse {
    // Check if the supplied url is a valid http/https link
    if validate_url(form.orig_url.as_str()) {
        // Initialize struct to be inserted:
        let short_url = Url::new(form.short_name.as_deref(), None, &form.orig_url);
        log::info!("Supplied short_name: {:?}", form.short_name.as_deref());
        // Convert 'short_url' instant to a Bson instance:
        let serialized_url = bson::to_bson(&short_url).unwrap();
        let document = serialized_url.as_document().unwrap();
        let collection = client.database(DB_NAME).collection(COLL_NAME);
        // Checking if the url is already in the DB
        let url = collection
            .find_one(Some(doc! {"orig_url": &form.orig_url}), None)
            .await;
        // If the url is in the DB then return the shortned url
        if let Ok(Some(its_indb)) = url {
            HttpResponse::Ok().json(its_indb)
            //HttpResponse::Ok().body("Already shortend".to_string())
        } else {
            // If the url is not the DB try insert the url
            let result = collection.insert_one(document.to_owned(), None).await;
            match result {
                Ok(insert_result) => {
                    // Get the inserted document's _id
                    if let Some(inserted_id) = insert_result.inserted_id.as_object_id() {
                        // Retrieve the inserted document using the _id
                        let filter = doc! { "_id": inserted_id };
                        let inserted_document = collection.find_one(Some(filter), None).await;
                        match inserted_document {
                            Ok(Some(document)) => {
                                // Deserialize the BSON document back to the original struct
                                if let Ok(url) =
                                    bson::from_bson::<Url>(bson::Bson::Document(document))
                                {
                                    // Return the newly inserted url to the customer
                                    HttpResponse::Ok().json(url)
                                } else {
                                    HttpResponse::InternalServerError()
                                        .body("Failed to deserialize the document")
                                }
                            }
                            Ok(None) => {
                                HttpResponse::InternalServerError().body("Document not found")
                            }
                            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                        }
                    } else {
                        HttpResponse::InternalServerError().body("Failed to retrieve _id")
                    }
                }
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        }
    } else {
        HttpResponse::BadRequest().body("Invalid URL ID".to_string())
    }
}
