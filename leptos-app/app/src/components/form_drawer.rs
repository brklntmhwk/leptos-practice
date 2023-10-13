use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    thread, time,
};

use base64::Engine;
use convert_case::{Case, Casing};
use entity::{chrono, uuid};
use leptos::{ev::MouseEvent, *};
use leptos_dom::*;
use leptos_router::*;
use rand::Rng;

use super::*;

/// ### FormFieldInputType
///
/// Enumeration representing types of form field inputs.
///
/// * `Text`
/// * `TextArea`
/// * `Date`
/// * `Hidden`
///
#[derive(Clone, Copy, Hash)]
pub enum FormFieldInputType {
    Text,
    TextArea,
    Date,
    Hidden,
    Password,
}

/// ### FormField
///
/// Structure representing an individual form field.
///
/// * `id: String` - Unique identifier for the form field.
/// * `label: Option<String>` - Optional label for the form field. If not provided, no
///   label is shown.
/// * `input_type: FormFieldInputType` - Type of input for the form field, defined
///   using `FormFieldInputType` enum.
/// * `placeholder: String` - Placeholder text for the form field.
/// * `value: String` - The pre-populated value for the form field.
/// * `required: bool` - Determines if the form field is required to submit the form
///   or not.
#[derive(Clone, Hash)]
pub struct FormField {
    pub id: String,
    pub label: Option<String>,
    pub input_type: FormFieldInputType,
    pub placeholder: String,
    pub value: String,
    pub required: bool,
}

impl IntoView for FormField {
    fn into_view(self) -> View {
        let label = match self.label {
            Some(label) => view! {
                <label
                    for=self.id.clone()
                    class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                >
                    {label}
                </label>
            }
            .into_view(),
            None => view! { <></> }.into_view(),
        };

        match self.input_type {
            FormFieldInputType::Text => {
                view! {
                    <div class="mb-6">
                        {label}
                        <input
                            id=self.id.clone()
                            value=self.value.clone()
                            type="text"
                            name=self.id.clone()
                            class="block p-2.5 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300 dark:placeholder-gray-400 dark:text-white dark:bg-gray-700 dark:border-gray-600 focus:border-blue-500 focus:ring-blue-500 dark:focus:ring-blue-500 dark:focus:border-blue-500"
                            placeholder=self.placeholder
                            required=self.required
                        />
                    </div>
                }.into_view()
            }
            FormFieldInputType::TextArea => {
                view! {
                    <div class="mb-6">
                        {label}
                        <textarea
                            id=self.id.clone()
                            name=self.id.clone()
                            rows="4"
                            class="block p-2.5 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300 dark:placeholder-gray-400 dark:text-white dark:bg-gray-700 dark:border-gray-600 focus:border-blue-500 focus:ring-blue-500 dark:focus:ring-blue-500 dark:focus:border-blue-500"
                            placeholder=self.placeholder
                        >
                            {self.value.clone()}
                        </textarea>
                    </div>
                }.into_view()
            }
            FormFieldInputType::Date => {
                view! {
                    <div class="relative mb-6">
                        {label}
                        <input
                            type="date"
                            value=""
                            id=self.id.clone()
                            name=self.id.clone()
                            value=self.value.clone()
                            class="block p-2.5 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300 dark:placeholder-gray-400 dark:text-white dark:bg-gray-700 dark:border-gray-600 focus:border-blue-500 focus:ring-blue-500 datepicker-input dark:focus:ring-blue-500 dark:focus:border-blue-500"
                            min="2000-01-01"
                            max="2999-99-99"
                        />
                    </div>
                }.into_view()
            }
            FormFieldInputType::Hidden => {
                view! {
                    <input
                        type="hidden"
                        id=self.id.clone()
                        name=self.id.clone()
                        value=self.value.clone()
                    />
                }.into_view()
            }
            FormFieldInputType::Password => {
                view! {
                    <div class="relative mb-6">
                        {label}
                        <input
                            type="password"
                            id=self.id.clone()
                            name=self.id.clone()
                            required=self.required
                        />
                    </div>
                }.into_view()
            }
        }
    }
}

/// # FormDrawerButton Component
///
/// `FormDrawerButton` is a Leptos component for creating a collapsible form drawer
/// that appears when clicking a button. The form drawer is customizable with the
/// provided fields, a form action, icon, and title.
///
#[component]
pub fn FormDrawerButton<S, O>(
    icon: Svg,
    title: String,
    action: MultiAction<S, Result<O, ServerFnError>>,
    fields: Vec<FormField>,
    #[prop(optional, into)] class: String,
) -> impl IntoView
where
    S: Clone + ServerFn + leptos::Serializable,
    O: Clone + Serializable + 'static,
{
    let (drawer_open, set_drawer_open) = create_signal(false);
    let sr_title = format!("Open {} drawer", title);

    let drawer = move || {
        let title = title.clone();
        let fields = fields.clone();
        match drawer_open.get() {
            true => {
                view! {
                    <div
                        id="drawer"
                        class="overflow-y-auto fixed top-0 left-0 z-40 p-4 w-80 h-screen bg-white animate-toggle"
                        tabindex="-1"
                        aria-labelledby="drawer-label"
                    >
                        <h5
                            id="drawer-label"
                            class="inline-flex items-center mb-6 text-base font-semibold text-gray-500 uppercase dark:text-gray-400"
                        >
                            <div class="mr-2 w-5 h-5">{icon}</div>
                            {title.clone()}
                        </h5>
                        <button
                            type="button"
                            on:click=move |_| set_drawer_open.set(false)
                            data-drawer-hide="drawer-form"
                            aria-controls="drawer-form"
                            class="inline-flex absolute top-2.5 right-2.5 items-center p-1.5 text-sm text-gray-400 bg-transparent rounded-lg hover:text-gray-900 hover:bg-gray-200 dark:hover:bg-gray-600 dark:hover:text-white"
                        >
                            <svg
                                aria-hidden="true"
                                class="w-5 h-5"
                                fill="currentColor"
                                viewBox="0 0 20 20"
                                xmlns="http://www.w3.org/2000/svg"
                            >
                                <path
                                    fill-rule="evenodd"
                                    d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
                                    clip-rule="evenodd"
                                ></path>
                            </svg>
                            <span class="sr-only">"Close menu"</span>
                        </button>
                        <div class="mb-6">
                            <MultiActionForm action=action class="mb-6">
                                {fields}
                                <button
                                    type="reset"
                                    onclick="form.requestSubmit()"
                                    on:click=move |_| set_drawer_open.set(false)
                                    class="flex justify-center items-center py-2.5 px-5 mr-2 mb-2 w-full text-sm font-medium text-white bg-blue-700 rounded-lg hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 focus:outline-none"
                                >
                                    <div class="mr-2 w-5 h-5">{icon}</div>
                                    {title.clone()}
                                </button>
                            </MultiActionForm>
                        </div>
                    </div>
                }.into_view()
            }
            false => {
                view! { <></> }.into_view()
            }
        }
    };

    view! {
        <div>
            <div
                on:click=move |_| set_drawer_open.set(false)
                class="fixed top-0 left-0 z-30 w-screen h-screen transition-all pointer-events-none"
                class:pointer-events-none=move || !drawer_open.get()
                class:backdrop-blur-sm=drawer_open
            ></div>
            <Button class=class on:click=move |_| set_drawer_open.set(true)>
                <div class="w-6 h-6">{icon}</div>
                <span class="sr-only">{sr_title}</span>
            </Button>
            {drawer}
        </div>
    }
}
