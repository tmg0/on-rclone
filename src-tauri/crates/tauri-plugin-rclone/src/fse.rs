use crate::Result;

pub async fn ensure_dir(dir: &str) -> Result<()> {
    tokio::fs::create_dir_all(dir).await?;
    Ok(())
}
