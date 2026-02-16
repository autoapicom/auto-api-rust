use auto_api_client::{Client, Error, OffersParams};
use mockito::{Mock, ServerGuard};

async fn setup() -> (ServerGuard, Client) {
    let server = mockito::Server::new_async().await;
    let mut client = Client::new("test-key");
    client.set_base_url(server.url().as_str());
    (server, client)
}

fn json_mock(server: &mut ServerGuard, method: &str, path: &str, status: usize, body: &str) -> Mock {
    server
        .mock(method, path)
        .match_query(mockito::Matcher::Any)
        .with_status(status)
        .with_header("content-type", "application/json")
        .with_body(body)
        .create()
}

// ── get_filters ─────────────────────────────────────────────────

#[tokio::test]
async fn test_get_filters_returns_parsed_response() {
    let (mut server, client) = setup().await;
    let mock = json_mock(
        &mut server,
        "GET",
        "/api/v2/encar/filters",
        200,
        r#"{"brands":["Toyota","Honda"],"body_types":["sedan","suv"]}"#,
    );

    let result = client.get_filters("encar").await.unwrap();

    mock.assert();
    assert_eq!(result["brands"][0], "Toyota");
    assert_eq!(result["brands"][1], "Honda");
}

#[tokio::test]
async fn test_get_filters_calls_correct_endpoint() {
    let (mut server, client) = setup().await;
    let mock = json_mock(&mut server, "GET", "/api/v2/encar/filters", 200, "{}");

    client.get_filters("encar").await.unwrap();

    mock.assert();
}

#[tokio::test]
async fn test_get_filters_includes_api_key() {
    let (mut server, client) = setup().await;
    let mock = server
        .mock("GET", "/api/v2/encar/filters")
        .match_query(mockito::Matcher::UrlEncoded("api_key".into(), "test-key".into()))
        .with_status(200)
        .with_body("{}")
        .create();

    client.get_filters("encar").await.unwrap();

    mock.assert();
}

// ── get_offers ──────────────────────────────────────────────────

#[tokio::test]
async fn test_get_offers_returns_data() {
    let (mut server, client) = setup().await;
    let body = r#"{"result":[{"id":1,"inner_id":"a1","change_type":"added","created_at":"2024-01-15","data":{}}],"meta":{"page":1,"next_page":2,"limit":20}}"#;
    let mock = json_mock(&mut server, "GET", "/api/v2/encar/offers", 200, body);

    let result = client
        .get_offers("encar", &OffersParams { page: 1, ..Default::default() })
        .await
        .unwrap();

    mock.assert();
    assert_eq!(result.result.len(), 1);
    assert_eq!(result.meta.page, 1);
}

#[tokio::test]
async fn test_get_offers_with_filters() {
    let (mut server, client) = setup().await;
    let mock = server
        .mock("GET", "/api/v2/mobile_de/offers")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("page".into(), "2".into()),
            mockito::Matcher::UrlEncoded("brand".into(), "BMW".into()),
        ]))
        .with_status(200)
        .with_body(r#"{"result":[],"meta":{"page":2,"next_page":0,"limit":20}}"#)
        .create();

    client
        .get_offers(
            "mobile_de",
            &OffersParams {
                page: 2,
                brand: Some("BMW".into()),
                ..Default::default()
            },
        )
        .await
        .unwrap();

    mock.assert();
}

// ── get_offer ───────────────────────────────────────────────────

