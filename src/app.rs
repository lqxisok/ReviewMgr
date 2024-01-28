use leptos::*;
use serde::{Deserialize, Serialize};

use leptos_router::Router;

use super::texcolrouters::RootRouter;

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>

            <main class="container">
                <RootRouter/>
            </main>
        </Router>
    }
}
