use dioxus::{document::eval, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TurnstilePayload {
    pub token: String,
}

#[component]
pub fn TurnstileWidget(mut on_verify: EventHandler<String>, site_key: String) -> Element {
    let element_id = "cf-turnstile-container";
    let sitekey_clone = site_key.clone();

    use_effect(move || {
        // 使用 evel 在浏览器中注入 Turnstile 的 JS 代码
        let mut eval_provider = eval(
            format!(
                r#"
        if(!document.getElementById("cf-js")) {{
            const script = document.createElement("script");
            script.id = "cf-js";
            script.src = "https://challenges.cloudflare.com/turnstile/v0/api.js";
            script.async = true;
            script.defer = true;
            document.head.appendChild(script);
        }}

        const initTurnstile = () => {{
          if(window.turnstile) {{
            window.turnstile.render('#{element_id}', {{
                sizekey: "{sitekey_clone}",
                callback: function(token) {{
                                // 验证成功，将 token 发送回 Rust
                                dioxus.send(JSON.stringify({{ token: token }}));
                            }}, // callback
            }}); // render
          }} else {{
           setTimeout(initTurnstile, 100);
           
            }} // window.turnstile
        }}; // initTurnstile

        initTurnstile();
        "#
            )
            .as_str(),
        );

        // 接收token
        spawn(async move {
            if let Ok(json) = eval_provider.recv::<String>().await {
                if let Ok(payload) = serde_json::from_str::<TurnstilePayload>(&json) {
                    on_verify.call(payload.token);
                }
            }
        });
    });

    rsx! {
        div { id: "{element_id}", "data-sitekey": "{site_key}" }
    }
}

#[cfg(feature = "server")]
const SECRET_KEY: &str = "1x0000000000000000000000000000000AA";

#[cfg(feature = "server")]
#[derive(Deserialize)]
pub struct Response {
    pub success: bool,
}

#[server]
pub async fn verify(token: String) -> Result<bool, ServerFnError> {
    let cli = reqwest::ClientBuilder::new()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .unwrap();
    let res = cli
        .post("https://challenges.cloudflare.com/turnstile/v0/siteverify")
        .form(&[("secret", SECRET_KEY), ("response", &token)])
        .send()
        .await
        .unwrap()
        .json::<Response>()
        .await
        .unwrap();
    Ok(res.success)
}
