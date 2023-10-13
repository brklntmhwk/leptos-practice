use leptos::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Svg {
    AddSquare,
    HamburgerMenu,
    Logo,
    MagnifyingGlass,
    RubbishBin,
}

impl IntoView for Svg {
    fn into_view(self) -> View {
        view! {
            <div inner_html=match self {
                Svg::AddSquare => include_str!("../../../style/inline-svg/add-square.svg"),
                Svg::HamburgerMenu => include_str!("../../../style/inline-svg/hamburger-menu.svg"),
                Svg::Logo => include_str!("../../../style/inline-svg/postgres_elephant.svg"),
                Svg::MagnifyingGlass => include_str!("../../../style/inline-svg/magnifying-glass.svg"),
                Svg::RubbishBin => include_str!("../../../style/inline-svg/rubbish-bin.svg"),
            }></div>
        }
        .into_view()
    }
}
