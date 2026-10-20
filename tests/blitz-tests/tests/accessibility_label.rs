use accesskit::{Node as AccessKitNode, NodeId, Role};
use blitz_dom::DocumentConfig;
use blitz_html::{HtmlDocument, HtmlProvider};
use blitz_traits::shell::{ColorScheme, Viewport};
use std::sync::Arc;
use test_that::prelude::*;

#[test]
fn includes_aria_label_as_label() -> TestResult<()> {
    let mut document = HtmlDocument::from_html(
        r#"<html><div role="button" aria-label="A node"></div></html>"#,
        default_document_config(),
    );
    document.resolve(0.0);

    let tree_update = document.build_accessibility_tree();

    verify_that!(
        tree_update.nodes,
        contains((
            anything(),
            matches_pattern!(AccessKitNode {
                role(): eq(Role::Button),
                label(): some(eq("A node")),
            })
        ))
    )
}

#[test]
fn includes_aria_labelledby_as_labelled_by_for_one_node() -> TestResult<()> {
    let mut document = HtmlDocument::from_html(
        r#"<html>
          <div role="button" aria-labelledby="arbitrary-element"></div>
          <label id="arbitrary-element">Label</label>
        </html>"#,
        default_document_config(),
    );
    document.resolve(0.0);

    let tree_update = document.build_accessibility_tree();

    let (label_node_id, _) = tree_update
        .nodes
        .iter()
        .find(|(_, node)| node.role() == Role::Label)
        .expect("Label must be present in accessibility tree");
    verify_that!(
        tree_update.nodes,
        contains((
            anything(),
            matches_pattern!(AccessKitNode {
                role(): eq(Role::Button),
                *labelled_by(): container_eq(vec![*label_node_id]),
            })
        ))
    )
}

#[test]
fn includes_aria_labelledby_as_labelled_by_for_two_nodes() -> TestResult<()> {
    let mut document = HtmlDocument::from_html(
        r#"<html>
          <div role="button" aria-labelledby="arbitrary-element another-element"></div>
          <label id="another-element">Another label</label>
          <nav id="arbitrary-element">Label</nav>
        </html>"#,
        default_document_config(),
    );
    document.resolve(0.0);

    let tree_update = document.build_accessibility_tree();

    // This awkward construction is due to having to fetch the node IDs for the label
    // elements in the same order they appear in the aria-labelledby attribute. The
    // order is also important to the assertion.
    let mut expected_node_ids = Vec::new();
    expected_node_ids.push(
        tree_update
            .nodes
            .iter()
            .find(|(_, node)| node.role() == Role::Navigation)
            .expect("Expected to find label arbitrary-element in the tree")
            .0,
    );
    expected_node_ids.push(
        tree_update
            .nodes
            .iter()
            .find(|(_, node)| node.role() == Role::Label)
            .expect("Expected to find label another-element in the tree")
            .0,
    );
    verify_that!(
        tree_update.nodes,
        contains((
            anything(),
            matches_pattern!(AccessKitNode {
                role(): eq(Role::Button),
                *labelled_by(): container_eq(expected_node_ids),
            })
        ))
    )
}

#[test]
fn ignores_labelledby_element_which_does_not_correspond_to_element_in_the_dom() -> TestResult<()> {
    let mut document = HtmlDocument::from_html(
        r#"<html><div role="button" aria-labelledby="nonexistent-element"></div></html>"#,
        default_document_config(),
    );
    document.resolve(0.0);

    let tree_update = document.build_accessibility_tree();

    verify_that!(
        tree_update.nodes,
        contains((
            anything(),
            matches_pattern!(AccessKitNode {
                role(): eq(Role::Button),
                *labelled_by(): is_empty(),
            })
        ))
    )
}

#[test]
fn uses_text_of_aria_labelledby_targets_as_label() -> TestResult<()> {
    let mut document = HtmlDocument::from_html(
        r#"<html>
          <div role="button" aria-labelledby="second first"></div>
          <span id="first">  First </span>
          <span id="second">Second
            label</span>
        </html>"#,
        default_document_config(),
    );
    document.resolve(0.0);

    let tree_update = document.build_accessibility_tree();

    verify_that!(
        tree_update.nodes,
        contains((
            anything(),
            matches_pattern!(AccessKitNode {
                role(): eq(Role::Button),
                label(): some(eq("Second label First")),
            })
        ))
    )
}

