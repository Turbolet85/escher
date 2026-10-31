use crate::events::focus::generate_focus_events;
use crate::{
    BaseDocument, ElementData,
    node::{
        GeneratedTextInputEvent, RangeMove,
        select::{self, RowGesture, SelectCursor, SelectStep, TYPEAHEAD_TIMEOUT, Typeahead},
    },
    util::ACTION_MOD,
};
use blitz_traits::node_id::NodeId;
use blitz_traits::{
    SmolStr,
    events::{BlitzInputEvent, BlitzKeyEvent, DomEvent, DomEventData},
};
use keyboard_types::{Key, Modifiers, NamedKey};
use markup5ever::local_name;

pub(super) enum KeyboardOrTextInputEvent {
    KeyPress(BlitzKeyEvent),
    AppleStandardKeyBinding(SmolStr),
}

pub(crate) fn handle_key_or_input_event<F: FnMut(DomEvent)>(
    doc: &mut BaseDocument,
    target: NodeId,
    event: KeyboardOrTextInputEvent,
    mut dispatch_event: F,
) {
    if let KeyboardOrTextInputEvent::KeyPress(event) = &event {
        if event.key == Key::Named(NamedKey::Tab) {
            let backwards = event.modifiers.contains(Modifiers::SHIFT);
            generate_focus_events(
                doc,
                &mut |doc| {
                    if backwards {
                        doc.focus_prev_node();
                    } else {
                        doc.focus_next_node();
                    }
                },
                &mut dispatch_event,
            );
            return;
        }

        // Handle copy (Ctrl+C/Cmd+C) for text selection when no text input is focused
        if event.state.is_pressed() {
            let action_mod = event.modifiers.contains(ACTION_MOD);
            if action_mod {
                if let Key::Character(c) = &event.key {
                    if c.to_lowercase() == "c" {
                        // Check if we have a text selection (and no focused text input)
                        let has_focused_text_input = doc.focus_node_id.is_some_and(|id| {
                            doc.get_node(id)
                                .and_then(|n| n.element_data())
                                .is_some_and(|e| e.text_input_data().is_some())
                        });

                        if !has_focused_text_input {
                            if let Some(text) = doc.get_selected_text() {
                                let _ = doc.shell_provider.set_clipboard_text(text);
                                return;
                            }
                        }
                    }
                }
            }
        }
    }

    if let Some(node_id) = doc.focus_node_id {
        if target != node_id {
            return;
        }

        let node = &mut doc.nodes[node_id];
        let Some(element_data) = node.element_data_mut() else {
            return;
        };

        if let Some(input_data) = element_data.text_input_data_mut() {
            let generated_event = match event {
                KeyboardOrTextInputEvent::KeyPress(blitz_key_event) => input_data
                    .apply_keypress_event(
                        &mut doc.font_ctx.lock().unwrap(),
                        &mut doc.layout_ctx,
                        &*doc.shell_provider,
                        blitz_key_event,
                    ),
                KeyboardOrTextInputEvent::AppleStandardKeyBinding(command) => input_data
                    .apply_apple_standard_keybinding(
                        &mut doc.font_ctx.lock().unwrap(),
                        &mut doc.layout_ctx,
                        &*doc.shell_provider,
                        &command,
                    ),
            };

            if let Some(generated_event) = generated_event {
                doc.apply_generated_text_input_event(node_id, generated_event, dispatch_event);
            }
        } else if element_data.range_input_data().is_some()
            && !element_data.has_attr(local_name!("disabled"))
            && let KeyboardOrTextInputEvent::KeyPress(key_event) = &event
            && key_event.state.is_pressed()
        {
            press_range_input_key(doc, node_id, &key_event.key, &mut dispatch_event);
        } else if select::is_single_select(element_data)
            && !element_data.is_disabled()
            && let KeyboardOrTextInputEvent::KeyPress(key_event) = &event
            && key_event.state.is_pressed()
        {
            press_select_key(doc, node_id, key_event, &mut dispatch_event);
        } else if select::is_multiple_select(element_data)
            && !element_data.is_disabled()
            && let KeyboardOrTextInputEvent::KeyPress(key_event) = &event
            && key_event.state.is_pressed()
        {
            press_multiple_select_key(doc, node_id, key_event, &mut dispatch_event);
        } else if let KeyboardOrTextInputEvent::KeyPress(key_event) = &event
            && key_event.state.is_pressed()
            && key_event.key == Key::Named(NamedKey::Enter)
            && activates_on_enter(element_data)
        {
            dispatch_activation_click(doc, node_id, key_event.modifiers, &mut dispatch_event);
        }
    }
}

