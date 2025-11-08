use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body class="m-0 p-0 overflow-hidden">
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/ntv.css"/>

        // favicon
        <leptos_meta::Link rel="icon" type_="image/svg+xml" href="/ntb_logo.svg"/>

        // sets the document title
        <Title text="Network Topology Builder"/>

        // content for this welcome page
        <Router>
            <Routes fallback=|| "Page not found.".into_view()>
                <Route path=StaticSegment("") view=HomePage/>
            </Routes>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    use crate::islands::TopologyEditor;

    // Create a signal for the current topology ID (default to 1)
    let current_topology_id = RwSignal::new(1i64);

    // Use TopologyEditor with full-screen layout
    view! {
        <TopologyEditor current_topology_id=current_topology_id />
    }
}
