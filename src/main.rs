use reqwest;
use thirtyfour::prelude::{By, DesiredCapabilities, WebDriver, WebDriverResult};
use tokio::{fs, io::AsyncWriteExt, task};

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let manga_name = "One Piece";
    let chapters = 340..=350;

    match fs::metadata(manga_name).await {
        Err(_) => {
            fs::DirBuilder::new()
                .create(format!("./{}", manga_name))
                .await?
        }
        _ => {}
    }

    let mut caps = DesiredCapabilities::chrome();
    caps.add_chrome_arg("--enable-automation")?;
    caps.set_headless()?;

    if let Ok(driver) = WebDriver::new("http://localhost:9515", caps.clone()).await {
        // Open needed tabs
        let mut tabs_handles = vec![];
        for _ in chapters.clone().enumerate() {
            let tab_handle = driver.new_tab();
            tabs_handles.push(tab_handle);
        }
        for tab_handle in tabs_handles {
            tab_handle.await?;
        }

        // Load a chapter on every tab
        let tabs_handles = driver.windows().await?;
        for (i, chapter) in chapters.clone().enumerate() {
            driver.switch_to_window(tabs_handles[i].clone()).await?;
            driver
                .goto(format!(
                    "https://www.leercapitulo.com/leer/5pljav/one-piece/{}/#1",
                    chapter
                ))
                .await?;
        }

        // Iterate chapters
        let tabs_handles = driver.windows().await?;
        let mut f_write_imgs = Vec::new();
        for (tab_handle, chapter) in tabs_handles.iter().zip(chapters) {
            driver.switch_to_window(tab_handle.clone()).await?;
            let current_url = driver.current_url().await?;
            let mut current_page: u8 = 1;

            match fs::metadata(format!("./{}/{}", manga_name, chapter)).await {
                Err(_) => {
                    fs::DirBuilder::new()
                        .create(format!("./{}/{}", manga_name, chapter))
                        .await?
                }
                _ => return Ok(()),
            }

            if current_url.path() != "blank" {
                loop {
                    // Download page
                    if let Some(img_url) = driver
                        .find(By::ClassName("lazy"))
                        .await?
                        .attr("src")
                        .await?
                    {                    
                        let mut img_file = fs::File::create(format!("./{}/{}/{}.png", manga_name, chapter, current_page)).await?;
                        let task = task::spawn(async move {
                            let img = reqwest::get(img_url);
                            let b_img = img.await.unwrap().bytes().await.unwrap();
                            img_file.write_all(&b_img).await.unwrap();
                        });
                        f_write_imgs.push(task);
                    }

                    // Click to go to next page
                    driver
                        .execute_async(
                            r#"
                            let done = arguments[0];
                            $("selector").ready(() => {
                                done(document.getElementsByClassName("img_land_next")[0].click());
                            });
                            "#,
                            Vec::new(),
                        )
                        .await?;

                    // Check if last page
                    if let Some(url_current_page) = driver.current_url().await?.fragment() {
                        if let Ok(url_current_page) = url_current_page.parse::<u8>() {
                            if current_page == url_current_page {
                                break;
                            } else {
                                current_page = url_current_page;
                            }
                        }
                    }
                }
            }
        }
        for f_write_img in f_write_imgs {
            f_write_img.await.unwrap();
        }
        driver.quit().await?;
    } else {
        println!("No driver (chromedriver) found. Make sure it is installed and running.");
    }

    Ok(())
}
