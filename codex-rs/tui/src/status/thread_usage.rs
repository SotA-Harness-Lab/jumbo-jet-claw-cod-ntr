//! Live-updating, thread-scoped billing details embedded in `/status` history cards.

use super::format::FieldFormatter;
use super::format::push_label;
use super::helpers::format_tokens_compact;
use crate::wrapping::RtOptions;
use crate::wrapping::word_wrap_lines;
use codex_app_server_protocol::ThreadUsage;
use codex_app_server_protocol::ThreadUsageBreakdownGroup;
use codex_protocol::account::PlanType;
use ratatui::prelude::Line;
use ratatui::style::Stylize;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::sync::RwLock;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

const REASONING_ORDER: [&str; 8] = [
    "None",
    "Minimal",
    "Light",
    "Medium",
    "High",
    "Extra High",
    "Max",
    "Ultra",
];
const SPEED_ORDER: [&str; 3] = ["Fast mode", "Ultrafast", "Standard"];

fn models_label() -> &'static str {
    crate::i18n::tr!("thread-usage-models-label")
}

fn reasoning_label() -> &'static str {
    crate::i18n::tr!("thread-usage-reasoning-label")
}

fn speed_label() -> &'static str {
    crate::i18n::tr!("thread-usage-speed-label")
}

fn billed_tokens_label() -> &'static str {
    crate::i18n::tr!("thread-usage-billed-tokens-label")
}

fn thread_usage_label() -> &'static str {
    crate::i18n::tr!("thread-usage-label")
}

/// The display form of a breakdown value. Values are grouped and ordered by their English
/// names; only the text shown is translated.
fn localized_group_name(dimension: BreakdownDimension, name: &str) -> String {
    let text = match (dimension, name) {
        (BreakdownDimension::Model, "Other")
        | (BreakdownDimension::Reasoning, "Other")
        | (BreakdownDimension::Speed, "Other") => crate::i18n::tr!("thread-usage-other"),
        (BreakdownDimension::Reasoning, "None") => crate::i18n::tr!("thread-usage-reasoning-none"),
        (BreakdownDimension::Reasoning, "Minimal") => {
            crate::i18n::tr!("thread-usage-reasoning-minimal")
        }
        (BreakdownDimension::Reasoning, "Light") => {
            crate::i18n::tr!("thread-usage-reasoning-light")
        }
        (BreakdownDimension::Reasoning, "Medium") => {
            crate::i18n::tr!("thread-usage-reasoning-medium")
        }
        (BreakdownDimension::Reasoning, "High") => crate::i18n::tr!("thread-usage-reasoning-high"),
        (BreakdownDimension::Reasoning, "Extra High") => {
            crate::i18n::tr!("thread-usage-reasoning-extra-high")
        }
        (BreakdownDimension::Reasoning, "Max") => crate::i18n::tr!("thread-usage-reasoning-max"),
        (BreakdownDimension::Reasoning, "Ultra") => {
            crate::i18n::tr!("thread-usage-reasoning-ultra")
        }
        (BreakdownDimension::Speed, "Fast mode") => crate::i18n::tr!("thread-usage-speed-fast"),
        (BreakdownDimension::Speed, "Ultrafast") => {
            crate::i18n::tr!("thread-usage-speed-ultrafast")
        }
        (BreakdownDimension::Speed, "Standard") => {
            crate::i18n::tr!("thread-usage-speed-standard")
        }
        _ => return name.to_string(),
    };
    text.to_string()
}

/// ENT26 is billed directly in dollars; missing native usage is not a zero estimate.
pub(crate) fn thread_cost_micros(usage: &ThreadUsage, plan_type: Option<PlanType>) -> Option<i64> {
    if plan_type == Some(PlanType::Ent26) {
        usage.native_usage_usd_micros
    } else {
        usage.estimated_usage_usd_micros
    }
}

/// Shared state updates a committed `/status` card when its asynchronous estimate arrives.
#[derive(Debug, Default)]
pub(crate) struct StatusThreadUsage {
    billing: RwLock<CardBilling>,
    reserve_label_width: AtomicBool,
}

#[derive(Debug, Default)]
struct CardBilling {
    estimate: Option<ThreadUsage>,
    plan: Option<PlanType>,
}

impl StatusThreadUsage {
    pub(crate) fn reserve_label_width(&self) {
        self.reserve_label_width
            .store(/*val*/ true, Ordering::Relaxed);
    }

    pub(crate) fn set_estimate(&self, estimate: Option<ThreadUsage>, plan: Option<PlanType>) {
        #[expect(clippy::expect_used)]
        let mut billing = self
            .billing
            .write()
            .expect("status history thread-usage state poisoned");
        *billing = CardBilling { estimate, plan };
    }

