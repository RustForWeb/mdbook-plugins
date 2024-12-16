use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let mut views: Vec<AnyView> = vec![];

    if cfg!(feature = "button") {
        use crate::button::Button;
        views.push(
            view! {
                <Button />
            }
            .into_any(),
        );
    }

    views.into_view()
}
