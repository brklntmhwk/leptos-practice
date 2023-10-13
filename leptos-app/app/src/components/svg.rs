use leptos::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Svg {
    AddFile,
    HamburgerMenu,
    Logo,
    MagnifyingGlass,
}

impl IntoView for Svg {
    fn into_view(self) -> View {
        view! {
            <div inner_html=match self {
                Svg::AddFile => include_str!("../../../style/inline-svg/add-file.svg"),
                Svg::HamburgerMenu => include_str!("../../../style/inline-svg/hamburger-menu.svg"),
                Svg::Logo => include_str!("../../../style/inline-svg/postgres_elephant.svg"),
                Svg::MagnifyingGlass => include_str!("../../../style/inline-svg/magnifying-glass.svg"),
            }></div>
        }
        .into_view()
    }
}
