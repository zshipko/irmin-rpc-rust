use irmin::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let arg: String = std::env::args().skip(1).take(1).collect();
    let (client, local) = Client::new(arg).await?;

    local
        .run_until(async move {
            println!("CONNECTED");
            let repo = client.repo().await?;
            println!("REPO");

            let master = repo.master().await?;

            println!("MASTER");

            assert!(master.find("a/b/c").await? == None);

            client.ping().await?;
            println!("PING");
            Ok(())
        })
        .await
}
