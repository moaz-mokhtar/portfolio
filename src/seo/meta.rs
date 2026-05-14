use leptos::prelude::*;
use leptos_meta::{Link, Meta, Title};

const TITLE: &str = "Moaz bin Mokhtar | Rust, React, Python Software Engineer";
const DESCRIPTION: &str = "Portfolio of Moaz bin Mokhtar, a software engineer building Rust backends, React applications, and Python systems for production teams and freelance clients.";
const OG_DESCRIPTION: &str =
    "Portfolio of Moaz bin Mokhtar, a software engineer building Rust backends, React applications, and Python systems.";
const CANONICAL_URL: &str = "https://moaz-mokhtar.netlify.app/";
const OG_IMAGE_URL: &str = "https://moaz-mokhtar.netlify.app/og-image.png";
const JSON_LD: &str = r#"{
  "@context": "https://schema.org",
  "@type": "Person",
  "name": "Moaz bin Mokhtar",
  "jobTitle": "Software Engineer",
  "url": "https://moaz-mokhtar.netlify.app",
  "sameAs": [
    "https://github.com/moaz-mokhtar",
    "https://www.linkedin.com/in/moazmoktar",
    "https://www.upwork.com/freelancers/~015f28b685a73145e6"
  ],
  "knowsAbout": [
    "Rust",
    "React",
    "Python",
    "Backend Development",
    "API Design"
  ]
}"#;

#[component]
pub fn PortfolioSeo() -> impl IntoView {
    view! {
        <Title text=TITLE/>

        <Link rel="icon" type_="image/x-icon" href="/favicon.ico"/>
        <Link rel="icon" type_="image/png" sizes="32x32" href="/favicon-32x32.png"/>
        <Link rel="icon" type_="image/png" sizes="16x16" href="/favicon-16x16.png"/>
        <Link rel="apple-touch-icon" href="/apple-touch-icon.png"/>
        <Link rel="manifest" href="/site.webmanifest"/>
        <Link rel="canonical" href=CANONICAL_URL/>

        <Meta name="description" content=DESCRIPTION/>
        <Meta name="theme-color" content="#c65d1e"/>
        <Meta name="msapplication-TileImage" content="/favicon-32x32.png"/>

        <Meta property="og:title" content=TITLE/>
        <Meta property="og:description" content=OG_DESCRIPTION/>
        <Meta property="og:image" content=OG_IMAGE_URL/>
        <Meta property="og:url" content=CANONICAL_URL/>
        <Meta property="og:type" content="website"/>

        <Meta name="twitter:card" content="summary_large_image"/>
        <Meta name="twitter:title" content=TITLE/>
        <Meta name="twitter:description" content=OG_DESCRIPTION/>
        <Meta name="twitter:image" content=OG_IMAGE_URL/>

        <script type="application/ld+json">{JSON_LD}</script>
    }
}