    pub(crate) fn push_labels(&self, labels: &mut Vec<String>, seen: &mut BTreeSet<String>) {
        // Keep existing card rows stable when asynchronous billing details arrive. Reserving a
        // formatter label does not emit a loading placeholder or an empty billing row.
        if self.reserve_label_width.load(Ordering::Relaxed) {
            push_label(labels, seen, billed_tokens_label());
        }
        #[expect(clippy::expect_used)]
        let billing = self
            .billing
            .read()
            .expect("status history thread-usage state poisoned");
        let Some(estimate) = billing.estimate.as_ref() else {
            return;
        };
        push_label(labels, seen, thread_usage_label());
        if !estimate.groups.is_empty() {
            push_label(labels, seen, models_label());
            push_label(labels, seen, reasoning_label());
            push_label(labels, seen, speed_label());
            if estimate
                .groups
                .iter()
                .any(|group| group.input_tokens.is_some() || group.output_tokens.is_some())
            {
                push_label(labels, seen, billed_tokens_label());
            }
        }
    }

    pub(crate) fn lines(
        &self,
        formatter: &FieldFormatter,
        value_width: usize,
    ) -> Vec<Line<'static>> {
        #[expect(clippy::expect_used)]
        let billing = self
            .billing
            .read()
            .expect("status history thread-usage state poisoned");
        let Some(estimate) = billing.estimate.as_ref() else {
            return Vec::new();
        };

        if billing.plan == Some(PlanType::Ent26) {
            // Credit-weighted breakdowns do not describe native dollar usage.
            return thread_cost_micros(estimate, billing.plan)
                .and_then(format_estimated_usd_micros)
                .map(|cost| vec![formatter.line(thread_usage_label(), vec![cost.into()])])
                .unwrap_or_default();
        }

        let credits_micros = estimate.estimated_usage_credits_micros;
        let mut usage = vec![
            crate::i18n::tr!(
                "thread-usage-credits",
                credits = format_credit_micros(credits_micros)
            )
            .into(),
        ];
        if let Some(cost) = estimate
            .estimated_usage_usd_micros
            .and_then(format_estimated_usd_micros)
        {
            usage.push(crate::i18n::tr!("thread-usage-separator").dim());
            usage.push(cost.into());
        }
        let mut lines = vec![formatter.line(thread_usage_label(), usage)];

        for (label, dimension) in [
            (models_label(), BreakdownDimension::Model),
            (reasoning_label(), BreakdownDimension::Reasoning),
            (speed_label(), BreakdownDimension::Speed),
        ] {
            let Some(value) = grouped_usage(&estimate.groups, dimension) else {
                continue;
            };
            let mut wrapped = word_wrap_lines(
                [Line::from(value)],
                RtOptions::new(value_width.max(/*other*/ 1)),
            )
            .into_iter();
            if let Some(first) = wrapped.next() {
                lines.push(formatter.line(label, first.spans));
                lines.extend(wrapped.map(|line| formatter.continuation(line.spans)));
            }
        }

        let sum_tokens = |count: fn(&ThreadUsageBreakdownGroup) -> Option<i64>| {
            estimate
                .groups
                .iter()
                .try_fold(/*init*/ 0_i64, |total, group| {
                    count(group).map(|tokens| total.saturating_add(tokens))
                })
        };
        let input_tokens = sum_tokens(|group| group.input_tokens);
        let cached_tokens = sum_tokens(|group| group.cached_input_tokens);
        let output_tokens = sum_tokens(|group| group.output_tokens);
        if !estimate.groups.is_empty() && (input_tokens.is_some() || output_tokens.is_some()) {
            let mut tokens = Vec::new();
            if let Some(input_tokens) = input_tokens {
                tokens.push(crate::i18n::tr!(
                    "thread-usage-tokens-input",
                    tokens = format_tokens_compact(input_tokens)
                ));
                if let Some(cached_tokens) = cached_tokens {
                    tokens.push(crate::i18n::tr!(
                        "thread-usage-tokens-cached",
                        tokens = format_tokens_compact(cached_tokens)
                    ));
                }
            }
            if let Some(output_tokens) = output_tokens {
                if !tokens.is_empty() {
                    tokens.push("+".to_string());
                }
                tokens.push(crate::i18n::tr!(
                    "thread-usage-tokens-output",
                    tokens = format_tokens_compact(output_tokens)
                ));
            }
            lines.push(formatter.line(billed_tokens_label(), vec![tokens.join(" ").into()]));
        }

        lines
    }
}

#[derive(Clone, Copy)]
enum BreakdownDimension {
    Model,
    Reasoning,
    Speed,
}

