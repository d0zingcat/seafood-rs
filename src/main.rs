fn main() {
    println!("Hello, world!");
    test();
}
async fn test() -> Result<(), reqwest::Error> {
    println!("Hello, world!");
    let res = reqwest::get("https://www.baidu.com").await?;
    println!("{:?}", res.status());
    let body = res.text().await?;
    println!("Body: \n\n{}", body);
    Ok(())
}
