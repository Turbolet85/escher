use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;

use blitz_dom::{
    Clock, DocGuard, DocGuardMut, Document, DocumentConfig, FontContext, VirtualClock,
};
use blitz_html::{HtmlDocument, HtmlProvider};
use blitz_traits::events::UiEvent;
use blitz_traits::net::NetProvider;
use blitz_traits::shell::{ColorScheme, Viewport};
use dioxus_core::{
    ComponentFunction, DynamicNode, Element, Properties, ScopeId, Template, TemplateNode,
    VComponent, VNode, VirtualDom, use_hook,
};
use dioxus_native_dom::{DEFAULT_SEED, DioxusDocument, SeededRandom};

use crate::settle::LoadCounter;

/// Options controlling document construction for a [`Harness`].
pub struct HarnessOptions {
    pub width: u32,
    pub height: u32,
    pub scale: f32,
    pub color_scheme: ColorScheme,
    /// Base url which relative URLs are resolved against
    pub base_url: Option<String>,
    /// Net provider used to fetch sub-resources (stylesheets, images, fonts, etc)
    pub net_provider: Option<Arc<dyn NetProvider>>,
    /// Font context used for text shaping. `None` uses the document's default
    /// (system fonts when the `system-fonts` feature is on).
    pub font_ctx: Option<FontContext>,
    /// Whether incremental layout is enabled. `None` uses the document's default.
    pub incremental: Option<bool>,
}

impl Default for HarnessOptions {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            scale: 1.0,
            color_scheme: ColorScheme::Light,
            base_url: None,
            net_provider: None,
            font_ctx: None,
            incremental: None,
        }
    }
}

impl HarnessOptions {
    /// The document's config reading `clock`, and the counter wrapped around the net provider
    /// when one that fetches was supplied.
    fn into_config(self, clock: &VirtualClock) -> (DocumentConfig, Option<Arc<LoadCounter>>) {
        let (net_provider, loads) = match self.net_provider {
            Some(provider) if !provider.is_noop() => {
                let loads = Arc::new(LoadCounter::new(provider));
                (Some(Arc::clone(&loads) as _), Some(loads))
            }
            provider => (provider, None),
        };
        let config = DocumentConfig {
            viewport: Some(Viewport::new(
                self.width,
                self.height,
                self.scale,
                self.color_scheme,
            )),
            base_url: self.base_url,
            net_provider,
            html_parser_provider: Some(Arc::new(HtmlProvider) as _),
            font_ctx: self.font_ctx,
            incremental: self.incremental,
            clock: Some(Arc::new(clock.clone()) as _),
            ..Default::default()
        };
        (config, loads)
    }
}

/// A headless wrapper around a [`Document`] for driving it programmatically in tests.
pub struct Harness<D: Document = HtmlDocument> {
    pub doc: D,
    /// The harness's time: every document a constructor builds reads this clock.
    clock: VirtualClock,
    /// Counts the requests the document issues. `None` for an offline document and for one
    /// passed to [`wrap`](Self::wrap).
    pub(crate) loads: Option<Arc<LoadCounter>>,
}

impl Harness<HtmlDocument> {
    /// Parse `html` into a document with default options (800x600 viewport, scale 1, light mode)
    pub fn from_html(html: &str) -> Self {
        Self::from_html_with(html, HarnessOptions::default())
    }

    pub fn from_html_with(html: &str, options: HarnessOptions) -> Self {
        let clock = VirtualClock::new();
        let (config, loads) = options.into_config(&clock);
        let doc = HtmlDocument::from_html(html, config);
        let mut harness = Self::wrap(doc);
        harness.clock = clock;
        harness.loads = loads;
        harness.pump();
        harness
    }
}

impl Harness<DioxusDocument> {
    /// Build a document from a Dioxus root component with default options
    pub fn from_component(app: fn() -> Element) -> Self {
        Self::from_vdom(VirtualDom::new(app), HarnessOptions::default())
    }

