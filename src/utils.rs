use std::{io, path::Path};

pub async fn write_file(path: impl AsRef<Path>, contents: impl AsRef<[u8]>) -> io::Result<()> {
    let dir = path.as_ref().parent();
    if let Some(dir) = dir {
        tokio::fs::create_dir_all(dir).await?;
    }
    tokio::fs::write(path, contents).await
}

pub async fn read_file(path: impl AsRef<Path>) -> io::Result<Vec<u8>> {
    tokio::fs::read(path).await
}
