use leptos::prelude::*;

const PROJECTS: [(&str, &str); 5] = [
    ("dynoboo-web-mobile", "https://github.com/MystiaFin/dynoboo-web-mobile.git"),
    ("dynoboo-server", "https://github.com/MystiaFin/dynoboo-server.git"),
    ("WeatherAPP", "https://github.com/MystiaFin/WeatherAPP"),
    ("redpost", "https://github.com/MystiaFin/redpost"),
    ("personal-website-v2", "https://github.com/MystiaFin/personal-website-v2"),
];

#[component]
pub fn QuickSummary() -> impl IntoView {
    view! {
        <main class="max-w-3xl mt-10 px-4 sm:px-6 md:px-0">
            <h1 class="text-2xl sm:text-3xl font-light mb-3 sm:mb-5">"Quick Summary"</h1>
            <p class="mt-2 text-sm sm:text-md font-extralight max-w-xl">
                "Third-year Computer Science student from Indonesia with 2 years of
                full-stack web development experience. Currently building personal
                projects to deepen understanding of software development."
            </p>
            <div class="w-full bg-black h-[1px] my-4 sm:my-5"></div>
        </main>
    }
}

#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <main class="px-4 sm:px-0">
            <h1 class="text-2xl sm:text-3xl font-light mb-3 sm:mb-5">"Projects"</h1>
            <ul>
                {PROJECTS
                    .into_iter()
                    .map(|(name, url)| view! {
                        <li>
                            <a href=url>
                                <div class="pl-2 sm:pl-3 relative py-3 sm:py-5 font-extralight border-b border-black overflow-hidden group">
                                    <span class="relative z-10 transition-colors duration-300 group-hover:text-white text-sm sm:text-base">
                                        {name}
                                    </span>
                                </div>
                            </a>
                        </li>
                    })
                    .collect_view()}
            </ul>
        </main>
    }
}
