use crate::backend::remove_dog;
use dioxus::logger::tracing::{event, Level};
use dioxus::prelude::*;

pub async fn list_dogs_component(count: &Signal<i32>) -> Result<Vec<(usize, String)>, ServerFnError> {
    let _= *count + 1;
    crate::backend::list_dogs().await
}

#[component]
pub fn Favorites() -> Element {
    // Create a pending resource that resolves to the list of dogs from the backend
    // Wait for the favorites list to resolve with `.suspend()`
    let mut favorites = use_resource(crate::backend::list_dogs).suspend()?;

    rsx! {
        div { id: "favorites",
            div { id: "favorites-container",
                for (id, url) in favorites().unwrap_or_default() {
                    // Render a div for each photo using the dog's ID as the list key
                    div {
                        div {
                            key: id,
                            class: "favorite-dog",
                            img { src: "{url}" }
                        }
                        button {
                            // TODO: Add onclick handler
                            onclick: {
                                move |_| {
                                let value = url.clone();
                                async move {
                                    match remove_dog(value.clone()).await {
                                        Err(error) => event!(Level::ERROR, "Couldn't remove {value}: {error}"),
                                        Ok(_) => {
                                            /* How do I  remove from favorites ??? 
                                            This doesn't work: favorites().unwrap().retain( |x| x.1 != value)
                                            Somehow, we need to introduce a signal into the use_resource() above and
                                            then update the signal so the use_resource block will re-run
                                            */
                                            event!(Level::INFO, "Figure out how to update favorites");
                                        },
                                    }
                                }
                            }
                            },
                            id: "button-{id}",
                            "Remove"
                        }
                    }
                }
            }
        }
    }
}
