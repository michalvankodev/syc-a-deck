use sycamore::prelude::*;

#[component]
pub fn Navigation<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        nav {
            section { "TODO online status" }
            a(href="/settings") { "Settings" }
        }
    }
}
