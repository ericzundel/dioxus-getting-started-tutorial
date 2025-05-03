mod components;
mod backend;

use crate::components::*;
use dioxus::prelude::*;

static CSS: Asset = asset!("/assets/main.css");

#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(NavBar)] 
    #[route("/")]
    DogView,

    #[route("/favorites")]
    Favorites,

    // We can collect the segments of the URL into a Vec<String>
    #[route("/:..segments")]
    PageNotFound { segments: Vec<String> },
}


fn main() {
    dioxus::launch(App);
}




fn App() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/main.css") }

        // 📣 delete Title and DogView and replace it with the Router component.
        Router::<Route> {}
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
    let mut img_src = use_resource(|| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap()
            .message
    });

    rsx! {
        div { id: "dogview",
            img { src: img_src.cloned().unwrap_or_default() }
        }
        div { id: "buttons",
            button { onclick: move |_| img_src.restart(),
                 id: "skip", "skip" }
            button { onclick: move |_| async move {
                let current = img_src.cloned().unwrap();
                img_src.restart();
                _ = backend::save_dog(current).await;
            }, id: "save", "save!" }
        }
    }
}

#[component]
fn PageNotFound(segments: Vec<String>) -> Element {
    let url = segments.join("/");
   rsx! {
    div { "Unknown page {url}"}
   }
}
