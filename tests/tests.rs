use splits_io_api::{run, Client};

#[tokio::test]
async fn can_query_run() {
    let client = Client::new();
    let run = run::get(&client, "4cg", false).await.unwrap();
    assert_eq!(&*run.game.unwrap().name, "Portal");
    assert_eq!(&*run.category.unwrap().name, "Inbounds");
    assert_eq!(run.attempts, Some(14));
}
