
use crate::helpers::spawn_app;

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app = spawn_app().await;

    let body = "name=joe&email=joe%40bts.org";
    let response = app.post_subscriptions(body.into()).await;

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("select email, name from subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscriptions.");

    assert_eq!(saved.email, "joe@bts.org");
    assert_eq!(saved.name, "joe");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app = spawn_app().await;

    let test_case = vec![
        ("name=joe", "missing the email"),
        ("email=joe%40bts.org", "missing the name"),
        ("", "missing both name and email")
    ];

    for (invalid_body, error_message) in test_case {
        let response = app.post_subscriptions(invalid_body.into()).await;

        assert_eq!(
            400, 
            response.status().as_u16(),
            "The api not fail with 400 bad request when the payload was {}.",
            error_message
        )
    }
}

#[tokio::test]
async fn subscribe_returns_a_400_when_fields_are_present_but_invalid() {
    let app = spawn_app().await;

    let test_case = vec![
        ("name=&email=joe%40bts.org", "empty name"),
        ("name=joe&email=", "empty email"),
        ("name=joe&email=joebts.org", "invalid email")
    ];

    for (body, description) in test_case {
        let response = app.post_subscriptions(body.into()).await;

        assert_eq!(
            400, 
            response.status().as_u16(),
            "The api not fail with 400 bad request when the payload was {}.",
            description
        )
    }
}