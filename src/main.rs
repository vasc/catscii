use serde::Deserialize;

#[tokio::main]
async fn main() {
    let res = reqwest::get("https://api.thecatapi.com/v1/images/search")
        .await
        .unwrap();
    if !res.status().is_success() {
        panic!("Request failed with HTTP {}", res.status());
    }

    #[derive(Deserialize)]
    struct CatImage {
        // this is the only field we really need, but let's show how we would
        // deserialize the whole thing anyway.
        url: String,
    }

    let images: Vec<CatImage> = res.json().await.unwrap();
    let image = images
        .first()
        .expect("the cat API should return at least one image");
    println!("The image is at {}", image.url);
}
