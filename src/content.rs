use leptos::prelude::*;

const PROJECTS: [(&str, &str, &str); 6] = [
    ("amane", "A framework for building Wayland desktop shells purely in Rust", "https://github.com/MystiaFin/amane"),
    ("suzuha", "A Wayland desktop shell built on Amane", "https://github.com/MystiaFin/suzuha"),
    ("scalar-engine", "Self-hosted expense tracker that reads receipts from Gmail with a local LLM", "https://github.com/MystiaFin/scalar-engine"),
    ("scalar_mobile", "Flutter app that shows grouped transactions from a REST API", "https://github.com/MystiaFin/scalar_mobile"),
    ("kemurnian-web", "Production school platform", "https://github.com/MystiaFin/kemurnian-web"),
    ("JAQC-shell", "My QML desktop shell config", "https://github.com/MystiaFin/JAQC-shell"),
];

#[component]
pub fn QuickSummary() -> impl IntoView {
    view! {
        <main class="max-w-3xl mt-10 px-4 sm:px-6 md:px-0">
            <h1 class="text-2xl sm:text-3xl font-light mb-3 sm:mb-5">"Quick Summary"</h1>
            <p class="mt-2 text-sm sm:text-md font-extralight max-w-xl">
                "Third-year Computer Science student from Indonesia and full-stack
                developer. Lately working closer to the system: building Amane, a
                framework for Wayland desktop shells in pure Rust, and Suzuha, a shell
                built on top of it."
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
                    .map(|(name, description, url)| view! {
                        <li>
                            <a href=url>
                                <div class="pl-2 sm:pl-3 relative py-3 sm:py-5 font-extralight border-b border-black overflow-hidden group">
                                    <span class="relative z-10 block transition-colors duration-300 group-hover:text-white text-sm sm:text-base">
                                        {name}
                                    </span>
                                    <span class="relative z-10 block transition-colors duration-300 text-gray-500 group-hover:text-gray-300 text-xs sm:text-sm">
                                        {description}
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
