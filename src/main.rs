use leptos::prelude::*;

mod content;
mod side;

fn main() {
    leptos::mount::mount_to_body(App);
}

pub fn year() -> u32 {
    js_sys::Date::new_0().get_full_year()
}

#[component]
fn App() -> impl IntoView {
    view! {
        <main class="m-4 p-4 flex justify-center">
            // the glow is wider than a phone, so clip it to the screen width
            <div class="absolute inset-x-0 top-0 h-80 overflow-hidden -z-10 pointer-events-none">
                <div
                    class="w-96 h-96 rounded-full absolute -top-20 left-10"
                    style="background: radial-gradient(circle, rgba(0,0,0,0.2) 0%, rgba(0,0,0,0.015) 50%, rgba(0,0,0,0) 75%)"
                ></div>
            </div>
            <div class="hidden md:block">
                <side::SideSection />
            </div>
            <div class="min-w-0 w-full md:w-auto">
                <div class="md:hidden">
                    <side::SideSectionMobile />
                </div>
                <content::QuickSummary />
                <content::Projects />

                <div class="mt-10 md:hidden text-center">
                    <p class="text-sm text-gray-500">"© " {year()} " MystiaFin"</p>
                </div>
            </div>
        </main>
    }
}
