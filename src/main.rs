use dioxus::prelude::*;

static CSS: Asset = asset!("/assets/main.css");

#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}


fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // asset!("/assets/icon.png", ImageAssetOptions::new().with_avif());

    rsx! {    
        document::Stylesheet {href: CSS}
        Title {}
        DogView {}
    }
}


#[component]
fn Title() -> Element {
    rsx! {
        div { id: "title",
            h1 { "HotDog! 🌭" }
        }
    }
}

#[component]
fn DogView() -> Element {
    let mut img_src = use_signal(|| "".to_string());

    let fetch_new = move |_| async move {
        let response = reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap();

        img_src.set(response.message);
    };

    let skip = move |evt| {};
    
    rsx! {
        div { id: "dogview",
            img { src: "{img_src}" }
        }
        div { id: "buttons",
            button { onclick: skip, id: "skip", "skip" }
            button { onclick: fetch_new, id: "save", "save!" }
        }
    }
}