/// A released Space activates the focused control it targets, as a click would.
pub(crate) fn handle_key_up<F: FnMut(DomEvent)>(
    doc: &mut BaseDocument,
    target: NodeId,
    event: &BlitzKeyEvent,
    mut dispatch_event: F,
) {
    if doc.focus_node_id != Some(target) || event.state.is_pressed() {
        return;
    }
    if !matches!(&event.key, Key::Character(c) if c == " ") {
        return;
    }
    if doc
        .get_node(target)
        .and_then(|node| node.element_data())
        .is_some_and(activates_on_space)
    {
        dispatch_activation_click(doc, target, event.modifiers, &mut dispatch_event);
    }
}

/// https://html.spec.whatwg.org/multipage/input.html#range-state-(type=range):
/// the arrows step a range input's value, Home and End take it to its minimum and maximum, and
/// every other key does nothing.
fn press_range_input_key(
    doc: &mut BaseDocument,
    node_id: NodeId,
    key: &Key,
    dispatch_event: &mut dyn FnMut(DomEvent),
) {
    let to = match key {
        Key::Named(NamedKey::ArrowRight) | Key::Named(NamedKey::ArrowUp) => RangeMove::StepUp,
        Key::Named(NamedKey::ArrowLeft) | Key::Named(NamedKey::ArrowDown) => RangeMove::StepDown,
        Key::Named(NamedKey::Home) => RangeMove::Minimum,
        Key::Named(NamedKey::End) => RangeMove::Maximum,
        _ => return,
    };
    doc.apply_range_move(node_id, to, dispatch_event);
}

/// A focused single-choice select's keys: the arrows choose the next or previous option that is
/// not disabled, Home and End the first and last, and a typed character joins the text typed so
/// far — afresh where it was typed into another select, or after [`TYPEAHEAD_TIMEOUT`] of the
/// document's clock — choosing the first option whose label starts with it. Every other key, and
/// a character typed with Ctrl, Alt or Meta held, does nothing: no list is opened.
fn press_select_key(
    doc: &mut BaseDocument,
    node_id: NodeId,
    event: &BlitzKeyEvent,
    dispatch_event: &mut dyn FnMut(DomEvent),
) {
    let options = select::options(doc, node_id);
    let readings = select::readings(doc, &options);
    let target = match (&event.key, select_step(&event.key)) {
        (_, Some(step)) => select::stepped(&readings, step, select::last_chosen(&readings)),
        (Key::Character(text), None) if types_into_select(event.modifiers) => {
            let typed = doc.type_into_select(node_id, text);
            select::matched(&readings, &typed)
        }
        _ => None,
    };
    if let Some(option) = target.and_then(|index| options.get(index).copied()) {
        doc.apply_select_choice(node_id, &[option], dispatch_event);
    }
}

/// The step a select's key takes.
fn select_step(key: &Key) -> Option<SelectStep> {
    match key {
        Key::Named(NamedKey::ArrowDown) => Some(SelectStep::Next),
        Key::Named(NamedKey::ArrowUp) => Some(SelectStep::Previous),
        Key::Named(NamedKey::Home) => Some(SelectStep::First),
        Key::Named(NamedKey::End) => Some(SelectStep::Last),
        _ => None,
    }
}

/// Whether a character pressed with `modifiers` held is typed text: none of Ctrl, Alt or Meta.
fn types_into_select(modifiers: Modifiers) -> bool {
    !modifiers.intersects(Modifiers::CONTROL | Modifiers::ALT | Modifiers::META)
}

/// A focused `multiple` select's keys: the arrows, Home and End alone choose the row they reach
/// alone, with Shift span from the anchor to it, and with Ctrl or Meta move the current row and
/// change nothing; a Space with no typed text standing flips the current row, and with typed text
/// standing joins it; a character typed with no Ctrl, Alt or Meta chooses the first row its label
/// starts, alone. Every other key does nothing: no list is opened.
fn press_multiple_select_key(
    doc: &mut BaseDocument,
    node_id: NodeId,
    event: &BlitzKeyEvent,
    dispatch_event: &mut dyn FnMut(DomEvent),
) {
    let options = select::options(doc, node_id);
    let readings = select::readings(doc, &options);
    let (_, current) = doc.select_rows(node_id, &options);
    let current = current.or_else(|| select::last_chosen(&readings));
    let gesture = match (&event.key, select_step(&event.key)) {
        (_, Some(step)) => select::stepped(&readings, step, current)
            .map(|row| keyed_row_gesture(row, event.modifiers)),
        (Key::Character(text), None) if types_into_select(event.modifiers) => {
            if text == " " && !doc.select_text_stands(node_id) {
                current.map(RowGesture::Toggle)
            } else {
                let typed = doc.type_into_select(node_id, text);
                select::matched(&readings, &typed).map(RowGesture::Alone)
            }
        }
        _ => None,
    };
    if let Some(gesture) = gesture {
        doc.apply_row_gesture(node_id, &options, gesture, dispatch_event);
    }
}

