use reqwest;

#[tokio::main]
async fn main() {
    let response = reqwest::get("http://flaskapiproj1-env.eba-sx2q5cpp.us-east-2.elasticbeanstalk.com/nqueens/4")
        .await
        // each response is wrapped in a `Result` type
        // we'll unwrap here for simplicity
        .unwrap()
        .text()
        .await;
    println!("{:?}", response);
}
