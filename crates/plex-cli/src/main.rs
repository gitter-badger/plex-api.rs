mod flags;
mod preferences;
mod wait;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    flags::PlexCli::from_env()?.run().await
}
