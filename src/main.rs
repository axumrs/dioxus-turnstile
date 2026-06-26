use dioxus::prelude::*;

mod turnstile;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut captch = use_signal(|| String::new());
    let submit = move |_| async move {
        let v = turnstile::verify(captch.read().cloned()).await.unwrap();
        dioxus::logger::tracing::info!("verify: {v}");
    };
    rsx! {

        turnstile::TurnstileWidget {
            on_verify: move |token| {
                captch.set(token);
                // dioxus::logger::tracing::info!("token: {token}");
            },
            site_key: "1x00000000000000000000AA".to_string(),
        }

        div {
            button { onclick: submit, "提交" }
        }
    }
}
