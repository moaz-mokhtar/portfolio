use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{Document, Element, Event, HtmlElement, NodeList, Storage, Window};

pub fn init() {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };

    init_theme(&window, &document);
    init_mobile_nav(&document);
    init_active_nav(&window, &document);
}

fn init_theme(window: &Window, document: &Document) {
    let Some(root) = document.document_element() else {
        return;
    };
    let Some(toggle) = document.get_element_by_id("theme-toggle") else {
        return;
    };
    let Some(icon) = document.get_element_by_id("theme-icon") else {
        return;
    };

    let storage = window.local_storage().ok().flatten();
    let saved_theme = storage
        .as_ref()
        .and_then(|item| item.get_item("theme").ok().flatten())
        .unwrap_or_else(|| "dark".to_string());

    web_sys::console::log_1(&format!("Hydrating theme: {}", saved_theme).into());
    apply_theme(&root, &icon, storage.as_ref(), &saved_theme);

    let root_clone = root.clone();
    let icon_clone = icon.clone();
    let storage_clone = storage.clone();
    let on_click = Closure::<dyn FnMut(Event)>::new(move |_| {
        let current = root_clone
            .get_attribute("data-theme")
            .unwrap_or_else(|| "dark".to_string());
        let next = if current == "dark" { "light" } else { "dark" };
        apply_theme(&root_clone, &icon_clone, storage_clone.as_ref(), next);
    });

    let _ = toggle.add_event_listener_with_callback("click", on_click.as_ref().unchecked_ref());
    on_click.forget();
}

fn apply_theme(root: &Element, icon: &Element, storage: Option<&Storage>, theme: &str) {
    let _ = root.set_attribute("data-theme", theme);
    icon.set_text_content(Some(if theme == "light" { "☾" } else { "☀" }));

    if let Some(storage) = storage {
        let _ = storage.set_item("theme", theme);
    }
}

fn init_mobile_nav(document: &Document) {
    let Some(hamburger) = document.get_element_by_id("nav-hamburger") else {
        return;
    };
    let Some(mobile_nav) = document.get_element_by_id("nav-mobile") else {
        return;
    };

    let mobile_nav_clone = mobile_nav.clone();
    let on_click = Closure::<dyn FnMut(Event)>::new(move |_| {
        let class_list = mobile_nav_clone.class_list();
        if class_list.contains("open") {
            let _ = class_list.remove_1("open");
        } else {
            let _ = class_list.add_1("open");
        }
    });

    let _ = hamburger.add_event_listener_with_callback("click", on_click.as_ref().unchecked_ref());
    on_click.forget();

    if let Ok(links) = document.query_selector_all("#nav-mobile a") {
        for_each_element(&links, |link| {
            let mobile_nav_clone = mobile_nav.clone();
            let close_on_click = Closure::<dyn FnMut(Event)>::new(move |_| {
                let _ = mobile_nav_clone.class_list().remove_1("open");
            });

            let _ = link.add_event_listener_with_callback(
                "click",
                close_on_click.as_ref().unchecked_ref(),
            );
            close_on_click.forget();
        });
    }
}

fn init_active_nav(window: &Window, document: &Document) {
    let Ok(sections) = document.query_selector_all("section[id]") else {
        return;
    };
    let Ok(nav_links) = document.query_selector_all(".nav-links a") else {
        return;
    };

    update_active_nav(window, &sections, &nav_links);

    let window_clone = window.clone();
    let sections_clone = sections.clone();
    let nav_links_clone = nav_links.clone();
    let on_scroll = Closure::<dyn FnMut(Event)>::new(move |_| {
        update_active_nav(&window_clone, &sections_clone, &nav_links_clone);
    });

    let _ = window.add_event_listener_with_callback("scroll", on_scroll.as_ref().unchecked_ref());
    on_scroll.forget();
}

fn update_active_nav(window: &Window, sections: &NodeList, nav_links: &NodeList) {
    let viewport_height = window
        .inner_height()
        .ok()
        .and_then(|value| value.as_f64())
        .unwrap_or(0.0);
    let trigger_line = viewport_height * 0.45;
    let mut active_id: Option<String> = None;

    for_each_element(sections, |section| {
        if active_id.is_some() {
            return;
        }

        let rect = section.get_bounding_client_rect();
        if rect.top() <= trigger_line && rect.bottom() >= trigger_line {
            active_id = section.get_attribute("id");
        }
    });

    for_each_element(nav_links, |link| {
        let _ = link.class_list().remove_1("active");

        if let (Some(id), Some(href)) = (active_id.as_ref(), link.get_attribute("href")) {
            if href == format!("#{id}") {
                let _ = link.class_list().add_1("active");
            }
        }
    });
}

fn for_each_element(list: &NodeList, mut f: impl FnMut(HtmlElement)) {
    for index in 0..list.length() {
        let Some(node) = list.item(index) else {
            continue;
        };
        let Ok(element) = node.dyn_into::<HtmlElement>() else {
            continue;
        };
        f(element);
    }
}
