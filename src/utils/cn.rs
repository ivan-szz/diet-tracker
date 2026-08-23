//! `cn` — Tailwind aware class name composition.
//!
//! A Rust port of the `cn` helper from the JavaScript ecosystem, i.e.
//! `twMerge(clsx(...))`:
//!
//! * the **clsx** half flattens a heterogeneous list of inputs (`&str`,
//!   `String`, `Option<_>`, `(bool, _)`, arrays/slices/`Vec`s of those) into a
//!   single space separated class list, skipping the empty ones;
//! * the **tailwind-merge** half resolves conflicts between utilities that
//!   target the same CSS property, keeping the *last* one, so a caller can
//!   override a component's defaults just by passing a class.
//!
//! ```ignore
//! use crate::cn;
//!
//! cn!("px-2 py-1", "p-3");                // -> "p-3"
//! cn!("text-sm", Some("text-lg"));        // -> "text-lg"
//! cn!("flex", (is_active, "bg-accent"));  // -> "flex" or "flex bg-accent"
//! cn!(BUTTON_CLASSES, variant.classes(), class /* Option<String> prop */);
//! ```
//!
//! Conflicts are scoped per variant, exactly like `tailwind-merge`:
//! `hover:p-2` never overrides `p-4`, and `!p-2` never overrides `p-4`.
//! Utilities the resolver does not recognise are always kept; repeats of the
//! very same class collapse into the last one.

// The helper is a self contained utility: parts of it are only exercised by the
// tests until the components adopt it.
#![allow(dead_code)]

use std::borrow::Cow;
use std::collections::HashSet;

/// Composes class names: `cn!(a, b, c)` is `tw_merge(clsx(a, b, c))`.
///
/// Every argument implements [`ClassValue`], so `&str`, `String`,
/// `Option<_>`, `(bool, _)`, `Vec<_>` and references to those can be mixed
/// freely. Falsy inputs (`None`, `(false, _)`, empty strings) are skipped.
#[macro_export]
macro_rules! cn {
    ($($class:expr),* $(,)?) => {{
        let mut builder = $crate::utils::cn::ClassBuilder::new();
        $( builder.push($class); )*
        builder.finish()
    }};
}

/// Accumulator behind [`cn!`]: the `clsx` half of the helper.
#[derive(Debug, Default, Clone)]
pub struct ClassBuilder {
    raw: String,
}

impl ClassBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Appends a value, ignoring the falsy ones.
    pub fn push(&mut self, value: impl ClassValue) -> &mut Self {
        value.write_classes(&mut self.raw);
        self
    }

    /// The joined class list *before* conflict resolution (plain `clsx`).
    pub fn as_str(&self) -> &str {
        &self.raw
    }

    /// The joined class list with Tailwind conflicts resolved.
    pub fn finish(&self) -> String {
        tw_merge(&self.raw)
    }
}

/// Anything that can contribute class names to a [`ClassBuilder`].
pub trait ClassValue {
    fn write_classes(&self, out: &mut String);
}

fn write_str(out: &mut String, value: &str) {
    let value = value.trim();
    if value.is_empty() {
        return;
    }
    if !out.is_empty() {
        out.push(' ');
    }
    out.push_str(value);
}

impl ClassValue for str {
    fn write_classes(&self, out: &mut String) {
        write_str(out, self);
    }
}

impl ClassValue for String {
    fn write_classes(&self, out: &mut String) {
        write_str(out, self);
    }
}

impl ClassValue for Cow<'_, str> {
    fn write_classes(&self, out: &mut String) {
        write_str(out, self);
    }
}

/// A bare `bool` carries no class, but makes `cn!(a, cond && b)` style code
/// compile the way it does in JavaScript.
impl ClassValue for bool {
    fn write_classes(&self, _out: &mut String) {}
}

impl<T: ClassValue + ?Sized> ClassValue for &T {
    fn write_classes(&self, out: &mut String) {
        (**self).write_classes(out);
    }
}

impl<T: ClassValue> ClassValue for Option<T> {
    fn write_classes(&self, out: &mut String) {
        if let Some(value) = self {
            value.write_classes(out);
        }
    }
}

/// `(condition, classes)` mirrors the `cond && "classes"` idiom of `clsx`.
impl<T: ClassValue> ClassValue for (bool, T) {
    fn write_classes(&self, out: &mut String) {
        if self.0 {
            self.1.write_classes(out);
        }
    }
}

impl<T: ClassValue> ClassValue for [T] {
    fn write_classes(&self, out: &mut String) {
        for value in self {
            value.write_classes(out);
        }
    }
}

impl<T: ClassValue, const N: usize> ClassValue for [T; N] {
    fn write_classes(&self, out: &mut String) {
        self[..].write_classes(out);
    }
}

impl<T: ClassValue> ClassValue for Vec<T> {
    fn write_classes(&self, out: &mut String) {
        self[..].write_classes(out);
    }
}

