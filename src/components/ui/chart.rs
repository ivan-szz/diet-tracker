use dioxus::prelude::*;

/// Colours handed out, in order, to the series that do not pick one themselves.
const CHART_COLORS: [&str; 8] = [
    "#C67139", "#56633F", "#38616B", "#8A472B", "#68496F", "#356052", "#70404A", "#46567A",
];

/// Gaps between horizontal grid lines. Every scale is forced onto exactly this
/// many, which is what lets their labels share the rows of the left column.
const AXIS_INTERVALS: usize = 5;

/// The multipliers a grid step is allowed to take, times a power of ten.
const NICE_STEPS: [f64; 4] = [1.0, 2.0, 2.5, 5.0];

/// How many day labels the bottom row shows at its widest. Odd on purpose:
/// dropping every other one still keeps the first and the last day.
const MAX_DAY_LABELS: usize = 7;

/// Below this many labels the bottom row is sparse enough to never thin out.
const CROWDED_LABELS: usize = 5;

/// A single line of the chart.
///
/// One entry in [`ChartProps::series`] is one line, so adding a line to a chart
/// is pushing one more `ChartSeries` into that list, nothing else.
///
/// The `unit` also picks the scale: series that share a unit share a y axis, and
/// every other unit gets its own, so calories and kilograms can sit in the same
/// chart without squashing each other.
///
/// ```ignore
/// ChartSeries::new("Calorie assunte", " kcal", vec![1450.0, 1720.0])
/// ChartSeries::new("Obiettivo", " kcal", vec![2000.0, 2000.0]).with_color("#6B665E").dashed()
/// ChartSeries::new("Peso", " kg", vec![95.5, 95.4])
/// ```
#[derive(Clone, PartialEq, Debug)]
pub struct ChartSeries {
    /// Shown in the legend and in the tooltip.
    pub name: String,

    /// Appended to every value of this series, e.g. `" kg"`. Doubles as the
    /// identity of the scale the series is drawn against.
    pub unit: String,

    /// One value per day, in the same order as [`ChartProps::days`].
    pub values: Vec<f64>,

    /// Overrides the colour taken from the default palette.
    pub color: Option<String>,

    /// Overrides the decimals, which otherwise follow the data.
    pub decimals: Option<usize>,

    /// Draws the line dashed, for a reference such as a target.
    pub dashed: bool,
}

impl ChartSeries {
    pub fn new(name: impl Into<String>, unit: impl Into<String>, values: Vec<f64>) -> Self {
        Self {
            name: name.into(),
            unit: unit.into(),
            values,
            color: None,
            decimals: None,
            dashed: false,
        }
    }

    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn with_decimals(mut self, decimals: usize) -> Self {
        self.decimals = Some(decimals);
        self
    }

    pub fn dashed(mut self) -> Self {
        self.dashed = true;
        self
    }

    /// Whole numbers are written whole, anything else keeps one decimal.
    fn resolved_decimals(&self) -> usize {
        self.decimals.unwrap_or_else(|| {
            let whole = self
                .values
                .iter()
                .all(|value| (value - value.round()).abs() < 1e-9);
            if whole {
                0
            } else {
                1
            }
        })
    }
}

/// The props for the [`Chart`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ChartProps {
    /// One line per entry.
    pub series: Vec<ChartSeries>,

    /// The bottom row: one label per day. Every series is read against it, so a
    /// series shorter than this list simply stops early.
    pub days: Vec<String>,

    /// Shows the colour/name legend above the plot.
    #[props(default = true)]
    pub legend: bool,

    /// Additional attributes to extend the chart element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// One y axis: the range every series with this unit is drawn against.
struct Scale {
    unit: String,
    /// Taken from the first series on the scale, so its labels are recognisable.
    color: String,
    lo: f64,
    hi: f64,
    step: f64,
    decimals: usize,
}

/// A line resolved into everything the markup needs.
struct Line {
    name: String,
    color: String,
    dashed: bool,
    /// The `d` of the smoothed curve, in the 0..100 space of the view box.
    path: String,
    /// One `(x, y)` per day, in the same 0..100 space, for the hover markers.
    points: Vec<(f64, f64)>,
}