#[test]
fn aria_labelledby_takes_precedence_over_aria_label_and_content() -> TestResult<()> {
    let mut document = HtmlDocument::from_html(
        r#"<html>
          <button aria-label="Ignored" aria-labelledby="label">Click</button>
          <span id="label">Save</span>
        </html>"#,
        default_document_config(),
    );
    document.resolve(0.0);

    let tree_update = document.build_accessibility_tree();

    let (span_node_id, _) = tree_update
        .nodes
        .iter()
        .find(|(_, node)| node.html_tag() == Some("span"))
        .expect("Span must be present in accessibility tree");
    verify_that!(
        tree_update.nodes,
        contains((
            anything(),
            matches_pattern!(AccessKitNode {
                role(): eq(Role::Button),
                label(): some(eq("Save")),
                *labelled_by(): container_eq(vec![*span_node_id]),
            })
        ))
    )
}

#[test]
fn includes_self_reference_and_nested_text_in_aria_labelledby() -> TestResult<()> {
    let mut document = HtmlDocument::from_html(
        r#"<html>
          <div role="button" id="self" aria-labelledby="self label">Delete</div>
          <span id="label">file<b>name</b></span>
        </html>"#,
        default_document_config(),
    );
    document.resolve(0.0);

    let tree_update = document.build_accessibility_tree();

    let (button_node_id, _) = tree_update
        .nodes
        .iter()
        .find(|(_, node)| node.role() == Role::Button)
        .expect("Button must be present in accessibility tree");
    let (span_node_id, _) = tree_update
        .nodes
        .iter()
        .find(|(_, node)| node.html_tag() == Some("span"))
        .expect("Span must be present in accessibility tree");
    verify_that!(
        tree_update.nodes,
        contains((
            anything(),
            matches_pattern!(AccessKitNode {
                role(): eq(Role::Button),
                label(): some(eq("Delete filename")),
                *labelled_by(): container_eq(vec![*button_node_id, *span_node_id]),
            })
        ))
    )
}

#[test]
fn uses_aria_label_of_aria_labelledby_target() -> TestResult<()> {
    let mut document = HtmlDocument::from_html(
        r#"<html>
          <div role="button" aria-labelledby="label"></div>
          <span id="label" aria-label="From aria-label">From content</span>
        </html>"#,
        default_document_config(),
    );
    document.resolve(0.0);

    let tree_update = document.build_accessibility_tree();

    verify_that!(
        tree_update.nodes,
        contains((
            anything(),
            matches_pattern!(AccessKitNode {
                role(): eq(Role::Button),
                label(): some(eq("From aria-label")),
            })
        ))
    )
}

#[test]
fn uses_hidden_aria_labelledby_targets_for_label_without_referencing_them() -> TestResult<()> {
    let mut document = HtmlDocument::from_html(
        r#"<html>
          <div role="button" aria-labelledby="hidden-element hidden-descendant"></div>
          <span id="hidden-element" style="display: none">Save</span>
          <div style="visibility: hidden"><span id="hidden-descendant">file</span></div>
        </html>"#,
        default_document_config(),
    );
    document.resolve(0.0);

    let tree_update = document.build_accessibility_tree();

    verify_that!(
        tree_update.nodes,
        contains((
            anything(),
            matches_pattern!(AccessKitNode {
                role(): eq(Role::Button),
                label(): some(eq("Save file")),
                *labelled_by(): is_empty(),
            })
        ))
    )
}

#[test]
fn falls_back_to_aria_label_when_aria_labelledby_targets_have_no_text() -> TestResult<()> {
    let mut document = HtmlDocument::from_html(
        r#"<html>
          <div role="button" aria-label="Fallback" aria-labelledby="empty"></div>
          <span id="empty"></span>
        </html>"#,
        default_document_config(),
    );
    document.resolve(0.0);

    let tree_update = document.build_accessibility_tree();

    verify_that!(
        tree_update.nodes,
        contains((
            anything(),
            matches_pattern!(AccessKitNode {
                role(): eq(Role::Button),
                label(): some(eq("Fallback")),
            })
        ))
    )
}

