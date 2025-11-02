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
                <HydrationScripts options islands=true/>
                <MetaTags/>
            </head>
            <body>
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

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    use crate::islands::{Counter, SimpleButton};

    #[cfg(feature = "ssr")]
    use crate::server::get_topologies;

    // Test database connectivity
    let topologies = Resource::new(|| (), |_| async move {
        #[cfg(feature = "ssr")]
        {
            get_topologies().await
        }
        #[cfg(not(feature = "ssr"))]
        {
            Ok(vec![])
        }
    });

    view! {
        <h1>"Welcome to Network Topology Visualizer!"</h1>
        <p>"Testing Leptos 0.8 Islands Architecture + Database"</p>

        <h3>"Database Status"</h3>
        <Suspense fallback=move || view! { <p>"Loading topologies..."</p> }>
            {move || {
                topologies.get().map(|result: Result<Vec<crate::models::Topology>, ServerFnError>| match result {
                    Ok(topos) => view! {
                        <div>
                            <p>"✅ Database connected! Found " {topos.len()} " topologies."</p>
                            {if topos.is_empty() {
                                view! { <p>"No topologies yet. Database is ready!"</p> }.into_any()
                            } else {
                                view! {
                                    <ul>
                                        {topos.into_iter().map(|t: crate::models::Topology| view! {
                                            <li>{t.name}</li>
                                        }).collect_view()}
                                    </ul>
                                }.into_any()
                            }}
                        </div>
                    }.into_any(),
                    Err(e) => view! {
                        <p>"❌ Database error: " {e.to_string()}</p>
                    }.into_any(),
                })
            }}
        </Suspense>

        <h3>"Interactive Islands (Loaded as WASM)"</h3>
        <p>"Simple button island:"</p>
        <SimpleButton />

        <p style="margin-top: 20px;">"Counter island with controls:"</p>
        <Counter initial_value=0 />
    }
}
