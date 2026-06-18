// src/routes/dashboard.rs
use askama::Template;
use axum::{
    Router, routing::get,
    response::{Html, IntoResponse},
    extract::Path,
};
use plotly::{
    Plot, 
    layout::Layout,
    common::{Mode, Line, Marker},
    Scatter,
};
use chrono::{Local, TimeZone};

use crate::{
    app::AppState,
    error::AppError,
    auth::user::User,
    repository::Repository,
    models::OwnedAsset,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/dashboard", get(dashboard))
        .route("/dashboard/asset/{id}", get(asset_detail))
}

#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardPage {
    pub user: User,
    pub total_value: f64,
    pub total_invested: f64,
    pub profit_loss: f64,
    pub profit_loss_percentage: f64,
    pub asset_count: usize,
    pub top_performers: Vec<OwnedAsset>,
    pub chart_html: String,
}

#[derive(Template)]
#[template(path = "asset_detail_dashboard.html")]
pub struct AssetDetailDashboardPage {
    pub user: User,
    pub asset: OwnedAsset,
    pub chart_html: String,
}

async fn dashboard(
    user: User,
    repository: Repository,
) -> Result<impl IntoResponse, AppError> {
    let owned_assets = repository.list_owned_assets(user.id()).await?;
    
    let total_value: f64 = owned_assets.iter()
        .map(|a| a.unit_value * a.quantity_owned)
        .sum();
    
    let total_invested: f64 = owned_assets.iter()
        .flat_map(|a| a.purchase_history.iter())
        .map(|p| p.bought_for * p.quantity_bought)
        .sum();
    
    let profit_loss = total_value - total_invested;
    let profit_loss_percentage = if total_invested > 0.0 {
        (profit_loss / total_invested) * 100.0
    } else {
        0.0
    };
    
    let mut top_performers = owned_assets.clone();
    top_performers.sort_by(|a, b| b.value_delta.partial_cmp(&a.value_delta).unwrap());
    top_performers.truncate(5);
    
    let chart_html = generate_performance_chart(&owned_assets);
    
    let page = DashboardPage {
        user,
        total_value,
        total_invested,
        profit_loss,
        profit_loss_percentage,
        asset_count: owned_assets.len(),
        top_performers,
        chart_html,
    };
    
    Ok(Html(page.render()?))
}

async fn asset_detail(
    user: User,
    repository: Repository,
    Path(asset_id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let owned_assets = repository.list_owned_assets(user.id()).await?;
    
    let asset = owned_assets
        .into_iter()
        .find(|a| a.id == asset_id)
        .ok_or(AppError::AssetDoesNotExist)?;
    
    let chart_html = generate_asset_detail_chart(&asset);
    
    let page = AssetDetailDashboardPage {
        user,
        asset,
        chart_html,
    };
    
    Ok(Html(page.render()?))
}

fn generate_performance_chart(assets: &[OwnedAsset]) -> String {
    if assets.is_empty() {
        return String::from("<p class='text-gray-400 text-center py-8'>No assets to display chart</p>");
    }
    
    let mut plot = Plot::new();
    
    // Gerar dados para os últimos 30 dias
    let dates: Vec<String> = (0..30)
        .map(|i| {
            let date = Local::now() - chrono::Duration::days(i);
            date.format("%Y-%m-%d").to_string()
        })
        .rev()
        .collect();
    
    // Para cada asset, criar uma linha no gráfico
    for asset in assets {
        let mut values = Vec::new();
        let base_value = asset.unit_value * asset.quantity_owned;
        
        // Simular variação diária (se não tiver histórico real)
        if asset.purchase_history.is_empty() {
            let mut current_value = base_value * 0.9;
            for _ in 0..30 {
                let variation = rand::random::<f64>() * 0.06 - 0.03;
                current_value *= 1.0 + variation;
                values.push(current_value);
            }
        } else {
            let mut current_value = base_value * 0.8;
            for i in 0..30 {
                if i < asset.purchase_history.len() {
                    let p = &asset.purchase_history[i % asset.purchase_history.len()];
                    current_value = p.bought_for * asset.quantity_owned;
                } else {
                    let variation = rand::random::<f64>() * 0.06 - 0.03;
                    current_value *= 1.0 + variation;
                }
                values.push(current_value);
            }
        }
        
        let trace = Scatter::new(dates.clone(), values)
            .mode(Mode::LinesMarkers)
            .name(&asset.name)
            .line(Line::new().width(2.0))
            .marker(Marker::new().size(4));
        
        plot.add_trace(trace);
    }
    
    let layout = Layout::new()
        .title("Portfolio Performance (30 days)")
        .width(800)
        .height(300)  // Reduzido de 400 para 300
        .paper_background_color("rgba(0,0,0,0)")
        .plot_background_color("rgba(0,0,0,0)")
        .x_axis(plotly::layout::Axis::new().title("Date").grid_color("rgba(255,255,255,0.1)"))
        .y_axis(plotly::layout::Axis::new().title("Value ($)").grid_color("rgba(255,255,255,0.1)"));
    
    plot.set_layout(layout);
    
    plot.to_inline_html(Some("performance-chart"))
}

fn generate_asset_detail_chart(asset: &OwnedAsset) -> String {
    if asset.purchase_history.is_empty() {
        return String::from("<p class='text-gray-400 text-center py-8'>No purchase history to display chart</p>");
    }
    
    let mut plot = Plot::new();
    
    let dates: Vec<String> = asset.purchase_history
        .iter()
        .map(|p| {
            let timestamp = p.bought_at.unix_timestamp();
            let dt = Local.timestamp_opt(timestamp, 0).unwrap();
            dt.format("%Y-%m-%d").to_string()
        })
        .collect();
    
    let bought_prices: Vec<f64> = asset.purchase_history
        .iter()
        .map(|p| p.bought_for)
        .collect();
    
    let current_prices: Vec<f64> = asset.purchase_history
        .iter()
        .map(|_| asset.unit_value)
        .collect();
    
    let bought_trace = Scatter::new(dates.clone(), bought_prices)
        .mode(Mode::LinesMarkers)
        .name("Purchase Price")
        .line(Line::new().width(2.0).color("orange"))
        .marker(Marker::new().size(6).color("orange"));
    
    let current_trace = Scatter::new(dates.clone(), current_prices)
        .mode(Mode::LinesMarkers)
        .name("Current Price")
        .line(Line::new().width(2.0).color("cyan"))
        .marker(Marker::new().size(6).color("cyan"));
    
    plot.add_trace(bought_trace);
    plot.add_trace(current_trace);
    
    let layout = Layout::new()
        .title(&format!("{} Price History", asset.name))
        .width(800)
        .height(300)  // Reduzido de 400 para 300
        .paper_background_color("rgba(0,0,0,0)")
        .plot_background_color("rgba(0,0,0,0)")
        .x_axis(plotly::layout::Axis::new().title("Date").grid_color("rgba(255,255,255,0.1)"))
        .y_axis(plotly::layout::Axis::new().title("Price ($)").grid_color("rgba(255,255,255,0.1)"));
    
    plot.set_layout(layout);
    
    plot.to_inline_html(Some(&format!("asset-{}-chart", asset.id)))
}