/// One label of the left column: a value on one of the scales.
#[derive(Clone)]
struct AxisCell {
    label: String,
    color: String,
}

/// A horizontal grid line and the label every scale puts on it.
struct AxisRow {
    y: f64,
    cells: Vec<AxisCell>,
}

/// A vertical grid line and its label in the bottom row.
struct DayTick {
    x: f64,
    label: String,
    /// Keeps the first and the last label inside the plot.
    anchor: &'static str,
    /// Drops the label out of a narrow chart, where the row would collide.
    visibility: &'static str,
}

/// What the hovered day looks like for one line.
struct HoverPoint {
    x: f64,
    y: f64,
    color: String,
    name: String,
    value: String,
}

/// A responsive multi line chart with a day axis.
///
/// The curves are drawn in a `0 0 100 100` view box stretched to the element, so
/// they stay sharp at any width without measuring the DOM. Everything that must
/// not be stretched (grid, markers, tooltip, labels) is plain HTML positioned in
/// percentages on top of it.
#[component]
pub fn Chart(props: ChartProps) -> Element {
    let mut hovered = use_signal(|| None::<usize>);

    let days = props.days.len();
    if days == 0 || props.series.is_empty() {
        return rsx! {
            p { class: "text-sm text-primary-light", "Nessun dato da mostrare" }
        };
    }

    let scales = build_scales(&props.series);
    let x_at = |day: usize| {
        if days <= 1 {
            50.0
        } else {
            day as f64 * 100.0 / (days - 1) as f64
        }
    };

    let lines: Vec<Line> = props
        .series
        .iter()
        .enumerate()
        .map(|(index, series)| {
            let scale = scale_of(&scales, &series.unit);
            let points: Vec<(f64, f64)> = series
                .values
                .iter()
                .take(days)
                .enumerate()
                .map(|(day, &value)| (x_at(day), scale.y_of(value)))
                .collect();

            Line {
                name: series.name.clone(),
                color: series
                    .color
                    .clone()
                    .unwrap_or_else(|| CHART_COLORS[index % CHART_COLORS.len()].to_string()),
                dashed: series.dashed,
                path: smooth_path(&points),
                points,
            }
        })
        .collect();

    // Every scale reads the same rows, top to bottom, so the labels line up.
    let axis_rows: Vec<AxisRow> = (0..=AXIS_INTERVALS)
        .map(|row| AxisRow {
            y: row as f64 * 100.0 / AXIS_INTERVALS as f64,
            cells: scales.iter().map(|scale| scale.cell(row)).collect(),
        })
        .collect();
    // Absolutely positioned rows cannot give the column its width, so the widest
    // label of every scale is laid out once, invisibly, to do it for them.
    let axis_sizer: Vec<AxisCell> = scales.iter().map(Scale::widest_cell).collect();

    // Bottom row: labels spread evenly, always keeping the first and the last day.
    let label_count = MAX_DAY_LABELS.min(days).max(1);
    let mut wanted: Vec<usize> = if label_count == 1 {
        vec![0]
    } else {
        (0..label_count)
            .map(|k| ((k * (days - 1)) as f64 / (label_count - 1) as f64).round() as usize)
            .collect()
    };
    wanted.dedup();
    let crowded = wanted.len() >= CROWDED_LABELS;
    let day_ticks: Vec<DayTick> = wanted
        .iter()
        .enumerate()
        .map(|(position, &day)| DayTick {
            x: x_at(day),
            label: props.days[day].clone(),
            anchor: label_anchor(day, days),
            visibility: if crowded && position % 2 == 1 {
                "hidden @lg:block"
            } else {
                ""
            },
        })
        .collect();

    let hover = hovered();
    let hover_points: Vec<HoverPoint> = match hover {
        Some(day) => lines
            .iter()
            .zip(props.series.iter())
            .filter_map(|(line, series)| {
                let &(x, y) = line.points.get(day)?;
                Some(HoverPoint {
                    x,
                    y,
                    color: line.color.clone(),
                    name: line.name.clone(),
                    value: format_value(
                        series.values[day],
                        series.resolved_decimals(),
                        &series.unit,
                    ),
                })
            })
            .collect(),
        None => Vec::new(),
    };

    let showing = !hover_points.is_empty();
    let hover_x = hover.map(|day| x_at(day)).unwrap_or_default();
    let hover_day = hover
        .and_then(|day| props.days.get(day).cloned())
        .unwrap_or_default();
    // Past the middle the tooltip would run off the card, so it flips to the left.
    let tooltip_side = if hover_x > 60.0 {
        "-translate-x-full -ml-3"
    } else {
        "ml-3"
    };

    // Transparent hit areas, one per day, each covering the half step around its
    // point so that the day nearest to the cursor always wins.
    let hit_areas: Vec<Element> = (0..days)
        .map(|day| {
            let left = hit_left(day, days);
            let width = hit_width(day, days);
            rsx! {
                div {
                    class: "absolute inset-y-0",
                    left: "{left:.3}%",
                    width: "{width:.3}%",
                    onmouseenter: move |_| hovered.set(Some(day)),
                }
            }
        })
        .collect();

    rsx! {
        div {
            class: "@container w-full",
            ..props.attributes,

            if props.legend {
                div {
                    class: "mb-5 flex flex-wrap items-center gap-x-5 gap-y-2",
                    for line in lines.iter() {
                        div {
                            class: "flex items-center gap-2",
                            span {
                                class: "size-2.5 shrink-0 rounded-full",
                                background_color: "{line.color}",
                                aria_hidden: "true",
                            }
                            p { class: "text-xs font-semibold text-primary-light", "{line.name}" }
                        }
                    }
                }
            }

            div {
                class: "grid grid-cols-[auto_1fr] gap-x-3",

                // Y axis: one row per grid line, one label per scale in each row.
                div {
                    class: "relative",
                    div {
                        class: "invisible flex items-baseline justify-end gap-2 text-[11px] whitespace-nowrap tabular-nums",
                        aria_hidden: "true",
                        for cell in axis_sizer.iter() {
                            p { "{cell.label}" }
                        }
                    }
                    for row in axis_rows.iter() {
                        div {
                            class: "absolute right-0 flex -translate-y-1/2 items-baseline justify-end gap-2 text-[11px] whitespace-nowrap tabular-nums",
                            top: "{row.y:.3}%",
                            for cell in row.cells.iter() {
                                p { color: "{cell.color}", "{cell.label}" }
                            }
                        }
                    }
                }

                // Plot area.
                div {
                    class: "relative h-56 w-full",
                    onmouseleave: move |_| hovered.set(None),

                    for row in axis_rows.iter() {
                        div {
                            class: "absolute inset-x-0 border-t border-dashed border-primary/15",
                            top: "{row.y:.3}%",
                        }
                    }
                    for tick in day_ticks.iter() {
                        div {
                            class: "absolute inset-y-0 border-l border-dashed border-primary/10 {tick.visibility}",
                            left: "{tick.x:.3}%",
                        }
                    }

                    svg {
                        class: "pointer-events-none absolute inset-0 h-full w-full overflow-visible",
                        view_box: "0 0 100 100",
                        preserve_aspect_ratio: "none",
                        for line in lines.iter() {
                            path {
                                d: "{line.path}",
                                fill: "none",
                                stroke: "{line.color}",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_dasharray: if line.dashed { "5 5" },
                                "vector-effect": "non-scaling-stroke",
                            }
                        }
                    }

                    if showing {
                        div {
                            class: "pointer-events-none absolute inset-y-0 w-px bg-primary/30",
                            left: "{hover_x:.3}%",
                        }
                        for point in hover_points.iter() {
                            div {
                                class: "pointer-events-none absolute size-3 -translate-x-1/2 -translate-y-1/2 rounded-full border-2 border-background-dark",
                                left: "{point.x:.3}%",
                                top: "{point.y:.3}%",
                                background_color: "{point.color}",
                            }
                        }
                        div {
                            class: "pointer-events-none absolute top-2 z-20 w-max rounded-2xl bg-primary/95 px-3 py-2 shadow-lg {tooltip_side}",
                            left: "{hover_x:.3}%",
                            p {
                                class: "mb-1.5 text-[10px] font-semibold tracking-wide text-background/60 uppercase",
                                "{hover_day}"
                            }
                            for point in hover_points.iter() {
                                div {
                                    class: "flex items-center justify-between gap-5",
                                    div {
                                        class: "flex items-center gap-1.5",
                                        span {
                                            class: "size-2 shrink-0 rounded-full",
                                            background_color: "{point.color}",
                                            aria_hidden: "true",
                                        }
                                        p { class: "text-xs text-background/80", "{point.name}" }
                                    }
                                    p { class: "text-xs font-semibold tabular-nums text-background", "{point.value}" }
                                }
                            }
                        }
                    }

                    {hit_areas.into_iter()}
                }

                // Bottom row: the days.
                div {}
                div {
                    class: "relative mt-3 h-4",
                    for tick in day_ticks.iter() {
                        p {
                            class: "absolute text-[11px] whitespace-nowrap text-primary-light {tick.anchor} {tick.visibility}",
                            left: "{tick.x:.3}%",
                            "{tick.label}"
                        }
                    }
                }
            }
        }
    }
}

