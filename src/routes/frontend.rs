// src/routes/frontend.rs
use askama::Template;
use axum::{
    Router, Form, routing::{get, post}, 
    response::{Html, IntoResponse, Redirect},
    extract::Path,
};
use axum_extra::extract::cookie::{CookieJar, Cookie};
use serde::Deserialize;
use tokio::try_join;

use crate::app::AppState;
use crate::error::AppError;
use crate::auth::user::{UnauthenticatedUser, User};
use crate::repository::Repository;
use crate::models::{Asset, OwnedAsset};

// src/routes/frontend.rs - parte do router
// src/routes/frontend.rs
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/login", get(login_page).post(login))
        .route("/logout", get(logout))
        .route("/assets", get(assets).post(purchase_asset))  // Mantém /assets
        .route("/assets/new", get(show_new_asset_form).post(create_new_asset))
        .route("/assets/{id}/edit", get(show_edit_asset_form).post(update_asset_form))
        .route("/assets/{id}/delete", post(delete_asset))
        .route("/assets/{id}", get(asset_detail))
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginPage;

async fn login_page() -> Result<Html<String>, AppError> {
    let html = LoginPage.render()?;
    Ok(Html(html))
}

#[derive(Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

async fn login(
    repository: Repository, 
    mut jar: CookieJar,
    Form(request): Form<LoginForm>
) -> Result<impl IntoResponse, AppError> {
    let unauth_user = UnauthenticatedUser::new(request.username, request.password);
    let user = match unauth_user.authenticate(&repository).await {
        Ok(user) => user,
        Err(AppError::UserDoesNotExist) => unauth_user.register(&repository).await?,
        Err(other_err) => return Err(other_err)
    };

    let token = user.auth_token()?;

    let cookie = Cookie::build(("token", token))
        .http_only(true)
        .path("/");
    
    jar = jar.add(cookie);
    
    Ok((jar, Redirect::to("/")))
}

async fn logout(jar: CookieJar) -> impl IntoResponse {
    let cookie = Cookie::build(("token", ""))
        .http_only(true)
        .path("/")
        .max_age(time::Duration::seconds(0));
    (jar.add(cookie), Redirect::to("/login"))
}

