use components::{navigation::Navigation, settings::Settings};
use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router};

pub mod components;

#[derive(Route)]
enum AppRoutes {
    #[to("/")]
    Index,
    #[to("/settings")]
    Settings,
    #[not_found]
    NotFound,
}

fn main() {
    sycamore::render(|cx| {
        view! { cx,
            main(class="app") {
                Navigation {}
                Router {
                    integration: HistoryIntegration::new(),
                    view: |cx, route: &ReadSignal<AppRoutes>| {
                        match route.get().as_ref() {
                            AppRoutes::Index => view! { cx,
                                "This is the index page"
                            },
                            AppRoutes::Settings => view! { cx,
                                Settings {}
                            },
                            AppRoutes::NotFound => view! { cx,
                                "404 Not Found"
                            },
                        }
                    }
                }
            }
        }
    });
}