impl Scale {
    /// Where `value` sits in the plot, as a percentage from the top.
    fn y_of(&self, value: f64) -> f64 {
        100.0 - (value - self.lo) / (self.hi - self.lo) * 100.0
    }

    /// The label this scale puts on grid line `row`, counted from the top.
    fn cell(&self, row: usize) -> AxisCell {
        let value = self.hi - self.step * row as f64;
        AxisCell {
            label: format_value(value, self.decimals, &self.unit),
            color: self.color.clone(),
        }
    }

    /// The longest label of the scale, which is the one that sizes the column.
    fn widest_cell(&self) -> AxisCell {
        (0..=AXIS_INTERVALS)
            .map(|row| self.cell(row))
            .max_by_key(|cell| cell.label.chars().count())
            .unwrap_or_else(|| AxisCell {
                label: self.unit.clone(),
                color: self.color.clone(),
            })
    }
}

/// Groups the series by unit, in order of first appearance, and gives every
/// group the range that fits its data in [`AXIS_INTERVALS`] rows.
fn build_scales(series: &[ChartSeries]) -> Vec<Scale> {
    let mut scales: Vec<Scale> = Vec::new();

    for (index, line) in series.iter().enumerate() {
        if scales.iter().any(|scale| scale.unit == line.unit) {
            continue;
        }

        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;
        for peer in series.iter().filter(|peer| peer.unit == line.unit) {
            for value in peer.values.iter().copied().filter(|value| value.is_finite()) {
                min = min.min(value);
                max = max.max(value);
            }
        }

        if !min.is_finite() || !max.is_finite() {
            min = 0.0;
            max = 1.0;
        }
        // A flat line would collapse the axis onto a single value.
        if (max - min).abs() < f64::EPSILON {
            min -= 0.5;
            max += 0.5;
        }

        let (lo, hi, step) = axis_range(min, max);

        scales.push(Scale {
            unit: line.unit.clone(),
            color: line
                .color
                .clone()
                .unwrap_or_else(|| CHART_COLORS[index % CHART_COLORS.len()].to_string()),
            lo,
            hi,
            step,
            decimals: tick_decimals(step),
        });
    }

    scales
}

