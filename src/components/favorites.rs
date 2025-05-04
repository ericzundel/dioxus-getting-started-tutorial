use crate::backend::remove_dog;
use dioxus::logger::tracing::{event, Level};
use dioxus::prelude::*;

#[component]
pub fn Favorites() -> Element {
    /* Signal used to refresh the component.  
       It would be nice if the data in the signal was useful, but right now it's just a trigger.
    */
    let mut update_signal = use_signal(|| 0);

    // Create a pending resource that resolves to the list of dogs from the backend
    // Wait for the favorites list to resolve with `.suspend()`
    let mut favorites = use_resource(move || async move {
        // Read the signal so that the resource will be re-run when the signal is modified
        let _ = update_signal();
        crate::backend::list_dogs().await
    })
    .suspend()?;

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
                            onclick: {
                                move |_| {
                                    let value = url.clone();
                                    async move {
                                        match remove_dog(value.clone()).await {
                                            Err(error) => event!(Level::ERROR, "Couldn't remove {value}: {error}"),
                                            Ok(_) => {
                                                // Trigger a refresh
                                                update_signal += 1;
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
