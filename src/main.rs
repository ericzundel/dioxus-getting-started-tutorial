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
    #[route("/")]
    DogView,
    // We can collect the segments of the URL into a Vec<String>
    #[route("/:..segments")]
    PageNotFound { segments: Vec<String> },
}

// The database is only available to server code
#[cfg(feature = "server")]
thread_local! {
    pub static DB: rusqlite::Connection = {
        // Open the database from the persisted "hotdog.db" file
        let conn = rusqlite::Connection::open("hotdog.db").expect("Failed to open database");

        // Create the "dogs" table if it doesn't already exist
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS dogs (
                id INTEGER PRIMARY KEY,
                url TEXT NOT NULL
            );",
        ).unwrap();

        // Return the connection
        conn
    };
}


fn main() {
    dioxus::launch(App);
}

// Expose a `save_dog` endpoint on our server that takes an "image" parameter
#[server]
async fn save_dog(image: String) -> Result<(), ServerFnError> {
    DB.with(|f| f.execute("INSERT INTO dogs (url) VALUES (?1)", &[&image]))?;
    Ok(())
}

/* 
#[component]
fn App() -> Element {
    // asset!("/assets/icon.png", ImageAssetOptions::new().with_avif());

    rsx! {    
        document::Stylesheet {href: CSS}
        Title {}
        DogView {}
    }
}
*/

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
                _ = save_dog(current).await;
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
