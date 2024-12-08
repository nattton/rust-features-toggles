use std::env;

use self::models::*;
use self::schema::features::dsl::*;
use chrono::prelude::Utc;
use diesel::prelude::*;
use dotenvy::dotenv;
use features_toggles::*;

use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    routing::{get, patch, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route(
            "/v1/adm-admin-feature-management-service/features-toggles/renew",
            put(put_features_toggles),
        )
        .route(
            "/v1/adm-admin-feature-management-service/features-toggles/inquiry",
            post(post_features_toggles),
        )
        .route(
            "/v1/adm-admin-feature-management-service/features-toggles/update",
            patch(patch_features_toggles),
        )
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    // run our app with hyper, listening globally on port 3000
    dotenv().ok();
    let server_addr = env::var("SERVER_ADDR").unwrap_or("0.0.0.0:3000".to_string());
    println!("Running server on http://{}", server_addr);
    let listener = tokio::net::TcpListener::bind(server_addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, Makathon!"
}

async fn put_features_toggles(
    Json(payload): Json<PostFeaturesTogglesResponse>,
) -> (StatusCode, &'static str) {
    let conn = &mut establish_connection();
    let _ = diesel::delete(features).execute(conn);
    for category in payload.data.product_category_list {
        let new_category = NewFeature {
            feature_id: category.feature_id,
            category_id: category.category_id,
            product_id: "".to_string(),
            code: "".to_string(),
            name_th: category.name_th,
            name_en: category.name_en,
            is_active: category.is_active,
            updated_date_time: category.updated_date_time,
            updated_by_id: category.updated_by_id,
            updated_by_name: category.updated_by_name,
            sorting_order: category.category_sorting_order,
            feature_type: "category".to_string(),
        };
        create_feature(conn, &new_category);
    }

    for product in payload.data.product_list {
        for sub_product in product.sub_product_list {
            let new_category = NewFeature {
                feature_id: sub_product.feature_id,
                category_id: product.category_id,
                product_id: sub_product.product_id,
                code: sub_product.code,
                name_th: sub_product.name_th,
                name_en: sub_product.name_en,
                is_active: sub_product.is_active,
                updated_date_time: sub_product.updated_date_time,
                updated_by_id: sub_product.updated_by_id,
                updated_by_name: sub_product.updated_by_name,
                sorting_order: sub_product.sorting_order,
                feature_type: "product".to_string(),
            };
            create_feature(conn, &new_category);
        }
    }

    for service in payload.data.service_list {
        let new_category = NewFeature {
            feature_id: service.feature_id,
            category_id: 0,
            product_id: "".to_string(),
            code: "".to_string(),
            name_th: service.name_th,
            name_en: service.name_en,
            is_active: service.is_active,
            updated_date_time: service.updated_date_time,
            updated_by_id: service.updated_by_id,
            updated_by_name: service.updated_by_name,
            sorting_order: service.sorting_order,
            feature_type: "service".to_string(),
        };
        create_feature(conn, &new_category);
    }

    for system in payload.data.system_list {
        let new_category = NewFeature {
            feature_id: system.feature_id,
            category_id: 0,
            product_id: "".to_string(),
            code: "".to_string(),
            name_th: system.name_th,
            name_en: system.name_en,
            is_active: system.is_active,
            updated_date_time: system.updated_date_time,
            updated_by_id: system.updated_by_id,
            updated_by_name: system.updated_by_name,
            sorting_order: system.sorting_order,
            feature_type: "system".to_string(),
        };
        create_feature(conn, &new_category);
    }

    (StatusCode::OK, "Success")
}

async fn post_features_toggles(
    Json(payload): Json<PostFeaturesTogglesRequest>,
) -> (StatusCode, Json<PostFeaturesTogglesResponse>) {
    
    println!("{:?}", payload.feature_category);

    let connection = &mut establish_connection();
    let results_categories: Vec<Feature> = features
        .filter(feature_type.eq("category"))
        .select(Feature::as_select())
        .load(connection)
        .expect("Error loading features");

    let mut product_category_list: Vec<ProductCatgory> = vec![];
    let mut product_list: Vec<Product> = vec![];

    for category in results_categories {
        let product_category: ProductCatgory = ProductCatgory {
            feature_id: category.feature_id,
            category_id: category.category_id,
            name_th: category.name_th,
            name_en: category.name_en,
            is_active: category.is_active,
            updated_date_time: category.updated_date_time,
            updated_by_id: category.updated_by_id,
            updated_by_name: category.updated_by_name,
            category_sorting_order: category.sorting_order,
        };
        product_category_list.push(product_category);

        let results_products: Vec<Feature> = features
            .filter(feature_type.eq("product"))
            .filter(category_id.eq(category.category_id))
            .select(Feature::as_select())
            .load(connection)
            .expect("Error loading products");

        let mut sub_product_list: Vec<SubProduct> = vec![];
        for results_product in results_products {
            let sub_product: SubProduct = SubProduct {
                feature_id: results_product.feature_id,
                name_th: results_product.name_th,
                name_en: results_product.name_en,
                is_active: results_product.is_active,
                updated_date_time: results_product.updated_date_time,
                updated_by_id: results_product.updated_by_id,
                updated_by_name: results_product.updated_by_name,
                sorting_order: results_product.sorting_order,
                product_id: results_product.product_id,
                code: results_product.code,
            };
            sub_product_list.push(sub_product);
        }
        let product = Product {
            category_id: category.category_id,
            sub_product_list,
        };
        product_list.push(product);
    }

    let results_services: Vec<Feature> = features
        .filter(feature_type.eq("service"))
        .select(Feature::as_select())
        .load(connection)
        .expect("Error loading services");

    let mut service_list: Vec<NonCategory> = vec![];
    for post in results_services {
        let service: NonCategory = NonCategory {
            feature_id: post.feature_id,
            name_th: post.name_th,
            name_en: post.name_en,
            is_active: post.is_active,
            updated_date_time: post.updated_date_time,
            updated_by_id: post.updated_by_id,
            updated_by_name: post.updated_by_name,
            sorting_order: post.sorting_order,
        };
        service_list.push(service);
    }

    let results_systems: Vec<Feature> = features
        .filter(feature_type.eq("system"))
        .select(Feature::as_select())
        .load(connection)
        .expect("Error loading systems");

    let mut system_list: Vec<NonCategory> = vec![];
    for post in results_systems {
        let service: NonCategory = NonCategory {
            feature_id: post.feature_id,
            name_th: post.name_th,
            name_en: post.name_en,
            is_active: post.is_active,
            updated_date_time: post.updated_date_time,
            updated_by_id: post.updated_by_id,
            updated_by_name: post.updated_by_name,
            sorting_order: post.sorting_order,
        };
        system_list.push(service);
    }

    let data = Data {
        product_category_list,
        product_list,
        service_list,
        system_list,
    };

    let status: ApiResponse = ApiResponse {
        code: String::from("0000"),
        header: String::from(""),
        description: String::from("Success"),
    };

    let response: PostFeaturesTogglesResponse = PostFeaturesTogglesResponse { status, data };

    (StatusCode::OK, Json(response))
}

async fn patch_features_toggles(
    payload: Result<Json<PatchFeaturesTogglesRequest>, JsonRejection>,
) -> (StatusCode, Json<PatchFeaturesTogglesResponse>) {
    match payload {
        Ok(payload) => {
            println!("{:?}", payload.feature_list);
            let connection = &mut establish_connection();

            for feature in payload.feature_list.iter() {
                let feature =
                    diesel::update(features.filter(feature_id.eq(feature.feature_id.clone())))
                        .set((
                            is_active.eq(feature.is_active),
                            updated_date_time.eq(Utc::now().to_string()),
                        ))
                        .returning(Feature::as_returning())
                        .get_result(connection)
                        .unwrap();
                println!(
                    "Feature {}: {} set active to {}",
                    feature.feature_id, feature.name_th, feature.is_active
                )
            }

            let status: ApiResponse = ApiResponse {
                code: String::from("0000"),
                header: String::from(""),
                description: String::from("Success"),
            };
            let response: PatchFeaturesTogglesResponse = PatchFeaturesTogglesResponse {
                status,
                data: String::from("null"),
            };
            (StatusCode::OK, Json(response))
        }
        Err(_) => {
            let status: ApiResponse = ApiResponse {
                code: String::from("8000"),
                header: String::from("Bad request"),
                description: String::from("The server could not understand the request due to invalid syntax or missing mandatory field"),
            };
            let response: PatchFeaturesTogglesResponse = PatchFeaturesTogglesResponse {
                status,
                data: String::from("null"),
            };
            (StatusCode::OK, Json(response))
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PostFeaturesTogglesRequest {
    feature_category: Vec<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct PostFeaturesTogglesResponse {
    status: ApiResponse,
    data: Data,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ApiResponse {
    code: String,
    header: String,
    description: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Data {
    product_category_list: Vec<ProductCatgory>,
    product_list: Vec<Product>,
    service_list: Vec<NonCategory>,
    system_list: Vec<NonCategory>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ProductCatgory {
    feature_id: String,
    category_id: i32,
    name_th: String,
    name_en: String,
    category_sorting_order: i32,
    is_active: bool,
    updated_date_time: String,
    updated_by_id: String,
    updated_by_name: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Product {
    category_id: i32,
    sub_product_list: Vec<SubProduct>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct SubProduct {
    feature_id: String,
    product_id: String,
    code: String,
    name_th: String,
    name_en: String,
    sorting_order: i32,
    is_active: bool,
    updated_date_time: String,
    updated_by_id: String,
    updated_by_name: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct NonCategory {
    feature_id: String,
    name_th: String,
    name_en: String,
    sorting_order: i32,
    is_active: bool,
    updated_date_time: String,
    updated_by_id: String,
    updated_by_name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PatchFeaturesTogglesRequest {
    feature_list: Vec<FeatureRequest>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct FeatureRequest {
    feature_id: String,
    is_active: bool,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct PatchFeaturesTogglesResponse {
    status: ApiResponse,
    data: String,
}
