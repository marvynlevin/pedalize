use serde::{Deserialize, Serialize};
use sqlx::{MySqlPool, Row};
use uuid::Uuid;
use crate::clients::ClientId;
use crate::product::ProductId;

pub(crate) type ShoppingCartId = String;

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone)]
pub(crate) struct ShoppingCart {
    pub user: ClientId,
    pub id: ShoppingCartId,
    pub articles: Vec<ShoppingCartArticles>
}

impl ShoppingCart {
    async fn create(pool: &MySqlPool, user: &str) -> Result<Uuid, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query("INSERT INTO shopping_cart (user, id) VALUE (?, ?)")
            .bind(user)
            .bind(id.to_string())
            .execute(pool)
            .await
            .map(|_| id)
    }

    async fn fetch_user_cart(pool: &MySqlPool, user: &str) -> Result<Option<Self>, sqlx::Error> {
        let row = sqlx::query("SELECT user, id FROM shopping_cart WHERE user = ?")
            .bind(user)
            .fetch_optional(pool)
            .await?;

        if row.is_none() { return Ok(None) }
        let row = row.unwrap();

        let user: String = row.get(0);
        let id: String = row.get(1);

        let articles = ShoppingCartArticles::fetch_all(pool, id.as_str()).await?;

        Ok(Some(Self { user, id, articles }))
    }

    async fn clear_articles(pool: &MySqlPool, shopping_cart_id: &str) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM shopping_cart_article WHERE shopping_cart_id = ?")
            .bind(shopping_cart_id)
            .execute(pool)
            .await
            .map(|_| ())
    }
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone)]
pub(crate) struct ShoppingCartArticles {
    pub product: ProductId,
    pub quantity: u64,
    pub shopping_cart_id: String,
}

impl ShoppingCartArticles {
    async fn fetch_all(pool: &MySqlPool, shopping_cart_id: &str) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM shopping_cart_article WHERE shopping_cart_id = ?;")
            .bind(shopping_cart_id)
            .fetch_all(pool)
            .await
    }

    async fn exists(pool: &MySqlPool, shopping_cart_id: &str, product: &str) -> Result<(), sqlx::Error> {
        sqlx::query("SELECT * FROM shopping_cart_article WHERE shopping_cart_id = ? AND product = ?")
            .bind(shopping_cart_id)
            .bind(product)
            .fetch_one(pool)
            .await
            .map(|_| ())
    }

    async fn add(pool: &MySqlPool, shopping_cart_id: &str, product: &str, quantity: u64) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO shopping_cart_article (shopping_cart_id, product, quantity) VALUE (?, ?, ?)")
            .bind(shopping_cart_id)
            .bind(product)
            .bind(quantity)
            .execute(pool)
            .await
            .map(|_| ())
    }

    async fn delete(pool: &MySqlPool, shopping_cart_id: &str, product: &str) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM shopping_cart_article WHERE shopping_cart_id = ? AND product = ?")
            .bind(shopping_cart_id)
            .bind(product)
            .execute(pool)
            .await
            .map(|_| ())
    }

    async fn add_quantity(pool: &MySqlPool, shopping_cart_id: &str, product: &str, quantity: u64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE shopping_cart_article SET quantity = quantity + ? WHERE shopping_cart_id = ? AND product = ?")
            .bind(quantity)
            .bind(shopping_cart_id)
            .bind(product)
            .execute(pool)
            .await
            .map(|_| ())
    }

    async fn set_quantity(pool: &MySqlPool, shopping_cart_id: &str, product: &str, quantity: u64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE shopping_cart_article SET quantity = ? WHERE shopping_cart_id = ? AND product = ?")
            .bind(quantity)
            .bind(shopping_cart_id)
            .bind(product)
            .execute(pool)
            .await
            .map(|_| ())
    }
}