/// Resolves conflicting Tailwind utilities in a class list, keeping the last
/// one of every conflicting group. This is the `tailwind-merge` half of
/// [`cn!`], usable on its own.
pub fn tw_merge(classes: &str) -> String {
    let mut kept: Vec<&str> = Vec::new();
    let mut claimed: HashSet<String> = HashSet::new();

    // Walking backwards means the *last* class of a group is the one we see
    // first, so it wins and everything it conflicts with is dropped.
    for class in classes.split_whitespace().rev() {
        let parsed = ParsedClass::parse(class);
        let group = class_group(parsed.base);

        let key = match &group {
            // Unknown utilities are keyed by their own text: they never
            // override anything, but an exact repeat still collapses.
            None => parsed.key(&format!("#{}", parsed.base)),
            Some(group) => parsed.key(group),
        };

        if !claimed.insert(key) {
            continue;
        }

        if let Some(group) = &group {
            for conflict in conflicting_groups(group) {
                claimed.insert(parsed.key(conflict));
            }
        }

        kept.push(class);
    }

    kept.reverse();
    kept.join(" ")
}

/// A class split into its variant prefix and the utility itself.
struct ParsedClass<'a> {
    /// Variants in a canonical order, plus the important flag, e.g.
    /// `"focus:hover:"` or `"hover:!"`. Conflicts are only resolved between
    /// classes sharing this prefix.
    modifiers: String,
    base: &'a str,
}

impl<'a> ParsedClass<'a> {
    fn parse(class: &'a str) -> Self {
        let (modifiers, base) = split_modifiers(class);

        // `!p-2` (v3) and `p-2!` (v4) both mark the utility as important, and
        // an important utility only ever conflicts with another important one.
        let (base, important) = match base.strip_prefix('!') {
            Some(base) => (base, true),
            None => match base.strip_suffix('!') {
                Some(base) => (base, true),
                None => (base, false),
            },
        };

        let mut prefix = String::new();
        for modifier in sort_modifiers(modifiers) {
            prefix.push_str(modifier);
            prefix.push(':');
        }
        if important {
            prefix.push('!');
        }

        Self {
            modifiers: prefix,
            base,
        }
    }

    fn key(&self, group: &str) -> String {
        // `\u{1}` cannot appear in a class name, so it is a safe separator.
        format!("{}\u{1}{group}", self.modifiers)
    }
}

/// Splits `hover:has-[>svg]:px-3` into `(["hover", "has-[>svg]"], "px-3")`.
///
/// Colons nested inside `[]` or `()` belong to arbitrary values and variants
/// (`[&_svg:not([class*='size-'])]:size-4`), so they never split.
fn split_modifiers(class: &str) -> (Vec<&str>, &str) {
    let mut modifiers = Vec::new();
    let mut depth = 0i32;
    let mut start = 0usize;

    for (index, char) in class.char_indices() {
        match char {
            '[' | '(' => depth += 1,
            ']' | ')' => depth -= 1,
            ':' if depth == 0 => {
                modifiers.push(&class[start..index]);
                start = index + char.len_utf8();
            }
            _ => {}
        }
    }

    (modifiers, &class[start..])
}

/// Variants are order independent for conflict purposes, so they are sorted —
/// except around arbitrary variants, which act as barriers because they can
/// change what the variants after them apply to.
fn sort_modifiers(modifiers: Vec<&str>) -> Vec<&str> {
    let mut sorted = Vec::with_capacity(modifiers.len());
    let mut chunk: Vec<&str> = Vec::new();

    for modifier in modifiers {
        if modifier.starts_with('[') {
            chunk.sort_unstable();
            sorted.append(&mut chunk);
            sorted.push(modifier);
        } else {
            chunk.push(modifier);
        }
    }

    chunk.sort_unstable();
    sorted.append(&mut chunk);
    sorted
}

/// The CSS "slot" a utility writes to. Two classes with the same group (and the
/// same variants) conflict, so only the last one survives.
type Group<'a> = Option<Cow<'a, str>>;

fn named(group: &'static str) -> Group<'static> {
    Some(Cow::Borrowed(group))
}

fn owned(group: String) -> Group<'static> {
    Some(Cow::Owned(group))
}

const DISPLAY: &[&str] = &[
    "block",
    "inline-block",
    "inline",
    "flex",
    "inline-flex",
    "table",
    "inline-table",
    "table-caption",
    "table-cell",
    "table-column",
    "table-column-group",
    "table-footer-group",
    "table-header-group",
    "table-row-group",
    "table-row",
    "flow-root",
    "grid",
    "inline-grid",
    "contents",
    "list-item",
    "hidden",
];