fn scale_of<'a>(scales: &'a [Scale], unit: &str) -> &'a Scale {
    scales
        .iter()
        .find(|scale| scale.unit == unit)
        .unwrap_or(&scales[0])
}

/// Rounds `min`/`max` out to exactly [`AXIS_INTERVALS`] rows of a round step.
///
/// The step is the smallest round one the data fits in; the rows left over are
/// handed to whichever side of the data has less room, which keeps the curves
/// centred instead of pinned to an edge.
fn axis_range(min: f64, max: f64) -> (f64, f64, f64) {
    let step = round_step(min, max);
    let mut lowest = (min / step).floor();
    let mut highest = (max / step).ceil();

    let spare = AXIS_INTERVALS as f64 - (highest - lowest);
    for _ in 0..spare.max(0.0) as usize {
        if min - lowest * step <= highest * step - max {
            lowest -= 1.0;
        } else {
            highest += 1.0;
        }
    }

    (lowest * step, highest * step, step)
}

/// The smallest round step that fits `min..max` in [`AXIS_INTERVALS`] rows.
fn round_step(min: f64, max: f64) -> f64 {
    let span = (max - min).abs();
    if span <= 0.0 || !span.is_finite() {
        return 1.0;
    }

    // Start a decade below the average row, so the first fit is the smallest one.
    let mut magnitude = 10f64.powf((span / AXIS_INTERVALS as f64).log10().floor() - 1.0);
    for _ in 0..8 {
        for multiplier in NICE_STEPS {
            let step = multiplier * magnitude;
            if (max / step).ceil() - (min / step).floor() <= AXIS_INTERVALS as f64 {
                return step;
            }
        }
        magnitude *= 10.0;
    }

    span / AXIS_INTERVALS as f64
}

