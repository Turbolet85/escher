DioxusLabs/blitz issue #1578, read by `gh issue view 1578 -R DioxusLabs/blitz` on 2026-11-23 (state OPEN, 0 comments; body first 1500 characters):

Title: Custom widget paint receives border-box dimensions but is appended at a content-box origin in device axes

While integrating a Canvas custom widget on beta.4, the size passed to `Widget::paint` and the placement of the returned scene used different box/coordinate conventions. Both operations remain visible on the inspected main revision:

- [process_custom_widget_node](https://github.com/DioxusLabs/blitz/blob/beaa796f4ea698a738d7861aba81553563bfbc0b/packages/blitz-paint/src/lib.rs#L119) passes `final_layout().size * scale` (border-box dimensions).
- [draw_custom_widget](https://github.com/DioxusLabs/blitz/blob/beaa796f4ea698a738d7861aba81553563bfbc0b/packages/blitz-paint/src/render.rs#L1399) appends at `content_box().origin()` using `self.transform.then_translate(...)` (translation after the element transform).

A minimal diagnostic setup is a `Widget` whose `paint` logs its width/height and returns an opaque rectangle from `(0,0)` to those supplied dimensions, attached to:

```html
<canvas style="box-sizing:content-box;width:100px;height:60px;
  padding:12px;border:3px solid black;
  transform-origin:0 0;transform:rotate(30deg) scale(1.4,.8)"></canvas>
```

At scale 1 the current callback size is 130 x 90, while the append origin is the content origin (15,15). A rectangle of that callback size starting there extends into/beyond the content box. Separately, the `(15,15)` offset is added in output axes, so it does not rotate/scale with the box. The expected placement for a content scene is `M * T(content_origin)`, not `T(content_origin) * M`.

Our downstream correcti[…cut at 1500 characters]