const POSITION: &[&str] = &["static", "fixed", "absolute", "relative", "sticky"];
const LINE_STYLES: &[&str] = &["solid", "dashed", "dotted", "double", "hidden", "none"];
const BORDER_SIDES: &[&str] = &["x", "y", "t", "r", "b", "l", "s", "e"];
const CORNERS: &[&str] = &[
    "s", "e", "t", "r", "b", "l", "ss", "se", "ee", "es", "tl", "tr", "br", "bl",
];
const FONT_WEIGHTS: &[&str] = &[
    "thin",
    "extralight",
    "light",
    "normal",
    "medium",
    "semibold",
    "bold",
    "extrabold",
    "black",
];
const TEXT_ALIGN: &[&str] = &["left", "center", "right", "justify", "start", "end"];
const BG_POSITIONS: &[&str] = &[
    "bottom",
    "center",
    "left",
    "right",
    "top",
    "left-bottom",
    "left-top",
    "right-bottom",
    "right-top",
    "bottom-left",
    "bottom-right",
    "top-left",
    "top-right",
];

/// Utilities that are a single keyword, where the value cannot disambiguate the
/// group (`flex` the display vs `flex-row` the direction).
fn keyword_group(base: &str) -> Option<&'static str> {
    if DISPLAY.contains(&base) {
        return Some("display");
    }
    if POSITION.contains(&base) {
        return Some("position");
    }

    let group = match base {
        "visible" | "invisible" | "collapse" => "visibility",
        "isolate" | "isolation-auto" => "isolation",
        "sr-only" | "not-sr-only" => "sr",
        "italic" | "not-italic" => "font-style",
        "antialiased" | "subpixel-antialiased" => "font-smoothing",
        "underline" | "overline" | "line-through" | "no-underline" => "text-decoration",
        "uppercase" | "lowercase" | "capitalize" | "normal-case" => "text-transform",
        "truncate" => "text-overflow",
        "normal-nums" => "fvn-normal",
        "ordinal" => "fvn-ordinal",
        "slashed-zero" => "fvn-slashed-zero",
        "lining-nums" | "oldstyle-nums" => "fvn-figure",
        "proportional-nums" | "tabular-nums" => "fvn-spacing",
        "diagonal-fractions" | "stacked-fractions" => "fvn-fraction",
        "container" => "container",
        "border" => "border-w",
        "rounded" => "rounded",
        "ring" => "ring-w",
        "shadow" => "shadow",
        "outline" => "outline-style",
        "transform" => "transform",
        "filter" => "filter",
        "blur" => "blur",
        "grayscale" => "grayscale",
        "invert" => "invert",
        "sepia" => "sepia",
        "grow" => "grow",
        "shrink" => "shrink",
        "transition" => "transition",
        "resize" => "resize",
        "underline-offset" => "underline-offset",
        _ => return None,
    };

    Some(group)
}