/// Round steps are short numbers, so the step alone says how many decimals its
/// labels need: `250` needs none, `2.5` needs one.
fn tick_decimals(step: f64) -> usize {
    let mut decimals = 0;
    let mut scaled = step;
    while decimals < 4 && (scaled - scaled.round()).abs() > 1e-9 {
        scaled *= 10.0;
        decimals += 1;
    }
    decimals
}

fn format_value(value: f64, decimals: usize, unit: &str) -> String {
    format!("{:.*}{}", decimals, value, unit)
}

/// Left edge of the hit area of `day`, as a percentage of the plot.
fn hit_left(day: usize, days: usize) -> f64 {
    if days <= 1 {
        return 0.0;
    }
    let half = 50.0 / (days - 1) as f64;
    (day as f64 * 2.0 * half - half).max(0.0)
}

/// Width of the hit area of `day`: a full step, halved on the two edges.
fn hit_width(day: usize, days: usize) -> f64 {
    if days <= 1 {
        return 100.0;
    }
    let half = 50.0 / (days - 1) as f64;
    if day == 0 || day + 1 == days {
        half
    } else {
        half * 2.0
    }
}

/// Centres a day label on its grid line, except on the two edges where that
/// would push it out of the plot.
fn label_anchor(day: usize, days: usize) -> &'static str {
    if day == 0 {
        "translate-x-0"
    } else if day + 1 == days {
        "-translate-x-full"
    } else {
        "-translate-x-1/2"
    }
}

/// Smooths the points into a cubic path.
///
/// The tangents come from the Fritsch–Carlson filter, which keeps the curve
/// monotone between the points: a line that touches zero never dips under it,
/// the way a plain Catmull-Rom spline would.
fn smooth_path(points: &[(f64, f64)]) -> String {
    let count = points.len();
    if count == 0 {
        return String::new();
    }
    if count == 1 {
        let (x, y) = points[0];
        return format!("M {x:.3} {y:.3} L {x:.3} {y:.3}");
    }

    let dx = points[1].0 - points[0].0;
    let slopes: Vec<f64> = (0..count - 1)
        .map(|i| (points[i + 1].1 - points[i].1) / dx)
        .collect();

    let mut tangents = vec![0.0; count];
    tangents[0] = slopes[0];
    tangents[count - 1] = slopes[count - 2];
    for i in 1..count - 1 {
        let (before, after) = (slopes[i - 1], slopes[i]);
        tangents[i] = if before * after <= 0.0 {
            // A turning point: flatten it so the curve cannot overshoot.
            0.0
        } else {
            let average = (before + after) / 2.0;
            let limit = 3.0 * before.abs().min(after.abs());
            average.signum() * average.abs().min(limit)
        };
    }

    let mut path = format!("M {:.3} {:.3}", points[0].0, points[0].1);
    for i in 0..count - 1 {
        let (x0, y0) = points[i];
        let (x1, y1) = points[i + 1];
        path.push_str(&format!(
            " C {:.3} {:.3}, {:.3} {:.3}, {:.3} {:.3}",
            x0 + dx / 3.0,
            y0 + tangents[i] * dx / 3.0,
            x1 - dx / 3.0,
            y1 - tangents[i + 1] * dx / 3.0,
            x1,
            y1
        ));
    }
    path
}
