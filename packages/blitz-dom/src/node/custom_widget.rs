use std::any::Any;

use anyrender::ResourceId;
use blitz_traits::events::UiEvent;
pub use style::properties::ComputedValues as ComputedStyles;
// use accesskit::Node as AccessKitNode;
// use taffy::{LayoutInput, LayoutOutput};

pub use anyrender::{RenderContext, Scene};

use crate::BaseDocument;
use crate::layout::replaced::IntrinsicSizes;

impl BaseDocument {
    pub fn can_create_surfaces(&mut self, render_context: &mut dyn RenderContext) {
        for &node_id in self.custom_widget_nodes.iter() {
            let node = &mut self.nodes[node_id];
            if let Some(widget_data) = node
                .element_data_mut()
                .and_then(|el| el.custom_widget_data_mut())
            {
                let mut render_context = ProxyRenderContext {
                    resource_ids: &mut widget_data.active_resource_ids,
                    inner: render_context,
                };

                widget_data
                    .widget
                    .can_create_surfaces(&mut render_context as _);
            }
        }
    }

    pub fn destroy_surfaces(&mut self) {
        for &node_id in self.custom_widget_nodes.iter() {
            let node = &mut self.nodes[node_id];
            if let Some(widget_data) = node
                .element_data_mut()
                .and_then(|el| el.custom_widget_data_mut())
            {
                widget_data.widget.destroy_surfaces();
            }
        }
    }
}

/// A `RenderContext` that proxies resource registrations through to an inner `RenderContext`
/// and also keeps track of the `ResourceId`s of all sucessfully registered resources so that
/// they can be automatically unregistered if the Widget's node is dropped.
pub struct ProxyRenderContext<'widget, 'rend> {
    pub resource_ids: &'widget mut Vec<ResourceId>,
    pub inner: &'rend mut dyn RenderContext,
}

impl anyrender::RenderContext for ProxyRenderContext<'_, '_> {
    fn try_register_custom_resource(
        &mut self,
        resource: Box<dyn Any>,
    ) -> Result<ResourceId, anyrender::RegisterResourceError> {
        let id = self.inner.try_register_custom_resource(resource)?;
        self.resource_ids.push(id);
        Ok(id)
    }

    fn unregister_resource(&mut self, resource_id: ResourceId) {
        self.resource_ids.retain(|id| *id != resource_id);
        self.inner.unregister_resource(resource_id);
    }

    fn renderer_specific_context(&self) -> Option<Box<dyn std::any::Any>> {
        self.inner.renderer_specific_context()
    }
}

pub trait Widget {
    // DOM lifecycle

    /// The widget was attached to the DOM
    fn connected(&mut self) {}
    /// The widget was removed from the DOM
    fn disconnected(&mut self) {}
    /// One of the widget's attributes changed
    ///
    /// Existing attributes are replayed on attachment with no old value.
    fn attribute_changed(&mut self, name: &str, old_value: Option<&str>, new_value: Option<&str>) {
        let _ = (name, old_value, new_value);
    }

    // Renderer lifecycle

    /// The renderer is active
    ///
    /// `ctx` parameter can be downcast to get access to renderer-specific contexts (e.g. the WGPU Device and Queue)
    fn can_create_surfaces(&mut self, render_ctx: &mut dyn RenderContext) {
        let _ = render_ctx;
    }
    /// The renderer is no longer active (destroy textures here)
    fn destroy_surfaces(&mut self) {}

    // Other

    /// Whether the widget currently requires redraws (e.g. because it is animating).
    ///
    /// Returning `true` causes the document to continuously schedule redraws
    /// (and hence repaints of the widget). Static widgets should return `false`.
    fn requires_redraw(&self) -> bool {
        false
    }

    /// Handle input events (mouse, keyboard, etc)
    ///
    /// A drag over the widget is refused unless the widget sets the `dropEffect` of the
    /// data transfer of the [`UiEvent::DragOver`] event to the operation that a drop
    /// would perform.
    fn handle_event(&mut self, event: &UiEvent) {
        let _ = event;
    }

    /// The widget's intrinsic dimensions: an intrinsic width, height and
    /// aspect ratio, each of which may independently be absent.
    ///
    /// An absent dimension is sized as it would be without the widget (for a
    /// replaced element, the element's own intrinsic dimension or the default
    /// object size).
    fn intrinsic_sizes(&self) -> IntrinsicSizes {
        IntrinsicSizes::default()
    }

    /// The document's own font context, handed before each paint, so text the widget
    /// draws is shaped from the fonts the document's text is.
    fn fonts(&mut self, fonts: &std::sync::Arc<std::sync::Mutex<parley::FontContext>>) {
        let _ = fonts;
    }

    /// Callback for the widget to paint it's content.
    ///
    /// Output is recorded to an AnyRender `Scene`.
    /// If the widget wants to render to a WGPU texture or similar then it should:
    ///   - Get a handle to the Device and Queue in `can_create_surfaces`
    ///   - Create it's own texture
    ///   - Pass the `ResourceId` of the paint for an Image in the AnyRender `Scene`
    fn paint(
        &mut self,
        render_ctx: &mut dyn RenderContext,
        styles: &ComputedStyles,
        width: u32,
        height: u32,
        scale: f64,
    ) -> Scene {
        let _ = (render_ctx, styles, width, height, scale);
        Scene::new()
    }

    /// The virtual nodes the widget declares inside its content box: targets it serves with
    /// no element of their own. None by default.
    ///
    /// Answered on demand, so an app keeps its list in step with what it paints; nothing in the
    /// engine calls it. Its reader validates each declaration. An escher addition, flagged
    /// upstreamable.
    fn virtual_nodes(&self) -> Vec<VirtualNodeDeclaration> {
        Vec::new()
    }

    // TODO: allow for multiple nodes per widget
    // fn accessibility_tree(&mut self) -> AccessKitNode;

    // TODO: simpler layout mode?
    // fn layout(&mut self, inputs: LayoutInput, styles: &ComputedStyles) -> LayoutOutput;
}

/// One virtual node a [`Widget`] declares: an author key, an accessibility role, a name and a
/// region.
///
/// The region is in CSS px of the widget's content box, its origin at the content box's top
/// left: the frame [`Widget::paint`] draws in. Its reader validates the declaration. An escher
/// addition, flagged upstreamable.
#[derive(Debug, Clone, PartialEq)]
pub struct VirtualNodeDeclaration {
    /// The node's author key, unique among the widget's declarations.
    pub key: String,
    /// The node's accessibility role.
    pub role: accesskit::Role,
    /// The node's accessible name.
    pub name: String,
    /// The node's region in the widget's content box.
    pub region: crate::BoundingRect,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CustomWidgetStatus {
    Suspended,
    Active,
    PendingRemoval,
}

pub struct CustomWidgetData {
    /// The custom widget
    pub widget: Box<dyn Widget>,
    /// The custom widget's status
    pub status: CustomWidgetStatus,
    /// The IDs of active resources
    /// (stored so that we can automatically unregister them if/when the widget is destroyed).
    pub active_resource_ids: Vec<ResourceId>,
}

impl CustomWidgetData {
    pub(crate) fn new(widget: Box<dyn Widget>) -> Self {
        Self {
            widget,
            status: CustomWidgetStatus::Suspended,
            active_resource_ids: Vec::new(),
        }
    }

    pub(crate) fn take_resource_ids(&mut self) -> Vec<ResourceId> {
        core::mem::take(&mut self.active_resource_ids)
    }
}
