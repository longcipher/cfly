mod utils;

use worker::*;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}]",
        Date::now().to_string(),
        req.path(),
    );
}

#[event(fetch)]
pub async fn fetch(
    req: Request,
    env: Env,
    _ctx: Context,
) -> Result<Response> {
    log_request(&req);

    // Set panic hook
    utils::set_panic_hook();

    // Use worker Router to handle route matching
    let router = Router::new();

    // Add route handlers
    let result = router
        .get_async("/", |_req, ctx| async move {
            let headers = Headers::new();
            let home_redirect = ctx.var("HOME")?;
            headers.append("Location", &home_redirect.to_string())?;
            Response::empty().map(|r| r.with_headers(headers).with_status(301))
        })
        .get_async("/:url", |req, ctx| async move {
            console_log!("Looking for url");

            if let Some(name) = ctx.param("url") {
                console_log!("Looking for url: {}", name);

                // First, try to find in KV storage
                if let Some(location) = ctx.kv("cfly")?.get(name).text().await? {
                    console_log!("Found KV for url: {}", name);
                    console_log!("Redirecting to: {}", location);

                    let headers = Headers::new();
                    headers.append("Location", &location)?;

                    console_log!("Responding...");
                    Response::empty().map(|r| r.with_headers(headers).with_status(301))
                } else {
                    console_log!("No url found in KV for: {}, trying Git commit logic", name);
                    
                    // Fallback: Use Git commit logic
                    let git_repo = ctx.var("GIT_REPO")
                        .map(|s| s.to_string())
                        .unwrap_or_else(|_| "https://github.com/longcipher/links".to_string());
                    let pathname = req.path(); // Get the full path from request
                    
                    // Construct Git patch URL
                    let git_patch_url = format!("{}/commit{}.patch", git_repo, pathname);
                    console_log!("Fetching Git patch from: {}", git_patch_url);
                    
                    // Fetch the patch directly using Fetch::Url
                    match Fetch::Url(git_patch_url.parse()?).send().await {
                        Ok(mut patch_response) => {
                            if patch_response.status_code() == 200 {
                                let patch_text = patch_response.text().await?;
                                console_log!("Got patch content, extracting subject");
                                
                                // Extract subject from patch using regex-like logic
                                let redirect_url = if pathname == "/" {
                                    git_repo.to_string()
                                } else {
                                    // Look for "Subject: [PATCH]" line in patch
                                    let mut subject_url = None;
                                    for line in patch_text.lines() {
                                        if line.starts_with("Subject:") && line.contains("[PATCH]") {
                                            // Extract everything after "[PATCH]" and trim
                                            if let Some(patch_pos) = line.find("[PATCH]") {
                                                let after_patch = &line[patch_pos + 7..];
                                                subject_url = Some(after_patch.trim().to_string());
                                                break;
                                            }
                                        }
                                    }
                                    subject_url.unwrap_or_else(|| git_repo.to_string())
                                };
                                
                                console_log!("Redirecting to Git URL: {}", redirect_url);
                                
                                let headers = Headers::new();
                                headers.append("Location", &redirect_url)?;
                                Response::empty().map(|r| r.with_headers(headers).with_status(301))
                            } else {
                                console_log!("Git patch fetch failed with status: {}", patch_response.status_code());
                                Response::error("Requested url not found", 404)
                            }
                        }
                        Err(e) => {
                            console_log!("Error fetching Git patch: {:?}", e);
                            Response::error("Requested url not found", 404)
                        }
                    }
                }
            } else {
                Response::error("Bad Request", 400)
            }
        })
        .run(req, env)
        .await;

    console_log!("Responding: {:?}", result);

    result
}