    /// Build a document holding `component` alone, rendered with the props `props` makes, with
    /// `options`: the virtual DOM [`vdom_alone`] makes, built through [`from_vdom`](Self::from_vdom), so it reads
    /// the harness's clock and the run seed; no other root context is provided.
    pub fn from_component_alone<C, P, M: 'static>(
        component: C,
        props: impl Fn() -> P + 'static,
        options: HarnessOptions,
    ) -> Self
    where
        C: ComponentFunction<P, M> + Clone,
        P: Properties + 'static,
    {
        Self::from_vdom(vdom_alone(component, props), options)
    }

    /// Build a document from `vdom` with `options`.
    ///
    /// The document reads the harness's clock and one run seed: the [`SeededRandom`] the vdom's
    /// root already holds (`VirtualDom::with_root_context`), else one seeded with
    /// [`DEFAULT_SEED`], provided at the root before the first render.
    pub fn from_vdom(vdom: VirtualDom, options: HarnessOptions) -> Self {
        if vdom
            .runtime()
            .has_context::<SeededRandom>(ScopeId::ROOT)
            .is_none()
        {
            vdom.provide_root_context(SeededRandom::from_seed(DEFAULT_SEED));
        }
        let clock = VirtualClock::new();
        let (config, loads) = options.into_config(&clock);
        let mut doc = DioxusDocument::new(vdom, config);
        doc.initial_build();
        let mut harness = Self::wrap(doc);
        harness.clock = clock;
        harness.loads = loads;
        harness.pump();
        harness
    }
}

/// A virtual DOM holding `component` alone, rendered with the props `props` makes, not yet built.
///
/// The component is mounted as the one child of an anonymous root, named by its type path as
/// `rsx!` names a child component, so its stable element ids and its UI graph name it as they do
/// inside its app. `props` runs once, in the anonymous root's scope at its first render, so props
/// that need a runtime to be made (an `EventHandler`, an `Element`) live as long as the document.
/// A caller provides root contexts on it (`VirtualDom::with_root_context`) before handing it to
/// [`Harness::from_vdom`].
pub fn vdom_alone<C, P, M: 'static>(component: C, props: impl Fn() -> P + 'static) -> VirtualDom
where
    C: ComponentFunction<P, M> + Clone,
    P: Properties + 'static,
{
    let child = move || props().into_vcomponent(component.clone());
    VirtualDom::new_with_props(
        alone_root,
        AloneRoot {
            child: Rc::new(child),
        },
    )
}

/// The anonymous root's props: what makes the one component it mounts.
#[derive(Clone)]
struct AloneRoot {
    child: Rc<dyn Fn() -> VComponent>,
}

/// Renders exactly one dynamic node, the mounted component, as `rsx!` renders a lone child.
fn alone_root(props: AloneRoot) -> Element {
    let child = use_hook(|| (props.child)());
    Ok(VNode::new(
        None,
        Template {
            roots: &[TemplateNode::Dynamic { id: 0 }],
            node_paths: &[&[0]],
            attr_paths: &[],
        },
        Box::new([DynamicNode::Component(child)]),
        Box::new([]),
    ))
}

impl<D: Document> Harness<D> {
    /// Wrap an already-constructed document. Does not [`pump`](Self::pump).
    ///
    /// The document keeps the clock its own config gave it (wall time by default); the
    /// harness's time, handed to `resolve`, is a fresh virtual clock of its own.
    pub fn wrap(doc: D) -> Self {
        Self {
            doc,
            clock: VirtualClock::new(),
            loads: None,
        }
    }

    pub fn into_inner(self) -> D {
        self.doc
    }

