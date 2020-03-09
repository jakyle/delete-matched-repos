mod response_model;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let repo_url = env::var("REPO_URL").expect("REPO_URL");
    let repo_token = env::var("REPO_TOKEN").expect("REPO_TOKEN");
    let user_agent = env::var("USER_AGENT").expect("USER_AGENT");
    let contains = env::var("CONTAINS").expect("CONTAINS");

    let url = repo_url + "/repos?type=all&sort=created&per_page=100";
    let token =  format!("token {}", &repo_token);

    let client = reqwest::Client::new();
    let body = client
        .get(&url)
        .header(
            "Authorization",
            &token,
        )
        .header("User-Agent", &user_agent)
        .send()
        .await?
        .json::<Vec<response_model::Root>>()
        .await?
        .iter()
        .map(|res| res.full_name.clone())
        .filter(|name| name.as_str().to_ascii_uppercase().contains(&contains))
        .collect::<Vec<String>>();

    for name in body {
        let url = format!("https://api.github.com/repos/{}", name);
        let res = client
            .delete(&url)
            .header(
                "Authorization",
                &token,
            )
            .header("User-Agent", &user_agent)
            .send()
            .await?
            .text()
            .await?;

        println!("deleted repo: {}, message: {}", name, res);
    }

    Ok(())
}
