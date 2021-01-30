use irmin::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let arg: String = std::env::args().skip(1).take(1).collect();
    let (client, local) = Client::new(arg).await?;

    local
        .run_until(async move {
            println!("Connected");

            println!("Sending ping");
            client.ping().await?;
            println!("Ping ok");

            println!("Getting repo handle");
            let repo = client.repo().await?;

            println!("Getting store");
            let master = repo.master().await?;

            println!("Removing a/b/c");
            master
                .remove("a/b/c", &Info::new("author", "message")?)
                .await?;

            println!("Creating new tree");
            let tree = repo.empty_tree();
            let tree = tree.add("a/b/c", "123");

            assert!(master.find("a/b/c").await? == None);

            println!("Setting tree");
            master
                .set_tree("/", &tree, &Info::new("author", "message")?)
                .await?;

            assert!(master.find("a/b/c").await?.unwrap().fetch(&repo).await? == b"123".to_vec());

            Ok(())
        })
        .await
}
