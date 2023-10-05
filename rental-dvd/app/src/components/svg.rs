use leptos::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Svg {
    Logo,
    MagnifyingGlass,
}

impl IntoView for Svg {
    fn into_view(self) -> View {
        view! {
            <div inner_html=match self {
                Svg::Logo => include_str!("../../../style/postgres_elephant.svg"),
                Svg::MagnifyingGlass => include_str!("../../../style/magnifying-glass.svg"),
            }></div>
        }
        .into_view()
    }
}
