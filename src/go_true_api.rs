pub struct GoTrueApi {
    url: String,
}

impl GoTrueApi {
    pub fn new(url: String) -> GoTrueApi {
        GoTrueApi { url }
    }

    pub async fn sign_up_with_email(
        self,
        email: String,
        password: String,
        redirect_to: Option<String>,
        captcha_token: Option<String>,
    ) -> Result<String, reqwest::Error> {
        // let query_string: String = redirect_to.unwrap_or(String::from(""));

        let client = reqwest::Client::new();
        let res = client
            .post(self.url)
            .body("the exact body that is sent")
            .send()
            .await?;

        return Ok(String::from("ASD"));
    }
}
