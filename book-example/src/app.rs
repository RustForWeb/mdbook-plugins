use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    let mut views: Vec<View> = vec![];

    if cfg!(feature = "button") {
        use crate::button::Button;
        views.push(view! {
            <Button />
        });
    }

    views.into_view()
}
