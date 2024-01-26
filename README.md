
# Rust Manga Scrapper

## Description
This program allows you to scrap manga chapters from [leercapitulo.com](https://www.leercapitulo.com/) (spanish only :D) in a simple and automated way. It uses Selenium to control a Chrome browser and Reqwest to download chapter images.

## Requirements
- [Chromedriver](https://sites.google.com/chromium.org/driver/) installed and in the PATH.
- [Google Chrome](https://www.google.com/chrome/) installed and compatible with previous Chromedriver.
- [Rust](https://www.rust-lang.org/tools/install) installed (to compile).

## Installation
1. Clone this repository:
   ```
   git clone https://github.com/Hakaary/rust_manga_scrapper.git
   ```
   
2. Build the binary:
   ```
   cd rust_manga_scrapper
   cargo build --release
   ```

3. Location:

   The binary will be located here:
   
   ```
   target/release/manga_downloader
   ```

## Usage

### Arguments

`--manga`: The name of the manga you want to download.

`--from`: The number of the first chapter to download.

`--to`: The number of the last chapter to download.

`--number`: Download a specific chapter.


Run the program from the command line, providing the necessary arguments. For example, assuming the binary is this path:

```
./target/release/manga_downloader --manga "Fullmetal Alchemist" --from 1 --to 5
```

![Screenshot](read_imgs/example_1.png)

```
./target/release/manga_downloader -m death\ note -n 1
```

![Screenshot](read_imgs/example_2.png)


