//! The order page, the stand's served page whose ordinary steps cause something: a Submit with
//! a quantity outside its rule returns an `Err` the page's own error boundary takes, showing only
//! a failure and a Retry; a good Submit logs one fixed message and places the order; Export
//! panics. Every text it writes is its own fixed text, with no field.

use dioxus_native::prelude::dioxus_core::CapturedError;
use dioxus_native::prelude::*;

use super::{use_graph_signal, writes};

/// The rule a Submit's quantity breaks, the text of the `Err` it returns.
pub const QUANTITY_RULE: &str = "quantity must be a whole number from 10 to 20";

/// The message a good Submit logs, at `info`.
pub const ORDER_PLACED: &str = "order placed";

/// The payload Export panics with.
pub const EXPORT_UNBUILT: &str = "export is not built yet";

/// The line Export's panic stands on in this file.
pub const EXPORT_PANIC_LINE: u32 = line!() + 2;
fn export() {
    panic!("{EXPORT_UNBUILT}");
}

/// Whether `quantity` is a whole number from 10 to 20.
fn quantity_holds(quantity: &str) -> bool {
    quantity
        .trim()
        .parse::<u32>()
        .is_ok_and(|count| (10..=20).contains(&count))
}

#[component]
pub fn OrderPage() -> Element {
    let mut quantity = use_graph_signal(
        "quantity",
        const { &[writes("order-quantity", "input")] },
        || String::from("0"),
    );
    let placed = use_graph_signal(
        "placed",
        const { &[writes("order-submit", "click")] },
        || false,
    );

    rsx! {
        div { class: "order-root",
            style { {CSS} }
            div { class: "order-card",
                label { r#for: "order-quantity", class: "order-label", "Quantity" }
                input {
                    id: "order-quantity",
                    class: "order-input",
                    value: "{quantity}",
                    oninput: move |evt| quantity.set(evt.value()),
                }
                ErrorBoundary {
                    handle_error: |errors: ErrorContext| rsx! {
                        p { id: "order-failed", class: "order-failed", "Order failed" }
                        button {
                            id: "order-retry",
                            class: "order-btn",
                            onclick: move |_| errors.clear_errors(),
                            "Retry"
                        }
                    },
                    OrderSubmit { quantity, placed }
                }
                button {
                    id: "order-export",
                    class: "order-btn",
                    onclick: move |_| export(),
                    "Export"
                }
                p { id: "order-status", class: "order-status",
                    if placed() { "Order placed" }
                }
            }
        }
    }
}

#[component]
fn OrderSubmit(quantity: Signal<String>, placed: Signal<bool>) -> Element {
    rsx! {
        button {
            id: "order-submit",
            class: "order-btn",
            onclick: move |_| -> Result<(), CapturedError> {
                if !quantity_holds(&quantity.read()) {
                    return Err(CapturedError::from_display(QUANTITY_RULE));
                }
                placed.set(true);
                tracing::info!("{ORDER_PLACED}");
                Ok(())
            },
            "Submit"
        }
    }
}

const CSS: &str = r#"
.order-root {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    height: 100%;
    background-color: #f5f5f5;
    font-family: sans-serif;
}

.order-card {
    display: flex;
    flex-direction: column;
    gap: 12px;
    background: #ffffff;
    border: 1px solid #d0d0d0;
    border-radius: 8px;
    padding: 28px 32px;
    min-width: 320px;
}

.order-label {
    font-size: 15px;
    color: #1a1a1a;
}

.order-input {
    font-size: 15px;
    padding: 8px 10px;
    border: 1px solid #c0c0c0;
    border-radius: 6px;
    color: #1a1a1a;
    background: #ffffff;
}

.order-input:focus {
    outline: none;
    border-color: #3a5ce5;
}

.order-btn {
    font-size: 15px;
    font-weight: 500;
    color: #ffffff;
    background-color: #3a5ce5;
    border: none;
    border-radius: 6px;
    padding: 10px;
    cursor: pointer;
}

.order-btn:hover {
    background-color: #2a4cd3;
}

.order-btn:active {
    background-color: #1f3fc1;
}

.order-failed {
    font-size: 15px;
    padding: 8px 10px;
    border-radius: 6px;
    color: #c53030;
    background: #fff5f5;
}

.order-status {
    font-size: 15px;
    color: #1a1a1a;
}
"#;
