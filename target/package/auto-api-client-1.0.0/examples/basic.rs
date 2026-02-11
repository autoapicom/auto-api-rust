// Auto API Rust Client — Complete usage example.
//
// Replace "your-api-key" with your actual API key from https://auto-api.com
//
// Run: cargo run --example basic

use auto_api_client::{Client, Error, OfferData, OffersParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("your-api-key");
    let source = "encar";

    // --- Get available filters ---

    let filters = client.get_filters(source).await?;
    println!("Filters: {}", filters);

    // --- Search offers with filters ---

    let offers = client
        .get_offers(
            source,
            &OffersParams {
                page: 1,
                brand: Some("Hyundai".into()),
                year_from: Some(2020),
                price_to: Some(50000),
                ..Default::default()
            },
        )
        .await?;

    println!("\n--- Offers (page {}) ---", offers.meta.page);
    for item in &offers.result {
        if let Ok(d) = serde_json::from_value::<OfferData>(item.data.clone()) {
            println!(
                "{} {} {} — ${} ({} km)",
                d.mark, d.model, d.year, d.price, d.km_age
            );
        }
    }

    // Pagination
    if offers.meta.next_page > 0 {
        let next_page = client
            .get_offers(
                source,
                &OffersParams {
                    page: offers.meta.next_page,
                    brand: Some("Hyundai".into()),
                    year_from: Some(2020),
                    ..Default::default()
                },
            )
            .await?;
        println!("Next page has {} offers", next_page.result.len());
    }

    // --- Get single offer ---

    let inner_id = if !offers.result.is_empty() {
        offers.result[0].inner_id.clone()
    } else {
        "40427050".to_string()
    };

    let offer = client.get_offer(source, &inner_id).await?;
    println!("\n--- Single offer ---");
    if let Some(first) = offer.result.first() {
        if let Ok(d) = serde_json::from_value::<OfferData>(first.data.clone()) {
            println!("URL: {}", d.url);
            println!("Seller: {}", d.seller_type);
            println!("Images: {}", d.images.len());
        }
    }

    // --- Track changes ---

    let change_id = client.get_change_id(source, "2025-01-15").await?;
    println!(
        "\n--- Changes from 2025-01-15 (change_id: {}) ---",
        change_id
    );

    let changes = client.get_changes(source, change_id).await?;
    for change in &changes.result {
        println!("[{}] {}", change.change_type, change.inner_id);
    }

    if changes.meta.next_change_id > 0 {
        let more_changes = client
            .get_changes(source, changes.meta.next_change_id)
            .await?;
        println!("Next batch: {} changes", more_changes.result.len());
    }

    // --- Get offer by URL ---

    let info = client
        .get_offer_by_url("https://www.encar.com/dc/dc_cardetailview.do?carid=40427050")
        .await?;
    println!("\n--- Offer by URL ---");
    println!(
        "{} {} — ${}",
        info["mark"], info["model"], info["price"]
    );

    // --- Error handling ---

    let bad_client = Client::new("invalid-key");
    match bad_client
        .get_offers(
            "encar",
            &OffersParams {
                page: 1,
                ..Default::default()
            },
        )
        .await
    {
        Ok(offers) => println!("Got {} offers", offers.result.len()),
        Err(Error::Auth {
            status_code,
            message,
        }) => {
            println!("\nAuth error: {} (HTTP {})", message, status_code);
        }
        Err(Error::Api {
            status_code,
            message,
            ..
        }) => {
            println!("\nAPI error: {} (HTTP {})", message, status_code);
        }
        Err(Error::Network(e)) => {
            println!("\nNetwork error: {}", e);
        }
    }

    Ok(())
}