/// What a step to `row` does with `modifiers` held.
fn keyed_row_gesture(row: usize, modifiers: Modifiers) -> RowGesture {
    if modifiers.intersects(Modifiers::CONTROL | Modifiers::META) {
        RowGesture::Move(row)
    } else if modifiers.contains(Modifiers::SHIFT) {
        RowGesture::Span(row)
    } else {
        RowGesture::Alone(row)
    }
}

/// https://html.spec.whatwg.org/multipage/interaction.html#activation-behaviour:
/// Enter activates a button, a submit, reset or button input, a link and a summary.
fn activates_on_enter(element: &ElementData) -> bool {
    if element.is_disabled() {
        return false;
    }
    match element.name.local {
        local_name!("button") | local_name!("summary") => true,
        local_name!("input") => matches!(
            element.attr(local_name!("type")),
            Some("submit" | "reset" | "button")
        ),
        local_name!("a") => element.has_attr(local_name!("href")),
        _ => false,
    }
}

/// Space activates a button, a submit, reset, button, checkbox or radio input and a summary;
/// a link takes no Space.
fn activates_on_space(element: &ElementData) -> bool {
    if element.is_disabled() {
        return false;
    }
    match element.name.local {
        local_name!("button") | local_name!("summary") => true,
        local_name!("input") => matches!(
            element.attr(local_name!("type")),
            Some("submit" | "reset" | "button" | "checkbox" | "radio")
        ),
        _ => false,
    }
}

/// Dispatches a click at the control, so the app's handlers run before the click's default
/// action, as for a pointer click.
fn dispatch_activation_click(
    doc: &BaseDocument,
    node_id: NodeId,
    modifiers: Modifiers,
    dispatch_event: &mut dyn FnMut(DomEvent),
) {
    let data = doc.nodes[node_id].synthetic_click_event_data(modifiers);
    dispatch_event(DomEvent::new(node_id, DomEventData::Click(data)));
}

impl BaseDocument {
    /// Adds `text` to the text typed into `select` and reads the whole of it.
    fn type_into_select(&mut self, select: NodeId, text: &str) -> String {
        let now = self.clock.now();
        let typeahead = match self.select_typeahead.take() {
            Some(mut typeahead)
                if typeahead.select == select
                    && now.saturating_sub(typeahead.at) < TYPEAHEAD_TIMEOUT =>
            {
                typeahead.text.push_str(text);
                typeahead.at = now;
                typeahead
            }
            _ => Typeahead {
                select,
                text: text.to_string(),
                at: now,
            },
        };
        let typed = typeahead.text.clone();
        self.select_typeahead = Some(typeahead);
        typed
    }

    /// Whether text typed into `select` is still kept for the next character.
    fn select_text_stands(&self, select: NodeId) -> bool {
        let now = self.clock.now();
        self.select_typeahead.as_ref().is_some_and(|typeahead| {
            typeahead.select == select && now.saturating_sub(typeahead.at) < TYPEAHEAD_TIMEOUT
        })
    }

    /// The anchor and the current row of a `multiple` select's cursor, as indices into `options`:
    /// each `None` where the cursor is another select's or its row has gone.
    pub(crate) fn select_rows(
        &self,
        select: NodeId,
        options: &[NodeId],
    ) -> (Option<usize>, Option<usize>) {
        let row = |id: NodeId| {
            self.get_node(id)?;
            options.iter().position(|option| *option == id)
        };
        match self.select_cursor.filter(|cursor| cursor.select == select) {
            Some(cursor) => (row(cursor.anchor), row(cursor.current)),
            None => (None, None),
        }
    }