    /// Read access to the underlying [`blitz_dom::BaseDocument`]
    pub fn base(&self) -> DocGuard<'_> {
        self.doc.inner()
    }

    /// Write access to the underlying [`blitz_dom::BaseDocument`]
    pub fn base_mut(&mut self) -> DocGuardMut<'_> {
        self.doc.inner_mut()
    }

    /// The current time (in seconds) of the harness's clock, the one its documents read
    pub fn time(&self) -> f64 {
        self.clock.now().as_secs_f64()
    }

    /// Poll pending async work (e.g. the Dioxus VirtualDom) and resolve style/layout
    pub fn pump(&mut self) {
        self.doc.poll(None);
        let time = self.time();
        self.doc.inner_mut().resolve(time);
    }

    /// Advance the harness's clock by `dt_seconds` and [`pump`](Self::pump). A negative or
    /// non-finite `dt_seconds` moves nothing: the clock never runs backwards.
    pub fn tick(&mut self, dt_seconds: f64) {
        self.advance_time(Duration::try_from_secs_f64(dt_seconds).unwrap_or(Duration::ZERO));
    }

    /// Advance the harness's clock by exactly `by` and [`pump`](Self::pump)
    pub fn advance_time(&mut self, by: Duration) {
        self.clock.advance(by);
        self.pump();
    }

    /// Dispatch a raw [`UiEvent`] through the document's event pipeline.
    ///
    /// Unlike the higher-level input helpers this does not [`pump`](Self::pump) afterwards.
    pub fn dispatch(&mut self, event: UiEvent) {
        self.doc.handle_ui_event(event);
    }

    /// Dispatch raw [`UiEvent`]s, recording the name of every DOM event dispatched
    /// to application code.
    ///
    /// The events are driven directly against the underlying [`blitz_dom::BaseDocument`],
    /// so document-specific event handling (e.g. forwarding to a Dioxus VirtualDom) is
    /// bypassed. Does not [`pump`](Self::pump).
    pub fn dispatch_recorded(&mut self, events: impl IntoIterator<Item = UiEvent>) -> Vec<String> {
        use blitz_dom::{EventDriver, EventHandler, NoopEventHandler};
        use blitz_traits::events::{DomEvent, EventState};
        use std::cell::RefCell;
        use std::rc::Rc;

        #[derive(Clone, Default)]
        struct RecordingHandler {
            events: Rc<RefCell<Vec<String>>>,
        }

        impl EventHandler for RecordingHandler {
            fn handle_event(
                &mut self,
                _chain: &[blitz_traits::node_id::NodeId],
                event: &mut DomEvent,
                _doc: &mut dyn Document,
                _event_state: &mut EventState,
            ) {
                self.events.borrow_mut().push(event.name().to_string());
            }
        }

        let mut doc = self.doc.inner_mut();

        // Events queued before this call (e.g. the focus events of an autofocused element)
        // are not part of the recording
        EventDriver::new(&mut *doc, NoopEventHandler).flush_pending_events();

        let handler = RecordingHandler::default();
        let recorded = handler.events.clone();
        let mut driver = EventDriver::new(&mut *doc, handler);
        for event in events {
            driver.handle_ui_event(event);
        }
        drop(doc);
        recorded.borrow().clone()
    }

    /// Resize the viewport, keeping the current scale and color scheme
    pub fn set_viewport_size(&mut self, width: u32, height: u32) {
        let mut doc = self.doc.inner_mut();
        let viewport = doc.get_viewport();
        doc.set_viewport(Viewport::new(
            width,
            height,
            viewport.hidpi_scale,
            viewport.color_scheme,
        ));
        drop(doc);
        self.pump();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus_core::VText;

    /// A tiny component's props, built by hand: the text its one paragraph shows.
    #[derive(Clone, PartialEq)]
    struct Label(&'static str);

    impl Properties for Label {
        type Builder = ();
        fn builder() -> Self::Builder {}
        fn memoize(&mut self, other: &Self) -> bool {
            self == other
        }
    }

    #[allow(non_snake_case)]
    fn Tiny(props: Label) -> Element {
        Ok(VNode::new(
            None,
            Template {
                roots: &[TemplateNode::Element {
                    tag: "p",
                    namespace: None,
                    attrs: &[],
                    children: &[TemplateNode::Dynamic { id: 0 }],
                }],
                node_paths: &[&[0, 0]],
                attr_paths: &[],
            },
            Box::new([DynamicNode::Text(VText::new(props.0))]),
            Box::new([]),
        ))
    }

    #[test]
    fn a_component_alone_reads_its_props_and_its_name() {
        let harness =
            Harness::from_component_alone(Tiny, || Label("given"), HarnessOptions::default());
        assert!(
            harness.text_content("p") == "given",
            "the paragraph shows the given props"
        );
        let paragraph = harness.query("p").expect("the paragraph is mounted");
        let id = harness
            .doc
            .element_id(paragraph)
            .expect("the paragraph has an id");
        assert!(
            id.starts_with("Tiny/"),
            "the unkeyed id opens with the name"
        );
        let graph = harness.doc.graph();
        assert!(
            graph
                .nodes
                .iter()
                .any(|node| node.id == id && node.component.as_deref() == Some("Tiny")),
            "the graph names the component"
        );
    }
}
