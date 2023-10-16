use leptos::*;
use leptos_dom::*;
use leptos_router::*;

#[component]
pub fn TableCell(
    children: Children,
    #[prop(optional, into)] class: String,
    #[prop(optional, into, default = 1)] colspan: i32,
) -> impl IntoView {
    view! {
        <td class=format!("p-3 md:p-4 {class}") colspan=colspan>
            {children()}
        </td>
    }
}

#[component]
pub fn TableRow(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    view! {
        <tr class=format!(
            "my-4 bg-white dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600 {class}"
        )>{children()}</tr>
    }
}

#[derive(Clone, Debug)]
pub struct ColumnHeader {
    pub id: String,
    pub label: String,
    pub width: Option<u32>,
    pub center: bool,
}

impl IntoView for ColumnHeader {
    fn into_view(self) -> View {
        match self.width {
            Some(width) => view! {
                               <th id=self.id scope="col" class=format!("p-3 md:p-4 w-{}", width) class:text-center=move || self.center>
                                   {self.label}
                               </th>
                           },
            None => view! {
                        <th id=self.id scope="col" class="p-3 md:p-4" class:text-center=move || self.center>
                            {self.label}
                        </th>
                    },
        }.into_view()
    }
}

#[component]
pub fn Table(children: Children, column_headers: Vec<ColumnHeader>) -> impl IntoView {
    view! {
        <table class="w-full text-left text-gray-500 table-fixed dark:text-gray-400">
            <thead class="hidden text-gray-700 uppercase bg-gray-50 md:table-header-group dark:text-gray-400 dark:bg-gray-700">
                <tr>{column_headers}</tr>
            </thead>
            <tbody>{children()}</tbody>
        </table>
    }
}
