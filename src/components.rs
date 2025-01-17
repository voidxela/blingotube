use dioxus::prelude::*;
const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const BLINGO_PNG: Asset = asset!("/assets/blingo.png");

static COUNT: GlobalSignal<u64> = Signal::global(|| 0);

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        BlingoClicker {}
    }
}

#[component]
pub fn Blingo(scale: u64, angle: i64) -> Element {
    rsx! {
        img {
            src: BLINGO_PNG,
            class: "blingo",
            style: format!("transform: scale({}%) rotate({}deg)", scale, angle),
            onclick: move |_| *COUNT.write() += 1,
        }
    }
}

#[component]
pub fn BlingoClicker() -> Element {
    let count = *COUNT.read();
    rsx! {
        main {
            h1 { "blingos clicked: {count}" }
            div {
                class: "blingo-clicker",
                Blingo { scale: (count % 10 + 10) * 100 / 10, angle: ((count * 25) % 360_000).try_into().unwrap() }
            }
        }
        footer {
            a { href: "https://blingo.tube", "Look at him go!" }
            a { href: "https://github.com/voidxela/blingotube", "Source code" }
        }
    }
}