    /// Runs `gesture` on the rows `options` of a `multiple` select: its cursor moves, and the set
    /// it leaves goes through [`Self::apply_select_choice`].
    pub(crate) fn apply_row_gesture(
        &mut self,
        select: NodeId,
        options: &[NodeId],
        gesture: RowGesture,
        dispatch_event: &mut dyn FnMut(DomEvent),
    ) {
        let readings = select::readings(self, options);
        let (anchor, _) = self.select_rows(select, options);
        let anchor = anchor.or_else(|| select::last_chosen(&readings));
        let gestured = select::gestured(&readings, anchor, gesture);
        if let (Some(&anchor), Some(&current)) =
            (options.get(gestured.anchor), options.get(gestured.current))
        {
            self.select_cursor = Some(SelectCursor {
                select,
                anchor,
                current,
            });
        }
        if let Some(set) = gestured.set {
            let chosen: Vec<NodeId> = set
                .into_iter()
                .filter_map(|row| options.get(row).copied())
                .collect();
            self.apply_select_choice(select, &chosen, dispatch_event);
        }
    }

    /// Makes `chosen` a select's chosen options. Where the choice changed, the select joins the
    /// changed set, an `input` and then a `change` event, each carrying the select's new value,
    /// are dispatched and a redraw is requested.
    pub(crate) fn apply_select_choice(
        &mut self,
        select: NodeId,
        chosen: &[NodeId],
        dispatch_event: &mut dyn FnMut(DomEvent),
    ) {
        let options = select::options(self, select);
        if !self.set_select_choice(&options, chosen) {
            return;
        }
        let value = self.select_value(select).unwrap_or_default();
        self.changed_nodes.insert(select);
        dispatch_event(DomEvent::new(
            select,
            DomEventData::Input(BlitzInputEvent {
                value: value.clone(),
            }),
        ));
        dispatch_event(DomEvent::new(
            select,
            DomEventData::Change(BlitzInputEvent { value }),
        ));
        self.shell_provider.request_redraw();
    }

    pub(crate) fn apply_generated_text_input_event<F: FnMut(DomEvent)>(
        &mut self,
        node_id: NodeId,
        event: GeneratedTextInputEvent,
        mut dispatch_event: F,
    ) {
        let node = &mut self.nodes[node_id];
        let element_data = node
            .element_data_mut()
            .expect("apply_generated_text_input_event called on a node that is not an element");
        let input_data = element_data
            .text_input_data_mut()
            .expect("apply_generated_text_input_event called on a node that is not a text input");

        match event {
            GeneratedTextInputEvent::Input => {
                let value = input_data.editor.raw_text().to_string();
                self.changed_nodes.insert(node_id);
                dispatch_event(DomEvent::new(
                    node_id,
                    DomEventData::Input(BlitzInputEvent { value }),
                ));
                self.shell_provider.request_redraw();
            }
            GeneratedTextInputEvent::Select | GeneratedTextInputEvent::PreEditChange => {
                self.shell_provider.request_redraw();
            }
            GeneratedTextInputEvent::Submit => {
                // TODO: Generate submit event that can be handled by script
                implicit_form_submission(self, node_id);
            }
        }
    }

    /// Moves a range input's current value. Where it changed, the node joins the changed set,
    /// an `input` event carrying the new value is dispatched and a redraw is requested.
    pub(crate) fn apply_range_move(
        &mut self,
        node_id: NodeId,
        to: RangeMove,
        dispatch_event: &mut dyn FnMut(DomEvent),
    ) {
        let Some(value) = self
            .get_node_mut(node_id)
            .and_then(|node| node.element_data_mut())
            .and_then(|element| element.move_range_input(to))
        else {
            return;
        };
        self.changed_nodes.insert(node_id);
        dispatch_event(DomEvent::new(
            node_id,
            DomEventData::Input(BlitzInputEvent { value }),
        ));
        self.shell_provider.request_redraw();
    }
}

/// https://html.spec.whatwg.org/multipage/form-control-infrastructure.html#field-that-blocks-implicit-submission
fn implicit_form_submission(doc: &BaseDocument, text_target: NodeId) {
    let Some(form_owner_id) = doc.controls_to_form.get(&text_target) else {
        return;
    };
    if doc
        .controls_to_form
        .iter()
        .filter(|(_control_id, form_id)| *form_id == form_owner_id)
        .filter_map(|(control_id, _)| doc.nodes[*control_id].element_data())
        .filter(|element_data| {
            element_data.attr(local_name!("type")).is_some_and(|t| {
                matches!(
                    t,
                    "text"
                        | "search"
                        | "email"
                        | "url"
                        | "tel"
                        | "password"
                        | "date"
                        | "month"
                        | "week"
                        | "time"
                        | "datetime-local"
                        | "number"
                )
            })
        })
        .count()
        > 1
    {
        return;
    }

    doc.submit_form(*form_owner_id, *form_owner_id);
}
