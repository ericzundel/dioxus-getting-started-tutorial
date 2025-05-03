mod components;
mod backend;

use crate::components::*;
use dioxus::prelude::*;

static CSS: Asset = asset!("/assets/main.css");



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
fn PageNotFound(segments: Vec<String>) -> Element {
    let url = segments.join("/");
   rsx! {
    div { "Unknown page {url}"}
   }
}
