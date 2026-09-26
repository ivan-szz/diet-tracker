use crate::cn;
use dioxus::prelude::*;
use dioxus_icons::lucide::Flame;

/// The flame is lit while the streak runs, grey once it's lost, and pulses
/// without a number while a gap can still be filled in.
#[component]
pub fn StreakIndicator(
    streak: i32,
    at_risk: bool,
    #[props(default = "1em".to_string())] icon_size: String,
    #[props(default)] class: String,
) -> Element {
    let (label, flame_class) = if at_risk {
        ("Serie in sospeso".to_string(), "text-accent animate-pulse")
    } else if streak > 0 {
        (format!("{streak} giorni di fila"), "text-accent")
    } else {
        ("Nessuna serie in corso".to_string(), "text-primary-light/50")
    };

    rsx! {
        span {
            class: cn!("inline-flex items-center gap-1.5", class),
            title: "{label}",
            aria_label: "{label}",
            Flame {
                size: icon_size,
                class: flame_class,
                stroke_width: 3,
            }
            if !at_risk {
                span { aria_hidden: "true", "{streak}" }
            }
        }
    }
}