/// The accessibility node id of the element `selector` matches.
fn tree_id(document: &HtmlDocument, selector: &str) -> NodeId {
    let node = document
        .query_selector(selector)
        .ok()
        .flatten()
        .unwrap_or_else(|| panic!("the fixture holds {selector}"));
    NodeId(node.as_u64())
}

#[test]
fn a_label_for_names_a_select_and_a_textarea() -> TestResult<()> {
    for incremental in [false, true] {
        let mut document = HtmlDocument::from_html(
            r#"<html>
              <label id="fruit" for="pick">Fruit</label>
              <select id="pick"><option>Apple</option></select>
              <label id="note" for="say">Note</label>
              <textarea id="say"></textarea>
            </html>"#,
            config_in(incremental),
        );
        document.resolve(0.0);
        let (fruit, note) = (tree_id(&document, "#fruit"), tree_id(&document, "#note"));

        let tree_update = document.build_accessibility_tree();

        verify_that!(
            tree_update.nodes,
            contains((
                anything(),
                matches_pattern!(AccessKitNode {
                    role(): eq(Role::ComboBox),
                    *labelled_by(): container_eq(vec![fruit]),
                })
            ))
        )?;
        verify_that!(
            tree_update.nodes,
            contains((
                anything(),
                matches_pattern!(AccessKitNode {
                    role(): eq(Role::MultilineTextInput),
                    *labelled_by(): container_eq(vec![note]),
                })
            ))
        )?;
    }
    Ok(())
}

#[test]
fn a_wrapping_label_names_a_select_and_a_textarea() -> TestResult<()> {
    for incremental in [false, true] {
        let mut document = HtmlDocument::from_html(
            r#"<html>
              <label id="colour">Colour <select><option>Red</option></select></label>
              <label id="comment">Comment <textarea></textarea></label>
            </html>"#,
            config_in(incremental),
        );
        document.resolve(0.0);
        let colour = tree_id(&document, "#colour");
        let comment = tree_id(&document, "#comment");

        let tree_update = document.build_accessibility_tree();

        verify_that!(
            tree_update.nodes,
            contains((
                anything(),
                matches_pattern!(AccessKitNode {
                    role(): eq(Role::ComboBox),
                    *labelled_by(): container_eq(vec![colour]),
                })
            ))
        )?;
        verify_that!(
            tree_update.nodes,
            contains((
                anything(),
                matches_pattern!(AccessKitNode {
                    role(): eq(Role::MultilineTextInput),
                    *labelled_by(): container_eq(vec![comment]),
                })
            ))
        )?;
    }
    Ok(())
}

#[test]
fn a_label_names_no_hidden_input() -> TestResult<()> {
    for incremental in [false, true] {
        let mut document = HtmlDocument::from_html(
            r#"<html>
              <label id="for-hidden" for="secret">Secret</label>
              <input id="secret" type="hidden" value="x">
              <label id="wrapping">Name <input type="hidden"><input id="name"></label>
            </html>"#,
            config_in(incremental),
        );
        document.resolve(0.0);
        let for_hidden = document
            .query_selector("#for-hidden")
            .ok()
            .flatten()
            .expect("the fixture holds the label");
        let wrapping = document
            .query_selector("#wrapping")
            .ok()
            .flatten()
            .expect("the fixture holds the label");
        let name = document
            .query_selector("#name")
            .ok()
            .flatten()
            .expect("the fixture holds the input");

        verify_that!(
            document
                .label_bound_input_element(for_hidden)
                .map(|node| node.id),
            none()
        )?;
        verify_that!(
            document
                .label_bound_input_element(wrapping)
                .map(|node| node.id),
            some(eq(name))
        )?;
    }
    Ok(())
}

fn config_in(incremental: bool) -> DocumentConfig {
    DocumentConfig {
        incremental: Some(incremental),
        ..default_document_config()
    }
}

fn default_document_config() -> DocumentConfig {
    DocumentConfig {
        viewport: Some(Viewport::new(800, 600, 1.0, ColorScheme::Light)),
        html_parser_provider: Some(Arc::new(HtmlProvider) as _),
        ..Default::default()
    }
}
