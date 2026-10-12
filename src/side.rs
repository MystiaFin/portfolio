use leptos::prelude::*;

// icons are compiled into the wasm, so no extra requests after it loads
macro_rules! icon {
    ($name:literal) => {
        include_str!(concat!("../icons/", $name, ".svg"))
    };
}

const STACKS: [(&str, &str); 8] = [
    ("Rust", icon!("rust")),
    ("Go", icon!("go")),
    ("TypeScript", icon!("typescript")),
    ("React", icon!("react")),
    ("NextJS", icon!("nextjs")),
    ("Laravel", icon!("laravel")),
    ("PostgreSQL", icon!("postgresql")),
    ("NixOS", icon!("nixos")),
];

const CONNECTIONS: [(&str, &str, &str); 4] = [
    ("GitHub", icon!("github"), "https://github.com/mystiafin"),
    ("Instagram", icon!("instagram"), "https://instagram.com/mystiafin"),
    ("Discord", icon!("discord"), "https://discordapp.com/users/331269525989490688"),
    ("Email", icon!("mail"), "mailto:mystiafin@gmail.com"),
];

// <img> keeps each svg's own <style> from leaking into the page
fn data_uri(svg: &str) -> String {
    format!("data:image/svg+xml,{}", js_sys::encode_uri_component(svg))
}

#[component]
fn Stacks() -> impl IntoView {
    STACKS
        .into_iter()
        .map(|(name, svg)| view! {
            <div class="flex items-center px-2 py-1 gap-2 rounded-sm bg-[#1A1B1C] hover:bg-[#2D2F2F] transition-colors duration-200 cursor-pointer">
                <img src=data_uri(svg) alt=format!("{name} icon") class="w-4 h-4" />
                <span class="text-xs text-white">{name}</span>
            </div>
        })
        .collect_view()
}

#[component]
fn Connections(big: bool) -> impl IntoView {
    let (button, icon, text) = if big {
        ("w-24 h-24", "w-8 h-8", "text-sm")
    } else {
        ("w-20 h-20", "w-6 h-6", "text-xs")
    };

    CONNECTIONS
        .into_iter()
        .map(|(name, svg, url)| {
            let (pos, set_pos) = signal((0, 0));

            /*
             * offset_x is relative to the event target. pointer-events-none
             * on the children keeps the target on <a>, so no rect math.
             * a tap on mobile fires an emulated mousemove too.
             */
            view! {
                <a
                    href=url
                    target="_blank"
                    rel="noopener noreferrer"
                    class=format!("connection-btn flex flex-col justify-center items-center relative overflow-hidden rounded-sm {button}")
                    style=move || {
                        let (x, y) = pos.get();
                        format!("--x:{x}px;--y:{y}px")
                    }
                    on:mousemove=move |ev| set_pos.set((ev.offset_x(), ev.offset_y()))
                >
                    <img src=data_uri(svg) alt=name class=format!("{icon} mb-1 relative z-10 pointer-events-none") />
                    <span class=format!("{text} relative z-10 pointer-events-none")>{name}</span>
                </a>
            }
        })
        .collect_view()
}

#[component]
pub fn SideSection() -> impl IntoView {
    view! {
        <main class="ml-5 mr-6 max-w-sm mt-5 border-r border-gray-900 sticky top-0">
            <div class="ml-2">
                <h2 class="text-2xl font-extralight">"This is,"</h2>
                <h1 class="text-4xl font-light">"MystiaFin"</h1>
            </div>

            <div class="my-5 w-76 h-[1px] bg-black"></div>

            <div class="ml-2">
                <p class="mb-4 text-md font-extralight">
                    "Full-stack developer, lately in Rust." <br /> "Mostly with these stacks:"
                </p>
                <div class="flex flex-wrap gap-2">
                    <Stacks />
                </div>
            </div>

            <div class="ml-2 mt-8">
                <h2 class="font-light text-lg mb-3">"Connections:"</h2>
                <div class="flex bg-[#1A1B1C] w-48 rounded-sm">
                    <div class="grid grid-cols-2 mr-1 w-48">
                        <Connections big=true />
                    </div>
                </div>
                <p class="mt-6 text-sm text-gray-500">"© " {crate::year()} " MystiaFin"</p>
            </div>
        </main>
    }
}

#[component]
pub fn SideSectionMobile() -> impl IntoView {
    let (scrolled, set_scrolled) = signal(false);

    let _ = window_event_listener(leptos::ev::scroll, move |_| {
        let scroll_y = window().scroll_y().unwrap_or(0.0);

        set_scrolled.set(scroll_y > 0.0);
    });

    view! {
        <div class="mb-8 px-4">
            /*
             * -mt-8 cancels the page's 2rem top margin + padding,
             * so the hero is exactly one screen tall
             */
            <div class="relative -mt-8 min-h-svh flex flex-col justify-center">
                /*
                 * centers the ink of text + line as one unit, measured:
                 * the line hangs 1.65rem below the text's ink center.
                 * 2svh more puts it on the optical center, just above the middle
                 */
                <div class="relative text-center -translate-y-[calc(1.65rem+2svh)]">
                    <h2 class="text-4xl font-extralight">"This is,"</h2>
                    <h1 class="text-[min(3.75rem,14vw)] font-light">"MystiaFin"</h1>
                    <div class="absolute inset-x-0 top-full mt-12 h-[1px] bg-black"></div>
                </div>

                <div
                    class="absolute bottom-8 inset-x-0 flex flex-col items-center text-gray-400 pointer-events-none transition-opacity duration-300"
                    class=("opacity-0", scrolled)
                >
                    <span class="text-xs font-extralight tracking-widest">"scroll"</span>
                    <svg class="scroll-hint-arrow w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                        <path d="m6 9 6 6 6-6" />
                    </svg>
                </div>
            </div>

            <div class="mb-6 mt-80">
                <p class="mb-4 text-md font-extralight text-center">
                    "Full-stack developer, lately in Rust." <br /> "Mostly with these stacks:"
                </p>
                <div class="flex flex-wrap gap-2 justify-center">
                    <Stacks />
                </div>
            </div>

            <div class="mt-8 mb-80">
                <h2 class="font-light text-lg mb-3 text-center">"Connections:"</h2>
                <div class="flex justify-center">
                    <div class="bg-[#1A1B1C] rounded-sm p-1">
                        <div class="grid grid-cols-2 gap-1">
                            <Connections big=false />
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
