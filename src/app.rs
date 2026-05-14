use crate::components::{
    basmala::Basmala, contact::Contact, experience::Experience, footer::Footer, hero::Hero,
    navbar::Navbar, profile::Profile, showcases::Showcases, skills::Skills,
};
use crate::models::portfolio::{portfolio_data, PortfolioData};
use crate::seo::meta::PortfolioSeo;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html
        lang="en"
            data-theme="dark">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
                <script>
                    "try { let t = localStorage.getItem('theme') || 'dark'; document.documentElement.setAttribute('data-theme', t); if (t === 'light') { let i = document.getElementById('theme-icon'); if (i) i.textContent = '☾'; } } catch (e) {}"
                </script>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let portfolio = portfolio_data().clone();

    view! {
        <Stylesheet id="leptos" href="/pkg/portfolio.css"/>
        <PortfolioSeo/>
        <Router>
            <Routes fallback=|| "Page not found.".into_view()>
                <Route
                    path=StaticSegment("")
                    view=move || view! { <HomePage portfolio=portfolio.clone()/> }
                />
            </Routes>
        </Router>
    }
}

#[component]
fn HomePage(portfolio: PortfolioData) -> impl IntoView {
    view! {
        <Navbar/>
        <main>
            <Basmala/>
            <Hero contact=portfolio.contact.clone()/>
            <Profile profile=portfolio.profile/>
            <Showcases showcases=portfolio.showcases/>
            <Skills skills=portfolio.skills/>
            <Experience experience=portfolio.experience/>
            <Contact contact=portfolio.contact/>
        </main>
        <Footer/>
    }
}