fn class_group(base: &str) -> Group<'_> {
    // Arbitrary properties (`[mask-type:luminance]`) are keyed by the property.
    if let Some(property) = arbitrary_property(base) {
        return Some(Cow::Borrowed(property));
    }

    // A leading `-` only negates the value, it does not change the group.
    let base = base.strip_prefix('-').unwrap_or(base);

    if let Some(group) = keyword_group(base) {
        return named(group);
    }

    let (head, rest) = match base.split_once('-') {
        Some((head, rest)) => (head, Some(rest)),
        None => (base, None),
    };

    match head {
        // -- Layout ------------------------------------------------------
        "aspect" => named("aspect"),
        "columns" => named("columns"),
        "break" => match first_segment(rest?).0 {
            "after" => named("break-after"),
            "before" => named("break-before"),
            "inside" => named("break-inside"),
            _ => named("break"),
        },
        "box" => match first_segment(rest?).0 {
            "decoration" => named("box-decoration"),
            _ => named("box-sizing"),
        },
        "float" => named("float"),
        "clear" => named("clear"),
        "object" => match rest? {
            "contain" | "cover" | "fill" | "none" | "scale-down" => named("object-fit"),
            _ => named("object-position"),
        },
        "overflow" | "overscroll" => match axis(rest?) {
            Some(axis) => owned(format!("{head}-{axis}")),
            None => head_group(head),
        },
        "inset" => {
            let rest = rest?;
            if let Some(shadow) = rest.strip_prefix("shadow-") {
                return if is_color_value(shadow) {
                    named("inset-shadow-color")
                } else {
                    named("inset-shadow")
                };
            }
            match axis(rest) {
                Some(axis) => owned(format!("inset-{axis}")),
                None => named("inset"),
            }
        }
        "top" | "right" | "bottom" | "left" | "start" | "end" => head_group(head),
        "z" => named("z"),

        // -- Flexbox and grid --------------------------------------------
        "basis" => named("basis"),
        "flex" => match rest? {
            "row" | "row-reverse" | "col" | "col-reverse" => named("flex-direction"),
            "wrap" | "wrap-reverse" | "nowrap" => named("flex-wrap"),
            _ => named("flex"),
        },
        "grow" => named("grow"),
        "shrink" => named("shrink"),
        "order" => named("order"),
        "grid" => match first_segment(rest?).0 {
            "cols" => named("grid-cols"),
            "rows" => named("grid-rows"),
            "flow" => named("grid-flow"),
            _ => None,
        },
        "col" | "row" => match first_segment(rest?).0 {
            "start" => owned(format!("{head}-start")),
            "end" => owned(format!("{head}-end")),
            _ => owned(format!("{head}-start-end")),
        },
        "auto" => match first_segment(rest?).0 {
            "cols" => named("auto-cols"),
            "rows" => named("auto-rows"),
            _ => None,
        },
        "gap" => match axis(rest?) {
            Some(axis) => owned(format!("gap-{axis}")),
            None => named("gap"),
        },
        "justify" => match first_segment(rest?).0 {
            "items" => named("justify-items"),
            "self" => named("justify-self"),
            _ => named("justify-content"),
        },
        "content" => {
            let rest = rest?;
            if rest == "none" || is_arbitrary(rest) || is_css_var(rest) {
                named("content")
            } else {
                named("align-content")
            }
        }
        "items" => named("align-items"),
        "self" => named("align-self"),
        "place" => match first_segment(rest?).0 {
            "content" => named("place-content"),
            "items" => named("place-items"),
            "self" => named("place-self"),
            _ => None,
        },

        // -- Spacing ------------------------------------------------------
        "p" | "px" | "py" | "pt" | "pr" | "pb" | "pl" | "ps" | "pe" | "m" | "mx" | "my" | "mt"
        | "mr" | "mb" | "ml" | "ms" | "me" => {
            rest?;
            head_group(head)
        }
        "space" => {
            let rest = rest?;
            match (axis(rest), rest.ends_with("-reverse")) {
                (Some(axis), true) => owned(format!("space-{axis}-reverse")),
                (Some(axis), false) => owned(format!("space-{axis}")),
                (None, _) => None,
            }
        }

        // -- Sizing --------------------------------------------------------
        "w" | "h" | "size" => {
            rest?;
            head_group(head)
        }
        "min" | "max" => match first_segment(rest?).0 {
            "w" => owned(format!("{head}-w")),
            "h" => owned(format!("{head}-h")),
            _ => None,
        },

        // -- Typography ----------------------------------------------------
        "font" => {
            let rest = rest?;
            if rest.starts_with("stretch") {
                named("font-stretch")
            } else if FONT_WEIGHTS.contains(&rest) || is_number(rest) {
                named("font-weight")
            } else {
                named("font-family")
            }
        }
        "text" => {
            let rest = rest?;
            if TEXT_ALIGN.contains(&rest) {
                named("text-align")
            } else if matches!(rest, "wrap" | "nowrap" | "balance" | "pretty") {
                named("text-wrap")
            } else if matches!(rest, "ellipsis" | "clip") {
                named("text-overflow")
            } else if is_size_value(rest) {
                named("font-size")
            } else {
                named("text-color")
            }
        }
        "tracking" => named("tracking"),
        "leading" => named("leading"),
        "list" => match rest? {
            "inside" | "outside" => named("list-style-position"),
            rest if rest.starts_with("image") => named("list-image"),
            _ => named("list-style-type"),
        },
        "placeholder" => named("placeholder-color"),
        "underline" => {
            if first_segment(rest?).0 == "offset" {
                named("underline-offset")
            } else {
                named("text-decoration")
            }
        }
        "decoration" => {
            let rest = rest?;
            if LINE_STYLES.contains(&rest) || rest == "wavy" {
                named("text-decoration-style")
            } else if is_length_value(rest) || is_tshirt_size(rest) {
                named("text-decoration-thickness")
            } else {
                named("text-decoration-color")
            }
        }
        "indent" => named("indent"),
        "align" => named("vertical-align"),
        "whitespace" => named("whitespace"),
        "hyphens" => named("hyphens"),

        // -- Backgrounds ----------------------------------------------------
        "bg" => {
            let rest = rest?;
            if BG_POSITIONS.contains(&rest) {
                return named("bg-position");
            }
            match first_segment(rest).0 {
                "none" | "gradient" | "linear" | "radial" | "conic" => named("bg-image"),
                "cover" | "contain" | "size" => named("bg-size"),
                "repeat" => named("bg-repeat"),
                "no" if rest == "no-repeat" => named("bg-repeat"),
                "fixed" | "local" | "scroll" => named("bg-attachment"),
                "clip" => named("bg-clip"),
                "origin" => named("bg-origin"),
                "blend" => named("bg-blend"),
                "position" => named("bg-position"),
                _ => named("bg-color"),
            }
        }

        // -- Borders ---------------------------------------------------------
        "rounded" => {
            let rest = rest?;
            let (segment, tail) = first_segment(rest);
            if CORNERS.contains(&segment) && (tail.is_some() || rest == segment) {
                owned(format!("rounded-{segment}"))
            } else {
                named("rounded")
            }
        }
        "border" => {
            let rest = rest?;
            if matches!(rest, "collapse" | "separate") {
                return named("border-collapse");
            }
            if let Some(spacing) = rest.strip_prefix("spacing") {
                return match axis(spacing.trim_start_matches('-')) {
                    Some(axis) => owned(format!("border-spacing-{axis}")),
                    None => named("border-spacing"),
                };
            }
            if LINE_STYLES.contains(&rest) {
                return named("border-style");
            }
            let (segment, tail) = first_segment(rest);
            if BORDER_SIDES.contains(&segment) {
                return match tail {
                    None => owned(format!("border-w-{segment}")),
                    Some(tail) if is_length_value(tail) => owned(format!("border-w-{segment}")),
                    Some(_) => owned(format!("border-color-{segment}")),
                };
            }
            if is_length_value(rest) {
                named("border-w")
            } else {
                named("border-color")
            }
        }
        "divide" => {
            let rest = rest?;
            if LINE_STYLES.contains(&rest) {
                return named("divide-style");
            }
            match (axis(rest), rest.ends_with("-reverse")) {
                (Some(axis), true) => owned(format!("divide-{axis}-reverse")),
                (Some(axis), false) => owned(format!("divide-{axis}")),
                (None, _) => named("divide-color"),
            }
        }
        "outline" => {
            let rest = rest?;
            if LINE_STYLES.contains(&rest) {
                return named("outline-style");
            }
            if rest.starts_with("offset-") {
                return named("outline-offset");
            }
            if is_length_value(rest) {
                named("outline-w")
            } else {
                named("outline-color")
            }
        }
        "ring" => {
            let rest = rest?;
            if rest == "inset" {
                return named("ring-w-inset");
            }
            if let Some(offset) = rest.strip_prefix("offset-") {
                return if is_length_value(offset) {
                    named("ring-offset-w")
                } else {
                    named("ring-offset-color")
                };
            }
            if is_length_value(rest) {
                named("ring-w")
            } else {
                named("ring-color")
            }
        }

        // -- Effects and filters ----------------------------------------------
        "shadow" => {
            let rest = rest?;
            if is_color_value(rest) {
                named("shadow-color")
            } else {
                named("shadow")
            }
        }
        "opacity" => named("opacity"),
        "mix" => named("mix-blend"),
        "blur" | "brightness" | "contrast" | "grayscale" | "invert" | "saturate" | "sepia" => {
            rest?;
            head_group(head)
        }
        "drop" => named("drop-shadow"),
        "hue" => named("hue-rotate"),
        "backdrop" => {
            let rest = rest?;
            if rest.starts_with("hue-rotate") {
                return named("backdrop-hue-rotate");
            }
            owned(format!("backdrop-{}", first_segment(rest).0))
        }

        // -- Tables -------------------------------------------------------------
        "table" => named("table-layout"),
        "caption" => named("caption"),

        // -- Transitions and animation ------------------------------------------
        "transition" => named("transition"),
        "duration" => named("duration"),
        "ease" => named("ease"),
        "delay" => named("delay"),
        "animate" => named("animate"),

        // -- Transforms -----------------------------------------------------------
        "scale" | "rotate" | "translate" | "skew" => match axis(rest?) {
            Some(axis) => owned(format!("{head}-{axis}")),
            None => head_group(head),
        },
        "origin" => named("transform-origin"),
        "transform" => match rest? {
            "none" | "gpu" | "cpu" => named("transform"),
            rest if rest.starts_with("style") => named("transform-style"),
            rest if rest.starts_with("box") => named("transform-box"),
            _ => None,
        },
        "perspective" => {
            if first_segment(rest?).0 == "origin" {
                named("perspective-origin")
            } else {
                named("perspective")
            }
        }

        // -- Interactivity ----------------------------------------------------------
        "accent" => named("accent"),
        "appearance" => named("appearance"),
        "caret" => named("caret-color"),
        "cursor" => named("cursor"),
        "field" => named("field-sizing"),
        "pointer" => named("pointer-events"),
        "resize" => named("resize"),
        "select" => named("select"),
        "will" => named("will-change"),
        "scroll" => {
            let rest = rest?;
            if matches!(rest, "auto" | "smooth") {
                return named("scroll-behavior");
            }
            let (segment, _) = first_segment(rest);
            if segment.len() <= 2 && (segment.starts_with('m') || segment.starts_with('p')) {
                owned(format!("scroll-{segment}"))
            } else {
                None
            }
        }
        "snap" => match rest? {
            "none" | "x" | "y" | "both" => named("snap-type"),
            "mandatory" | "proximity" => named("snap-strictness"),
            "start" | "end" | "center" | "align-none" => named("snap-align"),
            "normal" | "always" => named("snap-stop"),
            _ => None,
        },
        "touch" => match rest? {
            "auto" | "none" | "manipulation" => named("touch"),
            "pan-x" | "pan-left" | "pan-right" => named("touch-x"),
            "pan-y" | "pan-up" | "pan-down" => named("touch-y"),
            "pinch-zoom" => named("touch-pz"),
            _ => None,
        },

        // -- SVG and accessibility -----------------------------------------------------
        "fill" => named("fill"),
        "stroke" => {
            if is_length_value(rest?) {
                named("stroke-w")
            } else {
                named("stroke")
            }
        }
        "forced" => named("forced-color-adjust"),

        _ => None,
    }
}

