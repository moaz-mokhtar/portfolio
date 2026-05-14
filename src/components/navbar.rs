use leptos::prelude::*;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav id="navbar">
            <div class="nav-inner">
                <a href="#" class="nav-logo-link">
                    <img src="/logo.png" alt="Moaz Mokhtar Logo" class="nav-logo-img"/>
                    <span class="nav-logo">"Moaz's Portfolio"</span>
                </a>
                <ul class="nav-links">
                    <li><a href="#profile">"Profile"</a></li>
                    <li><a href="#showcases">"Showcases"</a></li>
                    <li><a href="#skills">"Skills"</a></li>
                    <li><a href="#experience">"Experience"</a></li>
                    <li><a href="#contact">"Contact"</a></li>
                </ul>
                <button id="theme-toggle" aria-label="Toggle light/dark mode" title="Toggle theme">
                    <span id="theme-icon">"☀"</span>
                </button>
                <button class="nav-hamburger" id="nav-hamburger" aria-label="Open menu">
                    <span></span><span></span><span></span>
                </button>
            </div>
            <ul class="nav-mobile" id="nav-mobile">
                <li><a href="#hero">"Home"</a></li>
                <li><a href="#profile">"Profile"</a></li>
                <li><a href="#skills">"Skills"</a></li>
                <li><a href="#experience">"Experience"</a></li>
                <li><a href="#showcases">"Showcases"</a></li>
                <li><a href="#contact">"Contact"</a></li>
            </ul>
        </nav>
    }
}