/// Contain every routes for axum
pub(crate) mod routes {
    use std::ops::Deref;
    use axum::extract::State;
    use axum::{Form, Json};
    use axum::response::IntoResponse;
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use tracing::error;
    use crate::AppState;
    use crate::constants::TEST_USER_ID;
    use crate::declaration::ApiError;
    use crate::shopping_cart::{ShoppingCart, ShoppingCartArticles};

    /// Route: GET /shopping_cart
    ///
    /// Get the shopping cart of the user
    pub(crate) async fn get_shopping_cart(State(app_state): State<AppState>) -> impl IntoResponse {
        // for now, a default user will be setup, so no authentification is required
        // we will use the id defined in constants.rs by the constant TEST_USER_ID
        let pool = app_state.database.get_pool().await;

        let shopping_cart = match ShoppingCart::fetch_user_cart(pool.deref(), TEST_USER_ID).await {
            Ok(sc) => sc,
            Err(e) => {
                error!(target: "GetShoppingCart", "Cannot fetch the shopping cart: {e:#?}");
                return Json(
                    ApiError {
                        code: 5001,
                        message: "Unable to obtain the shopping cart for this user".into()
                    }
                ).into_response()
            }
        };

        match shopping_cart {
            Some(sc) => Json(sc).into_response(),
            None => {
                let new_id = match ShoppingCart::create(pool.deref(), TEST_USER_ID).await {
                    Ok(sc) => sc,
                    Err(e) => {
                        error!(target: "GetShoppingCart", "Cannot create the shopping cart: {e:#?}");
                        return Json(
                            ApiError {
                                code: 5002,
                                message: "Unable to create a new shopping cart for this user".into()
                            }
                        ).into_response()
                    }
                };

                Json(
                    ShoppingCart {
                        id: new_id.to_string(),
                        user: TEST_USER_ID.to_string(),
                        articles: Vec::new()
                    }
                ).into_response()
            }
        }
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub(crate) struct ShoppingCartArticleForm {
        product: String
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub(crate) struct ChangeArticleQuantity {
        product: String,
        quantity: u64
    }

    /// Route: POST /shopping_cart
    ///
    /// Add an article to the shopping cart
    pub(crate) async fn add_article_to_shopping_cart(
        State(app_state): State<AppState>,
        Form(form): Form<ShoppingCartArticleForm>
    ) -> impl IntoResponse
    {
        // for now, a default user will be setup, so no authentification is required
        // we will use the id defined in constants.rs by the constant TEST_USER_ID
        let pool = app_state.database.get_pool().await;

        let shopping_cart = match ShoppingCart::fetch_user_cart(pool.deref(), TEST_USER_ID).await {
            Ok(Some(sc)) => sc,
            Ok(None) => {
                let new_id = match ShoppingCart::create(pool.deref(), TEST_USER_ID).await {
                    Ok(sc) => sc,
                    Err(e) => {
                        error!(target: "GetShoppingCart", "Cannot create the shopping cart: {e:#?}");
                        return Json(
                            ApiError {
                                code: 5003,
                                message: "Unable to create a new shopping cart for this user".into()
                            }
                        ).into_response()
                    }
                };

                ShoppingCart {
                    id: new_id.to_string(),
                    user: TEST_USER_ID.to_string(),
                    articles: Vec::new()
                }
            }
            Err(e) => {
                error!(target: "GetShoppingCart", "Cannot fetch the shopping cart: {e:#?}");
                return Json(
                    ApiError {
                        code: 5004,
                        message: "Unable to obtain the shopping cart for this user".into()
                    }
                ).into_response()
            }
        };

        // if the article already exists, we just increment the quantity
        let exists = ShoppingCartArticles::exists(
            pool.deref(),
            shopping_cart.id.as_str(),
            form.product.as_str()
        ).await;

        if exists.is_ok() {
            let res = ShoppingCartArticles::add_quantity(
                pool.deref(),
                shopping_cart.id.as_str(),
                form.product.as_str(),
                1
            ).await;

            return match res {
                Ok(_) => Json(json!({"message": "Quantity increased", "code": 5100})).into_response(),
                Err(e) => {
                    error!(target: "GetShoppingCart", "Cannot increase the quantity for the article: {e:#?}");
                    Json(
                        ApiError {
                            code: 5006,
                            message: "Unable to increase the quantity for the article".into()
                        }
                    ).into_response()
                }
            }
        }

        // add the article to the shopping cart
        let res = ShoppingCartArticles::add(
            pool.deref(),
            shopping_cart.id.as_str(),
            form.product.as_str(),
            1
        ).await;

        match res {
            Ok(_) => Json(json!({"message": "Article added to the shopping cart", "code": 5101})).into_response(),
            Err(e) => {
                error!(target: "GetShoppingCart", "Cannot add the article to the shopping cart: {e:#?}");
                Json(
                    ApiError {
                        code: 5005,
                        message: "Unable to add the article to the shopping cart".into()
                    }
                ).into_response()
            }
        }
    }

    pub(crate) async fn edit_article_quantity(
        State(app_state): State<AppState>,
        Form(form): Form<ChangeArticleQuantity>
    ) -> impl IntoResponse
    {
        // for now, a default user will be setup, so no authentification is required
        // we will use the id defined in constants.rs by the constant TEST_USER_ID
        let pool = app_state.database.get_pool().await;

        let shopping_cart = match ShoppingCart::fetch_user_cart(pool.deref(), TEST_USER_ID).await {
            Ok(Some(sc)) => sc,
            Ok(None) => {
                let new_id = match ShoppingCart::create(pool.deref(), TEST_USER_ID).await {
                    Ok(sc) => sc,
                    Err(e) => {
                        error!(target: "GetShoppingCart", "Cannot create the shopping cart: {e:#?}");
                        return Json(
                            ApiError {
                                code: 5003,
                                message: "Unable to create a new shopping cart for this user".into()
                            }
                        ).into_response()
                    }
                };

                ShoppingCart {
                    id: new_id.to_string(),
                    user: TEST_USER_ID.to_string(),
                    articles: Vec::new()
                }
            }
            Err(e) => {
                error!(target: "GetShoppingCart", "Cannot fetch the shopping cart: {e:#?}");
                return Json(
                    ApiError {
                        code: 5004,
                        message: "Unable to obtain the shopping cart for this user".into()
                    }
                ).into_response()
            }
        };

        // if the article already exists, we just increment the quantity
        let exists = ShoppingCartArticles::exists(
            pool.deref(),
            shopping_cart.id.as_str(),
            form.product.as_str()
        ).await;
        if exists.is_ok() {
            let res = ShoppingCartArticles::set_quantity(
                pool.deref(),
                shopping_cart.id.as_str(),
                form.product.as_str(),
                form.quantity
            ).await;

            return match res {
                Ok(_) => Json(json!({"message": "Quantity increased", "code": 5100})).into_response(),
                Err(e) => {
                    error!(target: "GetShoppingCart", "Cannot increase the quantity for the article: {e:#?}");
                    Json(
                        ApiError {
                            code: 5006,
                            message: "Unable to increase the quantity for the article".into()
                        }
                    ).into_response()
                }
            }
        }

        // add the article to the shopping cart
        let res = ShoppingCartArticles::add(
            pool.deref(),
            shopping_cart.id.as_str(),
            form.product.as_str(),
            form.quantity
        ).await;

        match res {
            Ok(_) => Json(json!({"message": "Article added to the shopping cart", "code": 5102})).into_response(),
            Err(e) => {
                error!(target: "GetShoppingCart", "Cannot add the article to the shopping cart: {e:#?}");
                Json(
                    ApiError {
                        code: 5005,
                        message: "Unable to add the article to the shopping cart".into()
                    }
                ).into_response()
            }
        }
    }

    pub(crate) async fn delete_article(
        State(app_state): State<AppState>,
        Form(form): Form<ShoppingCartArticleForm>
    ) -> impl IntoResponse
    {
        // for now, a default user will be setup, so no authentification is required
        // we will use the id defined in constants.rs by the constant TEST_USER_ID
        let pool = app_state.database.get_pool().await;

        let shopping_cart = match ShoppingCart::fetch_user_cart(pool.deref(), TEST_USER_ID).await {
            Ok(Some(sc)) => sc,
            Ok(None) => {
                let new_id = match ShoppingCart::create(pool.deref(), TEST_USER_ID).await {
                    Ok(sc) => sc,
                    Err(e) => {
                        error!(target: "GetShoppingCart", "Cannot create the shopping cart: {e:#?}");
                        return Json(
                            ApiError {
                                code: 5003,
                                message: "Unable to create a new shopping cart for this user".into()
                            }
                        ).into_response()
                    }
                };

                ShoppingCart {
                    id: new_id.to_string(),
                    user: TEST_USER_ID.to_string(),
                    articles: Vec::new()
                }
            }
            Err(e) => {
                error!(target: "GetShoppingCart", "Cannot fetch the shopping cart: {e:#?}");
                return Json(
                    ApiError {
                        code: 5004,
                        message: "Unable to obtain the shopping cart for this user".into()
                    }
                ).into_response()
            }
        };

        let res = ShoppingCartArticles::delete(
            pool.deref(),
            shopping_cart.id.as_str(),
            form.product.as_str()
        ).await;

        match res {
            Ok(_) => Json(json!({"message": "Article deleted from the shopping cart", "code": 5103})).into_response(),
            Err(e) => {
                error!(target: "GetShoppingCart", "Cannot delete the article from the shopping cart: {e:#?}");
                Json(
                    ApiError {
                        code: 5007,
                        message: "Unable to delete the article from the shopping cart".into()
                    }
                ).into_response()
            }
        }
    }

    pub(crate) async fn clear_articles(
        State(app_state): State<AppState>
    ) -> impl IntoResponse
    {
        // for now, a default user will be setup, so no authentification is required
        // we will use the id defined in constants.rs by the constant TEST_USER_ID
        let pool = app_state.database.get_pool().await;

        let shopping_cart = match ShoppingCart::fetch_user_cart(pool.deref(), TEST_USER_ID).await {
            Ok(Some(sc)) => sc,
            Ok(None) => {
                let new_id = match ShoppingCart::create(pool.deref(), TEST_USER_ID).await {
                    Ok(sc) => sc,
                    Err(e) => {
                        error!(target: "GetShoppingCart", "Cannot create the shopping cart: {e:#?}");
                        return Json(
                            ApiError {
                                code: 5003,
                                message: "Unable to create a new shopping cart for this user".into()
                            }
                        ).into_response()
                    }
                };

                ShoppingCart {
                    id: new_id.to_string(),
                    user: TEST_USER_ID.to_string(),
                    articles: Vec::new()
                }
            }
            Err(e) => {
                error!(target: "GetShoppingCart", "Cannot fetch the shopping cart: {e:#?}");
                return Json(
                    ApiError {
                        code: 5004,
                        message: "Unable to obtain the shopping cart for this user".into()
                    }
                ).into_response()
            }
        };

        let res = ShoppingCart::clear_articles(
            pool.deref(),
            shopping_cart.id.as_str()
        ).await;

        match res {
            Ok(_) => Json(json!({"message": "Articles deleted from the shopping cart", "code": 5104})).into_response(),
            Err(e) => {
                error!(target: "GetShoppingCart", "Cannot delete the articles from the shopping cart: {e:#?}");
                Json(
                    ApiError {
                        code: 5008,
                        message: "Unable to delete the articles from the shopping cart".into()
                    }
                ).into_response()
            }
        }
    }
}