use dioxus::{document::eval, prelude::*};

#[component]
pub fn HCaptchaWidget(mut on_verify: EventHandler<String>, site_key: String) -> Element {
    let element_id = "hcaptcha-container";
    let sitekey_clone = site_key.clone();

    use_effect(move || {
        // 使用 evel 在浏览器中注入 Turnstile 的 JS 代码
        let mut eval_provider = eval(
            format!(
                r#"
        if(!document.getElementById("hcaptcha-js")) {{
            const script = document.createElement("script");
            script.id = "hcaptcha-js";
            script.src = "https://js.hcaptcha.com/1/api.js";
            script.async = true;
            script.defer = true;
            document.head.appendChild(script);
        }}

        const initHcaptcha = () => {{
          if(window.hcaptcha) {{
            window.hcaptcha.render('{element_id}', {{
                sizekey: "{sitekey_clone}",
                callback: function(token) {{
                                // 验证成功，将 token 发送回 Rust
                                // dioxus.send(JSON.stringify({{ token: token }}));
                                dioxus.send(token);
                            }}, // callback
            }}); // render
          }} else {{
           setTimeout(initHcaptcha, 100);
           
            }} // window.hcaptcha
        }}; // initHcaptcha

        initHcaptcha();
        "#
            )
            .as_str(),
        );

        // 接收token
        spawn(async move {
            if let Ok(token) = eval_provider.recv::<String>().await {
                on_verify.call(token);
            }
        });
    });

    rsx! {
        div { id: "{element_id}", "data-sitekey": "{site_key}" }
    }
}

#[cfg(feature = "server")]
const SECRET_KEY: &str = "0x0000000000000000000000000000000000000000";

#[cfg(feature = "server")]
#[derive(serde::Deserialize)]
pub struct Response {
    pub success: bool,
}

#[server]
pub async fn verify(token: String) -> Result<bool, ServerFnError> {
    dioxus::logger::tracing::info!(
        "verify: {:?}",
        &[("secret", SECRET_KEY), ("response", &token)]
    );
    let cli = reqwest::ClientBuilder::new()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .unwrap();
    let res = cli
        .post("https://api.hcaptcha.com/siteverify")
        .form(&[("secret", SECRET_KEY), ("response", &token)])
        .send()
        .await
        .unwrap()
        .json::<Response>()
        .await
        .unwrap();
    Ok(res.success)
}
