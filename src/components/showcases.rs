use leptos::prelude::*;

use crate::models::portfolio::ShowcaseEntry;

#[component]
pub fn Showcases(showcases: Vec<ShowcaseEntry>) -> impl IntoView {
    view! {
        <section id="showcases" class="section fade-in">
            <div class="container">
                <h2 class="section-title">"Showcases"</h2>
                <div id="showcases-list" class="card-list">
                    {showcases
                        .into_iter()
                        .map(|showcase| {
                            view! {
                                <div class="card">
                                    <h3 class="card-title">{showcase.title}</h3>
                                    <p class="card-stack">{format!("Stack: {}", showcase.stack)}</p>
                                    <ul class="card-bullets">
                                        {showcase
                                            .bullets
                                            .into_iter()
                                            .map(|bullet| {
                                                view! {
                                                    <li>{bullet}</li>
                                                }
                                            })
                                            .collect_view()}
                                    </ul>
                                    {showcase
                                        .link
                                        .map(|link| {
                                            view! {
                                                <a class="card-link" href=link.clone() target="_blank" rel="noopener">
                                                    {format!("↗ {}", link)}
                                                </a>
                                            }
                                        })}
                                </div>
                            }
                        })
                        .collect_view()}
                </div>
            </div>
        </section>
    }
}