/// Groups that are simply named after the utility prefix.
fn head_group(head: &str) -> Group<'static> {
    const HEADS: &[&str] = &[
        "overflow",
        "overscroll",
        "top",
        "right",
        "bottom",
        "left",
        "start",
        "end",
        "p",
        "px",
        "py",
        "pt",
        "pr",
        "pb",
        "pl",
        "ps",
        "pe",
        "m",
        "mx",
        "my",
        "mt",
        "mr",
        "mb",
        "ml",
        "ms",
        "me",
        "w",
        "h",
        "size",
        "blur",
        "brightness",
        "contrast",
        "grayscale",
        "invert",
        "saturate",
        "sepia",
        "scale",
        "rotate",
        "translate",
        "skew",
    ];

    HEADS
        .iter()
        .find(|candidate| **candidate == head)
        .map(|group| Cow::Borrowed(*group))
}

/// `"x-4"` -> `Some("x")`. Used by the utilities that come in axis flavours.
fn axis(rest: &str) -> Option<&'static str> {
    match first_segment(rest).0 {
        "x" => Some("x"),
        "y" => Some("y"),
        "z" => Some("z"),
        _ => None,
    }
}

/// `"offset-4"` -> `("offset", Some("4"))`.
fn first_segment(rest: &str) -> (&str, Option<&str>) {
    match rest.split_once('-') {
        Some((segment, tail)) => (segment, Some(tail)),
        None => (rest, None),
    }
}

