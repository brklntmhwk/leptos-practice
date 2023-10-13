use std::{cell::RefCell, rc::Rc, sync::Arc};

use entity::{
    chrono,
    todos::{self, Model},
    uuid,
};
use js_sys::Function;
use leptos::{ev::MouseEvent, *};
use leptos_dom::*;
use leptos_router::*;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Document, DomRect, Element, Event, EventTarget, Window};

pub fn set_href(href: String) {
    let window = window();
    let location = window.location();

    location.set_href(&href).unwrap();
}

pub fn get_element_dom_rect(element_id: &str) -> Option<DomRect> {
    let document = document();
    let element = document.get_element_by_id(element_id)?;

    let rect = element.get_bounding_client_rect();

    Some(rect)
}

pub fn get_window_dimensions() -> (i32, i32) {
    let width = window().inner_width().unwrap().as_f64().unwrap() as i32;
    let height = window().inner_height().unwrap().as_f64().unwrap() as i32;

    (width, height)
}

/// # `setup_scroll_listener` function
///
/// This function sets up a scroll listener on the window with debounce and catch
/// last functionality. it takes three parameters: `debounce_duration`,
/// `catch_last_delay_duration`, and a `callback`. the function returns a `result` with an
/// empty tuple or a `jsvalue` error.
///
/// ## Parameters
///
/// * `debounce_duration: i32` - the debounce duration in milliseconds. scroll events
///   will be ignored if they occur within this duration.
/// * `catch_last_delay_duration: i32` - the catch last delay duration in
///   milliseconds. this is the delay when triggering the callback after the user
///   stops scrolling.
/// * `callback: f` - the callback function to be called when a scroll event occurs or
///   when the user stops scrolling. this function must be a type `fn()`.
///
/// ## Return
///
/// * `Result<(), JsValue>` - returns an empty tuple on success or a `jsvalue` error on
///   failure.
///
pub fn setup_scroll_listener<F>(
    debounce_duration: i32,
    catch_last_delay_duration: i32,
    callback: F,
) -> Result<(), JsValue>
where
    F: Fn() + 'static,
{
    let last_event_time = std::cell::RefCell::new(None);
    let last_timeout_id = std::cell::RefCell::new(None);
    let callback = Rc::new(callback);

    tracing::trace!("Setting up scroll listener");

    let on_scroll = Closure::wrap(Box::new(move |_: Event| {
        let now = js_sys::Date::now();
        let mut last_event = last_event_time.borrow_mut();
        let window = window();

        if let Some(last) = *last_event {
            if now - last < debounce_duration as f64 {
                return;
            }
        }

        tracing::trace!("Scroll event");
        callback();

        // Cancel the previously scheduled catch_last_closure
        if let Some(timeout_id) = *last_timeout_id.borrow() {
            window.clear_timeout_with_handle(timeout_id);
        }

        let catch_last_callback = callback.clone();
        let catch_last_closure = Closure::once(move || {
            tracing::trace!("Scroll event catch last");
            catch_last_callback();
        });

        let timeout_id = window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                catch_last_closure.as_ref().unchecked_ref::<Function>(),
                catch_last_delay_duration,
            )
            .expect("Unable to set timeout");

        last_timeout_id.replace(Some(timeout_id));
        last_event.replace(now);

        catch_last_closure.forget();
    }) as Box<dyn FnMut(_)>);

    let win = window();
    let window_event_target: &EventTarget = win.as_ref();
    window_event_target
        .add_event_listener_with_callback("scroll", on_scroll.as_ref().unchecked_ref())?;

    on_scroll.forget(); // Leak the closure to avoid invalidating the event listener callback

    Ok(())
}