async fn index(user: Option<User>) -> Result<impl IntoResponse, AppError> {
    match user {
        Some(user) => {
            let html = format!(
                r#"
                <!DOCTYPE html>
                <html>
                <head>
                    <title>Dashboard - Wallet Live</title>
                    <script src="https://cdn.tailwindcss.com"></script>
                    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;800&display=swap" rel="stylesheet">
                    <style>
                        body {{ font-family: 'Inter', sans-serif; background: linear-gradient(135deg, #0f172a 0%, #1e1b4b 100%); }}
                        .glass-card {{ background: rgba(255,255,255,0.05); backdrop-filter: blur(10px); border: 1px solid rgba(255,255,255,0.1); }}
                        .glass-card:hover {{ background: rgba(255,255,255,0.08); border-color: rgba(255,255,255,0.2); transform: translateY(-2px); transition: all 0.3s ease; }}
                        .btn-cyan {{ background: rgba(34,211,238,0.15); border: 1px solid rgba(34,211,238,0.25); color: #22d3ee; transition: all 0.3s ease; }}
                        .btn-cyan:hover {{ background: rgba(34,211,238,0.25); box-shadow: 0 0 25px rgba(34,211,238,0.15); transform: translateY(-2px); }}
                        .btn-emerald {{ background: rgba(52,211,153,0.15); border: 1px solid rgba(52,211,153,0.25); color: #34d399; transition: all 0.3s ease; }}
                        .btn-emerald:hover {{ background: rgba(52,211,153,0.25); box-shadow: 0 0 25px rgba(52,211,153,0.15); transform: translateY(-2px); }}
                        .btn-purple {{ background: rgba(168,85,247,0.15); border: 1px solid rgba(168,85,247,0.25); color: #a855f7; transition: all 0.3s ease; }}
                        .btn-purple:hover {{ background: rgba(168,85,247,0.25); box-shadow: 0 0 25px rgba(168,85,247,0.15); transform: translateY(-2px); }}
                        .btn-rose {{ background: rgba(244,63,94,0.15); border: 1px solid rgba(244,63,94,0.25); color: #fb7185; transition: all 0.3s ease; }}
                        .btn-rose:hover {{ background: rgba(244,63,94,0.25); box-shadow: 0 0 25px rgba(244,63,94,0.15); transform: translateY(-2px); }}
                        .btn-red {{ background: rgba(239,68,68,0.15); border: 1px solid rgba(239,68,68,0.25); color: #f87171; transition: all 0.3s ease; }}
                        .btn-red:hover {{ background: rgba(239,68,68,0.25); box-shadow: 0 0 25px rgba(239,68,68,0.15); transform: translateY(-2px); }}
                        .fade-in {{ animation: fadeIn 0.5s ease-out; }}
                        @keyframes fadeIn {{ from {{ opacity: 0; transform: translateY(20px); }} to {{ opacity: 1; transform: translateY(0); }} }}
                    </style>
                </head>
                <body class="min-h-screen flex items-center justify-center p-4">
                    <div class="glass-card rounded-2xl p-8 shadow-2xl max-w-md w-full fade-in">
                        <div class="text-center">
                            <div class="inline-flex items-center justify-center w-20 h-20 bg-gradient-to-br from-cyan-500 to-blue-500 rounded-2xl shadow-lg mb-4">
                                <svg class="w-10 h-10 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                </svg>
                            </div>
                            <h1 class="text-3xl font-bold mb-2 text-white">Welcome, {}!</h1>
                            <p class="text-gray-400 text-sm mb-6">Your user ID: <strong class="text-cyan-400">{}</strong></p>
                        </div>
                        
                        <div class="space-y-3">
                            <a href="/assets" class="block w-full text-center btn-cyan px-4 py-2.5 rounded-xl text-sm font-medium transition-all duration-300">
                                📊 View Assets
                            </a>
                            <a href="/assets/new" class="block w-full text-center btn-emerald px-4 py-2.5 rounded-xl text-sm font-medium transition-all duration-300">
                                ➕ New Asset
                            </a>
                            <a href="/dashboard" class="block w-full text-center btn-purple px-4 py-2.5 rounded-xl text-sm font-medium transition-all duration-300">
                                📈 Dashboard
                            </a>
                            <a href="/chat" class="block w-full text-center btn-rose px-4 py-2.5 rounded-xl text-sm font-medium transition-all duration-300">
                                🤖 AI Chat
                            </a>
                            <a href="/logout" class="block w-full text-center btn-red px-4 py-2.5 rounded-xl text-sm font-medium transition-all duration-300">
                                🚪 Logout
                            </a>
                        </div>
                    </div>
                </body>
                </html>
                "#,
                user.username(),
                user.id()
            );
            Ok(Html(html).into_response())
        }
        None => Ok(Redirect::to("/login").into_response()),
    }
}

#[derive(Template)]
#[template(path = "assets.html")]
pub struct AssetsPage {
    pub owned_assets: Vec<OwnedAsset>,
    pub available_assets: Vec<Asset>,
    pub user: User,
    pub total_value: f64,
    pub total_invested: f64,
}

async fn assets(repository: Repository, user: User) -> Result<Html<String>, AppError> {
    let (owned_assets, available_assets) = try_join!(
        repository.list_owned_assets(user.id()),
        repository.list_assets()
    )?;

    let total_value: f64 = owned_assets.iter()
        .map(|a| a.unit_value * a.quantity_owned)
        .sum();
    
    let total_invested: f64 = owned_assets.iter()
        .flat_map(|a| a.purchase_history.iter())
        .map(|p| p.bought_for * p.quantity_bought)
        .sum();

    let html = AssetsPage {
        owned_assets,
        available_assets,
        user,
        total_value,
        total_invested,
    }
    .render()?;
    Ok(Html(html))
}

#[derive(Deserialize)]
pub struct PurchaseAssetForm {
    asset_id: i64,
    unit_value: f64,
    quantity: f64,
}

async fn purchase_asset(
    repository: Repository,
    user: User,
    Form(request): Form<PurchaseAssetForm>,
) -> Result<Redirect, AppError> {
    repository.insert_owned_asset(
        user.id(),
        request.asset_id,
        request.quantity,
        request.unit_value,
    )
    .await?;

    Ok(Redirect::to("/assets"))
}

// =============== CRUD DE ASSETS ===============

#[derive(Template)]
#[template(path = "new_asset.html")]
struct NewAssetPage {
    user: User,
}

async fn show_new_asset_form(user: User) -> Result<Html<String>, AppError> {
    let html = NewAssetPage { user }.render()?;
    Ok(Html(html))
}

#[derive(Deserialize)]
pub struct NewAssetForm {
    name: String,
    unit_value: f64,
}

async fn create_new_asset(
    _user: User,
    repository: Repository,
    Form(form): Form<NewAssetForm>,
) -> Result<Redirect, AppError> {
    repository.create_asset(form.name, form.unit_value).await?;
    Ok(Redirect::to("/assets"))
}

#[derive(Template)]
#[template(path = "edit_asset.html")]
struct EditAssetPage {
    user: User,
    asset: Asset,
}

async fn show_edit_asset_form(
    _user: User,
    repository: Repository,
    Path(asset_id): Path<i64>,
) -> Result<Html<String>, AppError> {
    let assets = repository.list_assets().await?;
    let asset = assets
        .into_iter()
        .find(|a| a.id == asset_id)
        .ok_or(AppError::AssetDoesNotExist)?;
    
    let html = EditAssetPage { user: _user, asset }.render()?;
    Ok(Html(html))
}

#[derive(Deserialize)]
pub struct EditAssetForm {
    name: Option<String>,
    unit_value: Option<f64>,
}

async fn update_asset_form(
    _user: User,
    repository: Repository,
    Path(asset_id): Path<i64>,
    Form(form): Form<EditAssetForm>,
) -> Result<Redirect, AppError> {
    repository.update_asset(asset_id, form.name, form.unit_value).await?;
    Ok(Redirect::to("/assets"))
}

async fn delete_asset(
    _user: User,
    repository: Repository,
    Path(asset_id): Path<i64>,
) -> Result<Redirect, AppError> {
    repository.delete_asset(asset_id).await?;
    Ok(Redirect::to("/assets"))
}

// Asset Detail - usando OwnedAsset
#[derive(Template)]
#[template(path = "asset_detail.html")]
struct AssetDetailPage {
    user: User,
    asset: OwnedAsset,
}

async fn asset_detail(
    user: User,
    repository: Repository,
    Path(asset_id): Path<i64>,
) -> Result<Html<String>, AppError> {
    let owned_assets = repository.list_owned_assets(user.id()).await?;
    
    let asset = owned_assets
        .into_iter()
        .find(|a| a.id == asset_id)
        .ok_or(AppError::AssetDoesNotExist)?;
    
    let html = AssetDetailPage {
        user,
        asset,
    }.render()?;
    
    Ok(Html(html))
}