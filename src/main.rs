#[allow(nonstandard_style)]

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();

    let mut universeID = String::new();
    let mut topic = String::new();
    let mut msg = String::new();
    let mut apiKey = String::new();

    println!("Enter universe ID:");
    std::io::stdin().read_line(&mut universeID).unwrap();
    trim_newline(&mut universeID);

    println!("Enter topic:");
    std::io::stdin().read_line(&mut topic).unwrap();
    trim_newline(&mut topic);

    println!("Enter message:");
    std::io::stdin().read_line(&mut msg).unwrap();
    trim_newline(&mut msg);

    println!("Enter API key:");
    std::io::stdin().read_line(&mut apiKey).unwrap();
    trim_newline(&mut apiKey);

    println!("Sending POST request...");

    let url = "https://apis.roblox.com/messaging-service/v1/universes/".to_owned() + &universeID + "/topics/" + &topic;

    let mut map = std::collections::HashMap::new();
    map.insert("message", msg);

    let res = client
    .post(url)
    .json(&map)
    .header("x-api-key", apiKey)
    .header(reqwest::header::CONTENT_TYPE, "application/json")
    .send()
    .await?;

    println!("{}", res.status());

    std::io::stdin().read_line(&mut String::new()).unwrap();

    Ok(())
}
