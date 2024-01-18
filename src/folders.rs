use tokio::fs;

pub async fn create_main_folder(manga_name: &str) -> Result<(), ()> {
    match fs::metadata(manga_name).await {
        Err(_) => {
            fs::DirBuilder::new()
                .create(format!("./{}", manga_name))
                .await
                .unwrap();
        }
        _ => {}
    }
    Ok(())
}

pub async fn create_chapter_folder(manga_name: &str, chapter: u32) -> Result<(), ()> {
    match fs::metadata(format!("./{}/{}", manga_name, chapter)).await {
        Err(_) => {
            fs::DirBuilder::new()
                .create(format!("./{}/{}", manga_name, chapter))
                .await
                .unwrap()
        }
        _ => return Err(()),
    }
    Ok(())
}