use leptos::*;

#[component]
pub fn Button() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <button on:click=move |_| set_count.update(|count| *count += 1 )>
            "Count: " {count}
        </button>
    }
}
