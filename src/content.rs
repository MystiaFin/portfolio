use leptos::prelude::*;

struct Project {
    name: &'static str,
    description: &'static str,
    language: &'static str,
    // github's color for the language
    color: &'static str,
    url: &'static str,
}

const PROJECTS: [Project; 6] = [
    Project {
        name: "amane",
        description: "A framework for building Wayland desktop shells purely in Rust",
        language: "Rust",
        color: "#dea584",
        url: "https://github.com/MystiaFin/amane",
    },
    Project {
        name: "suzuha",
        description: "A Wayland desktop shell built on Amane",
        language: "Rust",
        color: "#dea584",
        url: "https://github.com/MystiaFin/suzuha",
    },
    Project {
        name: "scalar-engine",
        description: "Self-hosted expense tracker that reads receipts from Gmail with a local LLM",
        language: "Go",
        color: "#00add8",
        url: "https://github.com/MystiaFin/scalar-engine",
    },
    Project {
        name: "scalar_mobile",
        description: "Flutter app that shows grouped transactions from a REST API",
        language: "Dart",
        color: "#00b4ab",
        url: "https://github.com/MystiaFin/scalar_mobile",
    },
    Project {
        name: "kemurnian-web",
        description: "Production school platform",
        language: "TypeScript",
        color: "#3178c6",
        url: "https://github.com/MystiaFin/kemurnian-web",
    },
    Project {
        name: "JAQC-shell",
        description: "My QML desktop shell config",
        language: "QML",
        color: "#44a51c",
        url: "https://github.com/MystiaFin/JAQC-shell",
    },
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
                    .map(|project| view! {
                        <li>
                            <a href=project.url>
                                <div class="pl-2 sm:pl-3 relative py-3 sm:py-5 font-light border-b border-black overflow-hidden group">
                                    <span class="relative z-10 block transition-colors duration-300 group-hover:text-white text-sm sm:text-base">
                                        {project.name}
                                    </span>
                                    <span class="relative z-10 block transition-colors duration-300 text-gray-500 group-hover:text-gray-300 text-xs sm:text-sm">
                                        {project.description}
                                    </span>
                                    <span class="relative z-10 mt-1.5 flex items-center gap-1.5 transition-colors duration-300 text-gray-500 group-hover:text-gray-300 text-xs">
                                        <span class="w-2 h-2 rounded-full" style:background=project.color></span>
                                        {project.language}
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
