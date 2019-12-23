use anyhow::Context;
use splits_io_api::{Client, Run, Runner};

macro_rules! test {
    ($($t:tt)*) => {
        let res: Result<(), anyhow::Error> = async {
            {
                $($t)*
            }
            Ok(())
        }
            .await;
        res.unwrap();
    };
}

#[tokio::test]
async fn can_query_run() {
    test! {
        let client = Client::new();
        let run = Run::get(&client, "4cg", false).await?;
        assert_eq!(&*run.game.context("No game")?.name, "Portal");
        assert_eq!(&*run.category.context("No category")?.name, "Inbounds");
        assert_eq!(run.attempts, Some(14));
    }
}

#[tokio::test]
async fn the_example_actually_works() {
    test! {
        // Create a Splits.io API client.
        let client = Client::new();

        // Search for a runner.
        let runner = Runner::search(&client, "cryze")
            .await?
            .into_iter()
            .next()
            .context("There is no runner with that name")?;

        assert_eq!(&*runner.name, "cryze92");

        // Get the PBs for the runner.
        let first_pb = runner.pbs(&client)
            .await?
            .into_iter()
            .next()
            .context("This runner doesn't have any PBs")?;

        // Get the game for the PB.
        let game = first_pb.game.context("There is no game for the PB")?;

        assert_eq!(&*game.name, "The Legend of Zelda: The Wind Waker");

        // Get the categories for the game.
        let categories = game.categories(&client).await?;

        // Get the runs for the Any% category.
        let runs = categories
            .iter()
            .find(|category| &*category.name == "Any%")
            .context("Couldn't find category")?
            .runs(&client)
            .await?;

        assert!(!runs.is_empty());
    }
}
