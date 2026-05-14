use leptos::prelude::*;

use crate::models::portfolio::ContactEntry;

#[component]
pub fn Contact(contact: Vec<ContactEntry>) -> impl IntoView {
    view! {
        <section id="contact" class="section fade-in">
            <div class="container">
                <h2 class="section-title">"Contact"</h2>
                <div class="contact-block">
                    <p class="contact-desc">
                        "Available for opportunities whether engineering roles, consulting or freelance work."
                    </p>
                    <div id="contact-links" class="contact-links">
                        {contact
                            .into_iter()
                            .map(|entry| {
                                let href = if entry.platform == "Email" {
                                    format!("mailto:{}", entry.url)
                                } else {
                                    entry.url.clone()
                                };
                                let target = if entry.platform == "Email" { None } else { Some("_blank") };
                                let rel = if entry.platform == "Email" { None } else { Some("noopener") };
                                let label = if entry.platform == "Email" {
                                    entry.url.clone()
                                } else {
                                    entry.url
                                        .trim_start_matches("https://")
                                        .trim_start_matches("http://")
                                        .to_string()
                                };

                                view! {
                                    <a href=href class="hero-link" target=target rel=rel>
                                        <span class="contact-icon">{render_icon(&entry.icon)}</span>
                                        <span>{label}</span>
                                    </a>
                                }
                            })
                            .collect_view()}
                    </div>
                </div>
            </div>
        </section>
    }
}

fn render_icon(name: &str) -> impl IntoView {
    match name {
        "upwork" => view! {
            <svg
                viewBox="0 0 512 512"
                xmlns="http://www.w3.org/2000/svg"
                width="18"
                height="18"
                fill-rule="evenodd"
                clip-rule="evenodd"
                stroke-linejoin="round"
                stroke-miterlimit="2"
            >
                <ellipse cx="184.5" cy="234.5" rx="57.5" ry="56.5" transform="translate(-546.174 -763.565) scale(4.34783)"></ellipse>
                <path d="M345.516 181.708c-42.168 0-65.774 27.481-72.532 55.773-7.658-14.416-13.335-33.698-17.75-51.628H196.94v72.531c0 26.31-11.984 45.772-35.41 45.772-23.427 0-36.852-19.462-36.852-45.772l.27-72.531H91.34v72.531c0 21.174 6.848 40.366 19.372 54.061 12.884 14.146 30.454 21.534 50.817 21.534 40.545 0 68.837-31.085 68.837-75.595V209.64c4.235 16.038 14.326 46.853 33.608 73.884l-18.02 102.625h34.148l11.893-72.712c3.875 3.244 8.02 6.127 12.434 8.74 11.443 7.208 24.508 11.263 38.023 11.713 0 0 2.073.09 3.154.09 41.807 0 75.054-32.346 75.054-76.045 0-43.7-33.337-76.226-75.144-76.226m0 122.358c-25.86 0-42.979-20.003-47.754-27.752 6.127-49.015 24.057-64.512 47.754-64.512 23.426 0 41.626 18.741 41.626 46.132 0 27.39-18.2 46.132-41.626 46.132" fill="currentColor" fill-rule="nonzero"></path>
            </svg>
        }.into_any(),
        "github" => view! {
            <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor">
                <path d="M12 0C5.374 0 0 5.373 0 12c0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23A11.509 11.509 0 0112 5.803c1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.652.242 2.874.118 3.176.77.84 1.235 1.91 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576C20.566 21.797 24 17.3 24 12c0-6.627-5.373-12-12-12z"></path>
            </svg>
        }.into_any(),
        "linkedin" => view! {
            <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor">
                <path d="M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433c-1.144 0-2.063-.926-2.063-2.065 0-1.138.92-2.063 2.063-2.063 1.14 0 2.064.925 2.064 2.063 0 1.139-.925 2.065-2.064 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.222 0h.003z"></path>
            </svg>
        }.into_any(),
        "email" => view! {
            <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor">
                <path d="M20 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 4l-8 5-8-5V6l8 5 8-5v2z"></path>
            </svg>
        }.into_any(),
        _ => view! {}.into_any(),
    }
}
