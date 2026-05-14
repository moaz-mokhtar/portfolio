use leptos::prelude::*;

use crate::models::portfolio::ProfileEntry;

#[component]
pub fn Profile(profile: Vec<ProfileEntry>) -> impl IntoView {
    view! {
        <section id="profile" class="section fade-in">
            <div class="container">
                <h2 class="section-title">"Profile"</h2>
                <ul id="profile-list" class="profile-list">
                    {profile
                        .into_iter()
                        .map(|entry| {
                            let link = entry.link;

                            view! {
                                <li>
                                    {entry.text}
                                    ". "
                                    {link
                                        .map(|href| {
                                            view! {
                                                <a href=href target="_blank" rel="noopener">
                                                    "Link"
                                                </a>
                                            }
                                        })}
                                </li>
                            }
                        })
                        .collect_view()}
                </ul>
            </div>
        </section>
    }
}