fn grouped_usage(
    groups: &[ThreadUsageBreakdownGroup],
    dimension: BreakdownDimension,
) -> Option<String> {
    let mut totals = BTreeMap::<String, i64>::new();
    for group in groups {
        if group.estimated_usage_credits_micros < 0 {
            continue;
        }
        let value = match dimension {
            BreakdownDimension::Model => group
                .model
                .as_deref()
                .map(format_model_display_name)
                .unwrap_or_else(|| "Other".to_string()),
            BreakdownDimension::Reasoning => match group.reasoning_effort.as_deref() {
                Some("none") => "None",
                Some("minimal") => "Minimal",
                Some("low") => "Light",
                Some("medium") => "Medium",
                Some("high") => "High",
                Some("xhigh") => "Extra High",
                Some("max") => "Max",
                Some("ultra") => "Ultra",
                Some(other) => other,
                None => "Other",
            }
            .to_string(),
            BreakdownDimension::Speed => match group.speed.as_deref() {
                Some("fast") => "Fast mode",
                Some("ultrafast") => "Ultrafast",
                Some("standard") => "Standard",
                Some(other) => other,
                None => "Other",
            }
            .to_string(),
        };
        let total = totals.entry(value).or_default();
        *total = total.saturating_add(group.estimated_usage_credits_micros);
    }
    if totals.is_empty() {
        return None;
    }

    let total_credits_micros =
        totals.values().fold(
            /*init*/ 0_i64,
            |total, credits| total.saturating_add(*credits),
        );
    let mut totals = totals.into_iter().collect::<Vec<_>>();
    totals.sort_by(
        |(left_name, left_credits), (right_name, right_credits)| match dimension {
            BreakdownDimension::Model => right_credits
                .cmp(left_credits)
                .then_with(|| left_name.cmp(right_name)),
            BreakdownDimension::Reasoning => REASONING_ORDER
                .iter()
                .position(|value| value == left_name)
                .unwrap_or(usize::MAX)
                .cmp(
                    &REASONING_ORDER
                        .iter()
                        .position(|value| value == right_name)
                        .unwrap_or(usize::MAX),
                )
                .then_with(|| left_name.cmp(right_name)),
            BreakdownDimension::Speed => SPEED_ORDER
                .iter()
                .position(|value| value == left_name)
                .unwrap_or(usize::MAX)
                .cmp(
                    &SPEED_ORDER
                        .iter()
                        .position(|value| value == right_name)
                        .unwrap_or(usize::MAX),
                )
                .then_with(|| left_name.cmp(right_name)),
        },
    );
    Some(
        totals
            .into_iter()
            .map(|(name, credits)| {
                let percentage = if total_credits_micros <= 0 {
                    0
                } else {
                    (i128::from(credits) * 100 + i128::from(total_credits_micros) / 2)
                        / i128::from(total_credits_micros)
                };
                crate::i18n::tr!(
                    "thread-usage-group-share",
                    name = localized_group_name(dimension, &name),
                    percentage = percentage
                )
            })
            .collect::<Vec<_>>()
            .join(", "),
    )
}

fn format_model_display_name(model: &str) -> String {
    let model = crate::model_catalog::model_display_name(model);
    let Some((prefix, remainder)) = model.split_once('-') else {
        return model.to_string();
    };
    if !prefix.eq_ignore_ascii_case("gpt") || remainder.is_empty() {
        return model.to_string();
    }

    let mut segments = remainder.split('-');
    let first = segments.next().unwrap_or_default();
    let mut display_name = format!("GPT-{first}");
    let numeric_model = first
        .chars()
        .next()
        .is_some_and(|character| character.is_ascii_digit());
    for segment in segments {
        display_name.push(if numeric_model { ' ' } else { '-' });
        if segment.eq_ignore_ascii_case("oai") {
            display_name.push_str("OAI");
            continue;
        }

        let mut characters = segment.chars();
        if let Some(first_character) = characters.next() {
            display_name.extend(first_character.to_uppercase());
            display_name.push_str(characters.as_str());
        }
    }

    display_name
}

pub(crate) fn format_credit_micros(micros: i64) -> String {
    let micros = i128::from(micros.max(/*other*/ 0));
    let (unit_micros, suffix) = [
        (1_000_000_000_000_000_000_i128, "T"),
        (1_000_000_000_000_000_i128, "B"),
        (1_000_000_000_000_i128, "M"),
        (1_000_000_000_i128, "K"),
    ]
    .into_iter()
    .find(|(unit, _)| micros >= *unit)
    .unwrap_or((1_000_000, ""));
    let tenths = (micros * 10 + unit_micros / 2) / unit_micros;
    let whole = tenths / 10;
    let fraction = tenths % 10;
    if fraction == 0 {
        format!("{whole}{suffix}")
    } else {
        format!("{whole}.{fraction}{suffix}")
    }
}

pub(crate) fn format_estimated_usd_micros(micros: i64) -> Option<String> {
    if micros < 0 {
        return None;
    }
    if micros > 0 && micros < 100 {
        return Some(format!("~$0.{micros:06}"));
    }
    if micros > 0 && micros < 10_000 {
        let ten_thousandths = micros.saturating_add(/*rhs*/ 50) / 100;
        return Some(format!("~$0.{ten_thousandths:04}"));
    }

    let cents = micros.saturating_add(/*rhs*/ 5_000) / 10_000;
    Some(format!("~${}.{:02}", cents / 100, cents % 100))
}

#[cfg(test)]
#[path = "thread_usage_tests.rs"]
mod tests;