/// `"[mask-type:luminance]"` -> `Some("mask-type")`.
fn arbitrary_property(class: &str) -> Option<&str> {
    let inner = class.strip_prefix('[')?.strip_suffix(']')?;
    let (property, _) = inner.split_once(':')?;
    let valid = !property.is_empty()
        && property
            .chars()
            .all(|char| char.is_ascii_lowercase() || char == '-');
    valid.then_some(property)
}

fn is_arbitrary(value: &str) -> bool {
    value.starts_with('[') && value.ends_with(']')
}

/// Tailwind v4 shorthand for a CSS variable, e.g. `w-(--sidebar)`.
fn is_css_var(value: &str) -> bool {
    value.starts_with('(') && value.ends_with(')')
}

fn is_number(value: &str) -> bool {
    !value.is_empty() && value.parse::<f64>().is_ok()
}

fn is_fraction(value: &str) -> bool {
    match value.split_once('/') {
        Some((numerator, denominator)) => is_number(numerator) && is_number(denominator),
        None => false,
    }
}

/// `xs`, `sm`, `2xl`, ... optionally followed by a `/line-height` modifier.
fn is_tshirt_size(value: &str) -> bool {
    let value = value.split('/').next().unwrap_or(value);
    let value = value.trim_start_matches(|char: char| char.is_ascii_digit() || char == '.');
    matches!(value, "xs" | "sm" | "md" | "lg" | "xl")
}

/// A value that can only be a length: widths, spacings, thicknesses.
fn is_length_value(value: &str) -> bool {
    is_number(value) || is_fraction(value) || is_arbitrary(value) || is_css_var(value)
}

/// Distinguishes `text-sm` (font size) from `text-primary` (color).
fn is_size_value(value: &str) -> bool {
    let head = value.split('/').next().unwrap_or(value);
    if is_arbitrary(head) || is_css_var(head) {
        return !is_color_value(head);
    }
    head == "base" || is_tshirt_size(head) || is_number(head)
}

/// Distinguishes `shadow-lg` (size) from `shadow-accent/40` (color).
fn is_color_value(value: &str) -> bool {
    if let Some(inner) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        return inner.starts_with('#')
            || inner.starts_with("color:")
            || inner.starts_with("rgb")
            || inner.starts_with("hsl")
            || inner.starts_with("oklch")
            || inner.starts_with("oklab");
    }
    if let Some(inner) = value
        .strip_prefix('(')
        .and_then(|value| value.strip_suffix(')'))
    {
        return inner.starts_with("color:");
    }
    !(value == "none"
        || value == "inner"
        || value == "initial"
        || is_tshirt_size(value)
        || is_number(value))
}

