use leptos::prelude::*;

/// A simple counter island to test code splitting
/// This will compile to a separate WASM file
#[island]
pub fn Counter(initial_value: i32) -> impl IntoView {
    let count = RwSignal::new(initial_value);
    let increment = move |_| *count.write() += 1;
    let decrement = move |_| *count.write() -= 1;
    let reset = move |_| *count.write() = initial_value;

    view! {
        <div class="counter-island">
            <h2>"Interactive Counter Island"</h2>
            <p>"This counter is an island - it loads as a separate WASM file!"</p>
            <div class="counter-controls">
                <button on:click=decrement>"-"</button>
                <span class="counter-value">{count}</span>
                <button on:click=increment>"+"</button>
                <button on:click=reset>"Reset"</button>
            </div>
        </div>
    }
}
