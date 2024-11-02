use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use uuid::Uuid;
use crate::clients::ClientId;

pub(crate) type ProductId = String;


#[derive(sqlx::FromRow, Serialize, Deserialize, Clone)]
pub(crate) struct Product {
    pub id: ProductId,
    pub name: String,
    pub description: Option<String>,
    pub price: u64,

    pub main_image: Option<String>,
    pub second_image: Option<String>,
    pub third_image: Option<String>,
    pub fourth_image: Option<String>,

    pub size: bool,
    pub wheel_size: bool
}

impl Product {
    async fn get_all(pool: &MySqlPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Product>("SELECT * FROM product")
            .fetch_all(pool)
            .await
    }

    async fn get_product(pool: &MySqlPool, id: &str) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Product>("SELECT * FROM product WHERE id = ?")
            .bind(id)
            .fetch_one(pool)
            .await
    }

    // async fn get_characteristics(pool: &MySqlPool, id: &str) -> Result<Vec<ProductCharacteristic>, sqlx::Error> {
    //     ProductCharacteristic::get_product_characteristics(pool, id).await
    // }
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone)]
pub(crate) struct ProductCharacteristic {
    pub(crate) product: String,
    pub(crate) name: String,
    pub(crate) detail: String,
}

impl ProductCharacteristic {
    async fn get_product_characteristics(pool: &MySqlPool, product_id: &str) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, ProductCharacteristic>("SELECT * FROM product_characteristic WHERE product = ?")
            .bind(product_id)
            .fetch_all(pool)
            .await
    }
}


#[derive(sqlx::FromRow, Serialize, Deserialize, Clone)]
struct Review {
    pub id: String,
    pub product: ProductId,
    pub user: ClientId,
    pub review: String,
    pub stars: u8,
    pub username: Option<String>
}

const GET_ALL_REVIEWS_QUERY: &str = r#"SELECT
    reviews.id AS id,
    reviews.product AS product,
    reviews.user AS user,
    reviews.review AS review,
    reviews.stars AS stars,
    c.username AS username
FROM
    reviews
RIGHT JOIN clients c on reviews.user = c.id
WHERE product = ?;"#;

impl Review {
    async fn get_reviews_of_product(pool: &MySqlPool, product: &ProductId) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(GET_ALL_REVIEWS_QUERY)
            .bind(product)
            .fetch_all(pool)
            .await
    }

    async fn post(
        pool: &MySqlPool,
        product: ProductId,
        user: ClientId,
        review: String,
        stars: u8
    ) -> Result<Uuid, sqlx::Error>
    {
        let id= Uuid::new_v4();
        sqlx::query("INSERT INTO reviews (id, product, user, review, stars) VALUE (?, ?, ?, ?, ?);")
            .bind(id.to_string())
            .bind(product)
            .bind(user)
            .bind(review)
            .bind(stars)
            .execute(pool)
            .await
            .map(|_| id)
    }
}




pub(crate) mod routes {
    use std::ops::Deref;
    use axum::extract::{Path, Query, State};
    use axum::{Form, Json};
    use axum::response::IntoResponse;
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use tracing::error;
    use crate::{AppState, constants};
    use crate::declaration::ApiError;
    use crate::product::{Product, ProductCharacteristic, Review};

    pub(crate) async fn get_all_products(
        State(app_state): State<AppState>
    ) -> impl IntoResponse
    {
        let pool = app_state.database.get_pool().await;

        match Product::get_all(pool.deref()).await {
            Ok(p) => {
                Json(p).into_response()
            }
            Err(e) => {
                error!(target: "GetAllProducts", "Cannot fetch all products: {e:#?}");
                Json(ApiError::new(4001, "Cannot get products")).into_response()
            }
        }
    }

    pub(crate) async fn get_product_detail(
        State(app_state): State<AppState>,
        Path(id): Path<String>
    ) -> impl IntoResponse
    {
        let pool = app_state.database.get_pool().await;

        match Product::get_product(pool.deref(), id.as_str()).await {
            Ok(p) => {
                Json(p).into_response()
            }
            Err(e) => {
                error!(target: "GetAllProducts", "Cannot fetch the product '{}': {e:#?}", id);
                Json(ApiError::new(4002, "Cannot get the product")).into_response()
            }
        }
    }

    #[derive(Serialize, Deserialize)]
    pub(crate) struct ProductPageForm {
        page: u64
    }

    pub(crate) async fn get_product_page(
        State(app_state): State<AppState>,
        Query(params): Query<ProductPageForm>
    ) -> impl IntoResponse
    {
        let pool = app_state.database.get_pool().await;

        let start = if params.page > 0 {
            params.page * constants::PRODUCTS_PER_PAGE
        } else {
            0
        };

        let query = format!(
            "SELECT * FROM product ORDER BY id LIMIT {start}, {end};",
            end = start + constants::PRODUCTS_PER_PAGE
        );

        match sqlx::query_as::<_, Product>(query.as_str()).fetch_all(pool.deref()).await {
            Ok(products) => {
                Json(products).into_response()
            }
            Err(e) => {
                error!(target: "GetProductPage", "Cannot fetch the product page: {e:#?}");
                Json(ApiError::new(4003, "Cannot get the product page")).into_response()
            }
        }
    }

    pub(crate) async fn get_product_characteristics(
        State(app_state): State<AppState>,
        Path(product): Path<String>
    ) -> impl IntoResponse
    {
        let pool = app_state.database.get_pool().await;

        match ProductCharacteristic::get_product_characteristics(pool.deref(), product.as_str()).await {
            Ok(characteristics) => {
                Json(characteristics).into_response()
            }
            Err(e) => {
                error!(target: "GetProductCharacteristics", "Cannot fetch the product characteristics: {e:#?}");
                Json(ApiError::new(4004, "Cannot get the product characteristics")).into_response()
            }
        }
    }

    pub(crate) async fn get_product_reviews(
        State(app_state): State<AppState>,
        Path(product): Path<String>
    ) -> impl IntoResponse
    {
        let pool = app_state.database.get_pool().await;

        match Review::get_reviews_of_product(pool.deref(), &product).await {
            Ok(reviews) => {
                Json(reviews).into_response()
            }
            Err(e) => {
                error!(target: "GetProductReviews", "Cannot fetch the product reviews: {e:#?}");
                Json(ApiError::new(4005, "Cannot get the product reviews")).into_response()
            }
        }
    }

    #[derive(Serialize, Deserialize)]
    pub(crate) struct ReviewPost {
        review: String,
        stars: u8
    }

    pub(crate) async fn new_review(
        State(app_state): State<AppState>,
        Path(product): Path<String>,
        Form(review): Form<ReviewPost>
    ) -> impl IntoResponse
    {
        let pool = app_state.database.get_pool().await;

        match Review::post(pool.deref(), product, constants::TEST_USER_ID.to_string(), review.review, review.stars).await {
            Ok(id) => {
                Json(
                    json!({
                        "id": id.to_string(),
                        "code": 4100,
                        "message": "Review posted"
                    })
                ).into_response()
            }
            Err(e) => {
                error!(target: "NewReview", "Cannot post the review: {e:#?}");
                Json(ApiError::new(4006, "Cannot post the review")).into_response()
            }
        }
    }
}