use sycamore::prelude::*;

#[component]
pub fn Settings<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        section {
            form {
                label {
                    "Server URI"
                    input(type="text", placeholder="wss://")
                }
                label {
                    "Password"
                    input(type="password")
                }
                label {
                    "Items per page"
                    input(type="number", value="16")
                }
            }
        }
    }
}
