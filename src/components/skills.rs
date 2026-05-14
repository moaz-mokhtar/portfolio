use leptos::prelude::*;

use crate::models::portfolio::SkillGroup;

#[component]
pub fn Skills(skills: Vec<SkillGroup>) -> impl IntoView {
    view! {
        <section id="skills" class="section fade-in">
            <div class="container">
                <h2 class="section-title">"Skills"</h2>
                <div class="skills-grid" id="skills-grid">
                    {skills
                        .into_iter()
                        .map(|group| {
                            view! {
                                <div class="skill-group">
                                    <p class="skill-group-title">{group.category}</p>
                                    <div class="skill-tags">
                                        {group
                                            .items
                                            .into_iter()
                                            .map(|item| {
                                                view! {
                                                    <span class="skill-tag">{item}</span>
                                                }
                                            })
                                            .collect_view()}
                                    </div>
                                </div>
                            }
                        })
                        .collect_view()}
                </div>
            </div>
        </section>
    }
}
