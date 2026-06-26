use dioxus::prelude::*;

mod hcaptcha;
mod turnstile;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        UsingTurnstile {}
        hr {}
        UsingHCaptcha {}
    }
}

#[component]
fn UsingTurnstile() -> Element {
    let mut turnstile_captcha = use_signal(|| String::new());
    let submit = move |_| async move {
        let v = turnstile::verify(turnstile_captcha.read().cloned())
            .await
            .unwrap();
        dioxus::logger::tracing::info!("verify: {v}");
    };
    rsx! {

        turnstile::TurnstileWidget {
            on_verify: move |token| {
                turnstile_captcha.set(token);
                // dioxus::logger::tracing::info!("token: {token}");
            },
            site_key: "1x00000000000000000000AA".to_string(),
        }

        div {
            button { onclick: submit, "提交" }
        }
    }
}

#[component]
fn UsingHCaptcha() -> Element {
    let mut hcaptcha_captcha = use_signal(|| String::new());
    let submit = move |_| async move {
        let v = hcaptcha::verify(hcaptcha_captcha.read().cloned())
            .await
            .unwrap();
        dioxus::logger::tracing::info!("verify: {v}");
    };
    rsx! {
        hcaptcha::HCaptchaWidget {
            on_verify: move |token| {
                hcaptcha_captcha.set(token);
                // dioxus::logger::tracing::info!("token: {token}");
            },
            site_key: "10000000-ffff-ffff-ffff-000000000001".to_string(),
        }

        div {
            button { onclick: submit, "提交" }
        }
    }
}
