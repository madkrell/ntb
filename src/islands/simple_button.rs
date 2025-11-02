use leptos::prelude::*;

/// A simple button island with client-side interactivity
#[island]
pub fn SimpleButton() -> impl IntoView {
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <div>
            <button on:click=on_click>"Click Me: " {count}</button>
        </div>
    }
}
