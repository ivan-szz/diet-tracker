use dioxus::prelude::*;

const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            // Attributes should be defined in the element before any children
            id: "hero",
            class: "m-0 flex flex-col items-center justify-center",
            img { src: HEADER_SVG, id: "header", class: "w-full max-w-300" }
            div {
                id: "links",
                class: "flex w-full max-w-100 flex-col text-left text-2xl text-white [&_a]:my-2.5 [&_a]:rounded-[5px] [&_a]:border [&_a]:border-white [&_a]:p-2.5 [&_a]:text-white [&_a]:no-underline [&_a:hover]:cursor-pointer [&_a:hover]:bg-[#1f1f1f]",
                a { href: "https://dioxuslabs.com/learn/0.7/", "📚 Learn Dioxus" }
                a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
        }
    }
}