/// Groups that a winning group also has to knock out, because they write to a
/// subset of the same CSS properties: `p-3` beats a previous `px-2`, `inset-0`
/// beats a previous `top-4`, and so on.
fn conflicting_groups(group: &str) -> &'static [&'static str] {
    match group {
        "overflow" => &["overflow-x", "overflow-y"],
        "overscroll" => &["overscroll-x", "overscroll-y"],
        "inset" => &[
            "inset-x", "inset-y", "start", "end", "top", "right", "bottom", "left",
        ],
        "inset-x" => &["right", "left"],
        "inset-y" => &["top", "bottom"],
        "flex" => &["basis", "grow", "shrink"],
        "gap" => &["gap-x", "gap-y"],
        "p" => &["px", "py", "ps", "pe", "pt", "pr", "pb", "pl"],
        "px" => &["pr", "pl"],
        "py" => &["pt", "pb"],
        "m" => &["mx", "my", "ms", "me", "mt", "mr", "mb", "ml"],
        "mx" => &["mr", "ml"],
        "my" => &["mt", "mb"],
        "size" => &["w", "h"],
        "font-size" => &["leading"],
        "fvn-normal" => &[
            "fvn-ordinal",
            "fvn-slashed-zero",
            "fvn-figure",
            "fvn-spacing",
            "fvn-fraction",
        ],
        "rounded" => &[
            "rounded-s",
            "rounded-e",
            "rounded-t",
            "rounded-r",
            "rounded-b",
            "rounded-l",
            "rounded-ss",
            "rounded-se",
            "rounded-ee",
            "rounded-es",
            "rounded-tl",
            "rounded-tr",
            "rounded-br",
            "rounded-bl",
        ],
        "rounded-s" => &["rounded-ss", "rounded-es"],
        "rounded-e" => &["rounded-se", "rounded-ee"],
        "rounded-t" => &["rounded-tl", "rounded-tr"],
        "rounded-r" => &["rounded-tr", "rounded-br"],
        "rounded-b" => &["rounded-br", "rounded-bl"],
        "rounded-l" => &["rounded-tl", "rounded-bl"],
        "border-spacing" => &["border-spacing-x", "border-spacing-y"],
        "border-w" => &[
            "border-w-s",
            "border-w-e",
            "border-w-t",
            "border-w-r",
            "border-w-b",
            "border-w-l",
            "border-w-x",
            "border-w-y",
        ],
        "border-w-x" => &["border-w-r", "border-w-l"],
        "border-w-y" => &["border-w-t", "border-w-b"],
        "border-color" => &[
            "border-color-s",
            "border-color-e",
            "border-color-t",
            "border-color-r",
            "border-color-b",
            "border-color-l",
            "border-color-x",
            "border-color-y",
        ],
        "border-color-x" => &["border-color-r", "border-color-l"],
        "border-color-y" => &["border-color-t", "border-color-b"],
        "scroll-m" => &[
            "scroll-mx",
            "scroll-my",
            "scroll-ms",
            "scroll-me",
            "scroll-mt",
            "scroll-mr",
            "scroll-mb",
            "scroll-ml",
        ],
        "scroll-mx" => &["scroll-mr", "scroll-ml"],
        "scroll-my" => &["scroll-mt", "scroll-mb"],
        "scroll-p" => &[
            "scroll-px",
            "scroll-py",
            "scroll-ps",
            "scroll-pe",
            "scroll-pt",
            "scroll-pr",
            "scroll-pb",
            "scroll-pl",
        ],
        "scroll-px" => &["scroll-pr", "scroll-pl"],
        "scroll-py" => &["scroll-pt", "scroll-pb"],
        "touch" => &["touch-x", "touch-y", "touch-pz"],
        "translate" => &["translate-x", "translate-y", "translate-z"],
        "scale" => &["scale-x", "scale-y", "scale-z"],
        "rotate" => &["rotate-x", "rotate-y", "rotate-z"],
        "skew" => &["skew-x", "skew-y"],
        _ => &[],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // -- clsx half ---------------------------------------------------------
    #[test]
    fn skips_falsy_inputs_and_joins_the_rest() {
        let none: Option<&str> = None;
        assert_eq!(
            cn!(
                "flex",
                none,
                Some("items-center"),
                (true, "gap-2"),
                (false, "hidden"),
                vec!["text-sm"],
                ["font-medium"],
                String::from("  rounded-full  "),
                "",
            ),
            "flex items-center gap-2 text-sm font-medium rounded-full"
        );
    }

    #[test]
    fn accepts_owned_and_borrowed_props() {
        let class: Option<String> = Some("bg-accent".to_string());
        assert_eq!(cn!("bg-secondary", &class), "bg-accent");
        assert_eq!(cn!("bg-secondary", class.as_deref()), "bg-accent");
    }

    #[test]
    fn builder_exposes_the_unmerged_list() {
        let mut builder = ClassBuilder::new();
        builder.push("px-2").push("p-3");
        assert_eq!(builder.as_str(), "px-2 p-3");
        assert_eq!(builder.finish(), "p-3");
    }

    // -- tailwind-merge half -----------------------------------------------
    #[test]
    fn last_utility_of_a_group_wins() {
        assert_eq!(cn!("px-2 py-1", "p-3"), "p-3");
        assert_eq!(cn!("px-2", "px-3"), "px-3");
        assert_eq!(cn!("text-red-500", "text-blue-500"), "text-blue-500");
        assert_eq!(cn!("bg-accent/10", "bg-secondary"), "bg-secondary");
        assert_eq!(cn!("flex", "hidden"), "hidden");
        assert_eq!(cn!("-mt-2", "mt-4"), "mt-4");
        assert_eq!(cn!("font-medium", "font-bold"), "font-bold");
    }

    #[test]
    fn a_narrower_utility_after_a_wider_one_is_kept() {
        assert_eq!(cn!("p-4", "px-2"), "p-4 px-2");
        assert_eq!(cn!("size-4", "w-6"), "size-4 w-6");
        assert_eq!(
            cn!("rounded-full", "rounded-t-none"),
            "rounded-full rounded-t-none"
        );
        assert_eq!(cn!("gap-2", "gap-x-4"), "gap-2 gap-x-4");
    }

    #[test]
    fn a_wider_utility_knocks_out_the_narrower_ones_before_it() {
        assert_eq!(cn!("w-6 h-6", "size-4"), "size-4");
        assert_eq!(cn!("border-t-2", "border-2"), "border-2");
        assert_eq!(cn!("rounded-t-none", "rounded-full"), "rounded-full");
        assert_eq!(cn!("gap-x-4", "gap-2"), "gap-2");
        assert_eq!(cn!("top-0 left-0", "inset-4"), "inset-4");
        assert_eq!(cn!("leading-5", "text-sm"), "text-sm");
    }

    #[test]
    fn utilities_of_different_groups_coexist() {
        assert_eq!(cn!("text-sm", "text-red-500"), "text-sm text-red-500");
        assert_eq!(cn!("flex", "flex-row"), "flex flex-row");
        assert_eq!(cn!("font-sans", "font-bold"), "font-sans font-bold");
        assert_eq!(
            cn!("border-2 border-red-500", "border-blue-500"),
            "border-2 border-blue-500"
        );
        assert_eq!(cn!("ring-2", "ring-accent/40"), "ring-2 ring-accent/40");
        assert_eq!(cn!("shadow-lg", "shadow-accent"), "shadow-lg shadow-accent");
    }

    #[test]
    fn conflicts_are_scoped_to_the_variants() {
        assert_eq!(
            cn!("hover:bg-red-500", "bg-blue-500"),
            "hover:bg-red-500 bg-blue-500"
        );
        assert_eq!(cn!("hover:p-2", "p-4"), "hover:p-2 p-4");
        assert_eq!(cn!("hover:p-2", "hover:p-4"), "hover:p-4");
        assert_eq!(cn!("md:hover:p-2", "hover:md:p-4"), "hover:md:p-4");
        assert_eq!(
            cn!("disabled:opacity-50", "disabled:opacity-70"),
            "disabled:opacity-70"
        );
    }

    #[test]
    fn important_utilities_form_their_own_scope() {
        assert_eq!(cn!("!p-2", "p-4"), "!p-2 p-4");
        assert_eq!(cn!("p-2!", "p-4!"), "p-4!");
        assert_eq!(cn!("!p-2", "!p-4"), "!p-4");
    }

    #[test]
    fn arbitrary_values_variants_and_properties() {
        assert_eq!(cn!("w-[100px]", "w-4"), "w-4");
        assert_eq!(cn!("w-4", "w-[100px]"), "w-[100px]");
        assert_eq!(cn!("[&_svg]:size-4", "[&_svg]:size-3"), "[&_svg]:size-3");
        assert_eq!(
            cn!(
                "[&_svg:not([class*='size-'])]:size-4",
                "[&_svg:not([class*='size-'])]:size-3"
            ),
            "[&_svg:not([class*='size-'])]:size-3"
        );
        assert_eq!(cn!("has-[>svg]:px-3", "has-[>svg]:px-4"), "has-[>svg]:px-4");
        assert_eq!(
            cn!("[mask-type:luminance]", "[mask-type:alpha]"),
            "[mask-type:alpha]"
        );
        assert_eq!(cn!("bg-[#fff]", "bg-accent"), "bg-accent");
    }

    #[test]
    fn unknown_utilities_survive_but_do_not_duplicate() {
        assert_eq!(cn!("dx-button", "dx-primary"), "dx-button dx-primary");
        assert_eq!(cn!("dx-button", "dx-button"), "dx-button");
        assert_eq!(cn!("flex", "flex"), "flex");
    }

    #[test]
    fn merges_a_real_component_class_list() {
        let base = "inline-flex shrink-0 cursor-pointer items-center justify-center gap-2 \
                    rounded-full p-0 text-sm leading-5 font-medium transition-colors duration-100 \
                    [&_svg:not([class*='size-'])]:size-4 focus-visible:ring-2 \
                    focus-visible:ring-accent/40 disabled:opacity-50";

        assert_eq!(
            cn!(base, "h-9 px-4 py-2", "rounded-md", "text-base"),
            "inline-flex shrink-0 cursor-pointer items-center justify-center gap-2 p-0 \
             font-medium transition-colors duration-100 \
             [&_svg:not([class*='size-'])]:size-4 focus-visible:ring-2 \
             focus-visible:ring-accent/40 disabled:opacity-50 h-9 px-4 py-2 rounded-md text-base"
        );
    }

    #[test]
    fn tw_merge_is_usable_on_its_own() {
        assert_eq!(tw_merge("px-2  py-1\np-3"), "p-3");
        assert_eq!(tw_merge(""), "");
    }
}
