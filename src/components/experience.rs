use leptos::prelude::*;

use crate::models::portfolio::ExperienceEntry;

#[component]
pub fn Experience(experience: Vec<ExperienceEntry>) -> impl IntoView {
    view! {
        <section id="experience" class="section fade-in">
            <div class="container">
                <h2 class="section-title">"Experience"</h2>
                <div id="experience-list" class="card-list">
                    {experience
                        .into_iter()
                        .map(|entry| {
                            view! {
                                <div class="card">
                                    <div class="card-header">
                                        <h3 class="card-title">{entry.title}</h3>
                                        <span class="card-meta">{entry.period}</span>
                                    </div>
                                    <ul class="card-bullets">
                                        {entry
                                            .bullets
                                            .into_iter()
                                            .map(|bullet| {
                                                view! {
                                                    <li>{bullet}</li>
                                                }
                                            })
                                            .collect_view()}
                                    </ul>
                                </div>
                            }
                        })
                        .collect_view()}
                </div>
            </div>
        </section>
    }
}
