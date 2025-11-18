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
        <Stylesheet id="leptos" href="/pkg/ntb.css"/>

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
    use crate::api::{get_last_topology_id, clear_traffic_data, get_topologies};
    use leptos::task::spawn_local;

    // Create a signal for the current topology ID (will be loaded from database)
    let current_topology_id = RwSignal::new(1i64);

    // Load the last viewed topology and clear traffic on startup
    Effect::new(move || {
        spawn_local(async move {
            // Get the last viewed topology ID
            if let Ok(Some(last_id)) = get_last_topology_id().await {
                current_topology_id.set(last_id);

                // Clear traffic data for this topology on startup
                let _ = clear_traffic_data(last_id).await;
            } else {
                // If no last topology, try to get the first available topology
                if let Ok(topologies) = get_topologies().await {
                    if let Some(first_topology) = topologies.first() {
                        current_topology_id.set(first_topology.id);
                        let _ = clear_traffic_data(first_topology.id).await;
                    }
                }
            }
        });
    });

    // Use TopologyEditor with full-screen layout
    view! {
        <TopologyEditor current_topology_id=current_topology_id />
    }
}
