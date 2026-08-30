use dioxus::prelude::*;
use dioxus_icons::lucide::Flame;

const AVATAR_COLORS: [&str; 8] = [
    "#56633F", "#8A472B", "#38616B", "#68496F", "#70551F", "#356052", "#70404A", "#46567A",
];

#[derive(Clone, PartialEq, Props)]
pub struct UserRowProps {
    index: i32,
    name: String,
    streak: i32,
    weight_delta: f32,

    // TODO: Estrapolare un tipo per i mesi
    month: String,

    #[props(default = 0)]
    calories: i32,

    #[props(default = 0)]
    target_calories: i32,

    #[props(default = false)]
    selected: bool
}

#[component]
pub fn UserRow(props: UserRowProps) -> Element {

    let hash = props.name.trim().bytes().fold(2_166_136_261_u32, |hash, byte| {
        (hash ^ u32::from(byte)).wrapping_mul(16_777_619)
    });
    let color = AVATAR_COLORS[hash as usize % AVATAR_COLORS.len()];

    let initials = props
        .name
        .split_whitespace()
        .filter_map(|word| word.chars().next())
        .take(2)
        .flat_map(char::to_uppercase)
        .collect::<String>();

    rsx! {
        div {
            role: "button",
            tabindex: 0,
            "data-selected": props.selected,
            class: "w-full flex justify-between items-center my-3 cursor-pointer rounded-3xl border border-transparent py-3 pl-10 pr-5 transition-colors data-[selected=true]:border-accent/50 data-[selected=true]:bg-white/50",
            div {
                class: "flex items-center gap-3",
                p {
                    class: "mr-4 text-primary-light",
                    "{props.index}"
                }
                span {
                    class: "flex size-10 shrink-0 items-center justify-center rounded-full text-sm font-semibold text-background",
                    background_color: color,
                    aria_hidden: "true",
                    "{initials}"
                }
                div {
                    p {
                        class: "font-semibold mb-1",
                        // TODO: Questo è un test per il rendering condizionale, andrà sostituito con un check sull'id dell'utente autenticato
                        if props.selected { "Tu" } else  { "{props.name}" }
                    }
                    p {
                        class: "text-xs text-primary-light",
                        "{props.weight_delta} da {props.month} · {props.calories}/{props.target_calories} kcal oggi"
                    }
                }
            }
            div {
                class: "flex items-center gap-1.5",
                Flame {
                    size: "1.25em",
                    color: "var(--color-accent)",
                    stroke_width: 3
                }
                p {
                    class: "font-semibold",
                    "{props.streak}"
                }
            }
        }
    }
}