#[tokio::test]
async fn test_get_offer_passes_inner_id() {
    let (mut server, client) = setup().await;
    let mock = server
        .mock("GET", "/api/v2/encar/offer")
        .match_query(mockito::Matcher::UrlEncoded("inner_id".into(), "abc123".into()))
        .with_status(200)
        .with_body(r#"{"result":[{"id":1,"inner_id":"abc123","change_type":"","created_at":"","data":{}}],"meta":{"page":1,"next_page":0,"limit":20}}"#)
        .create();

    let result = client.get_offer("encar", "abc123").await.unwrap();

    mock.assert();
    assert_eq!(result.result.len(), 1);
}

// ── get_change_id ───────────────────────────────────────────────

#[tokio::test]
async fn test_get_change_id_returns_integer() {
    let (mut server, client) = setup().await;
    let mock = json_mock(&mut server, "GET", "/api/v2/encar/change_id", 200, r#"{"change_id":42567}"#);

    let result = client.get_change_id("encar", "2024-01-15").await.unwrap();

    mock.assert();
    assert_eq!(result, 42567);
}

#[tokio::test]
async fn test_get_change_id_passes_date() {
    let (mut server, client) = setup().await;
    let mock = server
        .mock("GET", "/api/v2/encar/change_id")
        .match_query(mockito::Matcher::UrlEncoded("date".into(), "2024-01-15".into()))
        .with_status(200)
        .with_body(r#"{"change_id":0}"#)
        .create();

    client.get_change_id("encar", "2024-01-15").await.unwrap();

    mock.assert();
}

#[tokio::test]
async fn test_get_change_id_returns_zero() {
    let (mut server, client) = setup().await;
    let mock = json_mock(&mut server, "GET", "/api/v2/encar/change_id", 200, r#"{"change_id":0}"#);

    let result = client.get_change_id("encar", "2024-01-01").await.unwrap();

    mock.assert();
    assert_eq!(result, 0);
}

// ── get_changes ─────────────────────────────────────────────────

#[tokio::test]
async fn test_get_changes_returns_feed() {
    let (mut server, client) = setup().await;
    let body = r#"{"result":[{"id":1,"inner_id":"new1","change_type":"added","created_at":"2024-01-15","data":{}}],"meta":{"cur_change_id":42567,"next_change_id":42568,"limit":500}}"#;
    let mock = json_mock(&mut server, "GET", "/api/v2/encar/changes", 200, body);

    let result = client.get_changes("encar", 42567).await.unwrap();

    mock.assert();
    assert_eq!(result.result.len(), 1);
    assert_eq!(result.meta.cur_change_id, 42567);
}

#[tokio::test]
async fn test_get_changes_passes_change_id() {
    let (mut server, client) = setup().await;
    let mock = server
        .mock("GET", "/api/v2/encar/changes")
        .match_query(mockito::Matcher::UrlEncoded("change_id".into(), "42567".into()))
        .with_status(200)
        .with_body(r#"{"result":[],"meta":{"cur_change_id":0,"next_change_id":0,"limit":500}}"#)
        .create();

    client.get_changes("encar", 42567).await.unwrap();

    mock.assert();
}

// ── get_offer_by_url ────────────────────────────────────────────

#[tokio::test]
async fn test_get_offer_by_url_returns_data() {
    let (mut server, client) = setup().await;
    let mock = json_mock(
        &mut server,
        "POST",
        "/api/v1/offer/info",
        200,
        r#"{"brand":"BMW","model":"X5","price":45000}"#,
    );

    let result = client
        .get_offer_by_url("https://www.encar.com/car/123")
        .await
        .unwrap();

    mock.assert();
    assert_eq!(result["brand"], "BMW");
}

#[tokio::test]
async fn test_get_offer_by_url_uses_post() {
    let (mut server, client) = setup().await;
    let mock = server
        .mock("POST", "/api/v1/offer/info")
        .with_status(200)
        .with_body("{}")
        .create();

    client.get_offer_by_url("https://example.com/car/123").await.unwrap();

    mock.assert();
}

#[tokio::test]
async fn test_get_offer_by_url_sends_api_key_header() {
    let (mut server, client) = setup().await;
    let mock = server
        .mock("POST", "/api/v1/offer/info")
        .match_header("x-api-key", "test-key")
        .with_status(200)
        .with_body("{}")
        .create();

    client.get_offer_by_url("https://example.com/car/123").await.unwrap();

    mock.assert();
}

#[tokio::test]
async fn test_get_offer_by_url_sends_content_type() {
    let (mut server, client) = setup().await;
    let mock = server
        .mock("POST", "/api/v1/offer/info")
        .match_header("content-type", "application/json")
        .with_status(200)
        .with_body("{}")
        .create();

    client.get_offer_by_url("https://example.com/car/123").await.unwrap();

    mock.assert();
}

#[tokio::test]
async fn test_get_offer_by_url_sends_url_in_body() {
    let (mut server, client) = setup().await;
    let mock = server
        .mock("POST", "/api/v1/offer/info")
        .match_body(mockito::Matcher::JsonString(
            r#"{"url":"https://example.com/car/123"}"#.into(),
        ))
        .with_status(200)
        .with_body("{}")
        .create();

    client.get_offer_by_url("https://example.com/car/123").await.unwrap();

    mock.assert();
}

// ── Custom API version ──────────────────────────────────────────

#[tokio::test]
async fn test_custom_api_version() {
    let (mut server, mut client) = setup().await;
    client.set_api_version("v3");
    let mock = json_mock(&mut server, "GET", "/api/v3/encar/filters", 200, "{}");

    client.get_filters("encar").await.unwrap();

    mock.assert();
}

// ── Error handling ──────────────────────────────────────────────

#[tokio::test]
async fn test_api_error_on_server_error() {
    let (mut server, client) = setup().await;
    let mock = json_mock(
        &mut server,
        "GET",
        "/api/v2/encar/filters",
        500,
        r#"{"message":"Internal server error"}"#,
    );

    let result = client.get_filters("encar").await;

    mock.assert();
    assert!(result.is_err());
    match result.unwrap_err() {
        Error::Api { status_code, .. } => assert_eq!(status_code, 500),
        other => panic!("expected Error::Api, got {:?}", other),
    }
}

#[tokio::test]
async fn test_api_error_uses_message_from_body() {
    let (mut server, client) = setup().await;
    let mock = json_mock(
        &mut server,
        "GET",
        "/api/v2/encar/filters",
        500,
        r#"{"message":"Custom error message"}"#,
    );

    let result = client.get_filters("encar").await;

    mock.assert();
    match result.unwrap_err() {
        Error::Api { message, .. } => assert_eq!(message, "Custom error message"),
        other => panic!("expected Error::Api, got {:?}", other),
    }
}

#[tokio::test]
async fn test_api_error_fallback_message() {
    let (mut server, client) = setup().await;
    let mock = json_mock(
        &mut server,
        "GET",
        "/api/v2/encar/filters",
        500,
        r#"{"error":"something"}"#,
    );

    let result = client.get_filters("encar").await;

    mock.assert();
    match result.unwrap_err() {
        Error::Api { message, .. } => assert_eq!(message, "API error: 500"),
        other => panic!("expected Error::Api, got {:?}", other),
    }
}

#[tokio::test]
async fn test_api_error_contains_body() {
    let (mut server, client) = setup().await;
    let response_body = r#"{"message":"Validation failed","errors":["invalid"]}"#;
    let mock = json_mock(
        &mut server,
        "GET",
        "/api/v2/encar/filters",
        422,
        response_body,
    );

    let result = client.get_filters("encar").await;

    mock.assert();
    match result.unwrap_err() {
        Error::Api { body, .. } => assert!(body.contains("Validation failed")),
        other => panic!("expected Error::Api, got {:?}", other),
    }
}

#[tokio::test]
async fn test_auth_error_on_401() {
    let (mut server, client) = setup().await;
    let mock = json_mock(
        &mut server,
        "GET",
        "/api/v2/encar/filters",
        401,
        r#"{"message":"Unauthorized"}"#,
    );

    let result = client.get_filters("encar").await;

    mock.assert();
    match result.unwrap_err() {
        Error::Auth { status_code, message } => {
            assert_eq!(status_code, 401);
            assert_eq!(message, "Unauthorized");
        }
        other => panic!("expected Error::Auth, got {:?}", other),
    }
}

#[tokio::test]
async fn test_auth_error_on_403() {
    let (mut server, client) = setup().await;
    let mock = json_mock(
        &mut server,
        "GET",
        "/api/v2/encar/offers",
        403,
        r#"{"message":"Forbidden"}"#,
    );

    let result = client
        .get_offers("encar", &OffersParams::default())
        .await;

    mock.assert();
    assert!(matches!(result.unwrap_err(), Error::Auth { .. }));
}

#[tokio::test]
async fn test_invalid_json_returns_api_error() {
    let (mut server, client) = setup().await;
    let mock = server
        .mock("GET", "/api/v2/encar/filters")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_body("not json at all")
        .create();

    let result = client.get_filters("encar").await;

    mock.assert();
    match result.unwrap_err() {
        Error::Api { message, .. } => assert!(message.contains("Invalid JSON response")),
        other => panic!("expected Error::Api, got {:?}", other),
    }
}

#[tokio::test]
async fn test_404_returns_api_error_not_auth() {
    let (mut server, client) = setup().await;
    let mock = json_mock(
        &mut server,
        "GET",
        "/api/v2/unknown/filters",
        404,
        r#"{"message":"Source not found"}"#,
    );

    let result = client.get_filters("unknown").await;

    mock.assert();
    match result.unwrap_err() {
        Error::Api { status_code, .. } => assert_eq!(status_code, 404),
        other => panic!("expected Error::Api, got {:?}", other),
    }
}

// ── Error Display ───────────────────────────────────────────────

#[test]
fn test_error_display_auth() {
    let err = Error::Auth {
        status_code: 401,
        message: "Unauthorized".into(),
    };
    assert_eq!(format!("{}", err), "auth error 401: Unauthorized");
}

#[test]
fn test_error_display_api() {
    let err = Error::Api {
        status_code: 500,
        message: "Server error".into(),
        body: "{}".into(),
    };
    assert_eq!(format!("{}", err), "API error 500: Server error");
}
