//! A remote page reads no local file through the engine's sub-resource fetches.
//!
//! Every request blitz-dom builds carries the URL of the document that loads it as its
//! `initiator`, and blitz-net's `file:` branch serves a `file:` URL only to a `file:`
//! document or to a request no document made. So an `http:` page's `file:` stylesheet,
//! `@import`, image and iframe read nothing from disk, while the same page as a `file:`
//! document reads them all.

use blitz_dom::BaseDocument;
use blitz_test_harness::{Harness, HarnessOptions};
use blitz_traits::net::{Bytes, NetHandler, NetProvider, Request, Url};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

const REMOTE_PAGE: &str = "http://remote.test/page.html";

/// Records each request's URL and initiator; answers a stylesheet with an empty sheet, so
/// a linked one is applied and can be reloaded, and leaves every other request in flight.
#[derive(Default)]
struct RecordingNet {
    requests: Mutex<Vec<(Url, Option<Url>)>>,
}

impl NetProvider for RecordingNet {
    fn fetch(&self, _doc_id: usize, request: Request, handler: Box<dyn NetHandler>) {
        let url = request.url.clone();
        self.requests
            .lock()
            .unwrap()
            .push((request.url, request.initiator));
        if url.path().ends_with(".css") {
            handler.bytes(url.to_string(), Bytes::new());
        }
    }
}

/// Hands each request to blitz-net's real provider on a runtime of its own, in the
/// caller's thread, and answers the handler with what it returned.
struct ReadThrough {
    runtime: tokio::runtime::Runtime,
    provider: blitz_net::Provider,
}

impl ReadThrough {
    fn new() -> Self {
        Self {
            runtime: tokio::runtime::Builder::new_current_thread()
                .build()
                .unwrap(),
            provider: blitz_net::Provider::new(None),
        }
    }
}

impl NetProvider for ReadThrough {
    fn fetch(&self, _doc_id: usize, request: Request, handler: Box<dyn NetHandler>) {
        let url = request.url.to_string();
        match self.runtime.block_on(self.provider.fetch_async(request)) {
            Ok((resolved, bytes)) => handler.bytes(resolved, bytes),
            Err(error) => handler.error(url, error.to_string()),
        }
    }
}

fn file_name(url: &Url) -> String {
    url.path_segments()
        .and_then(|mut segments| segments.next_back())
        .unwrap_or_default()
        .to_string()
}

#[test]
fn every_engine_request_carries_its_document_as_initiator() {
    let link = "file:///stamp/link.css";
    let html = format!(
        r#"<!doctype html>
<html>
<head>
<style>
@import url("file:///stamp/import.css");
@font-face {{ font-family: probe; src: url("file:///stamp/font.ttf") format("truetype"); }}
#bg {{ width: 10px; height: 10px; background-image: url("file:///stamp/bg.png"); }}
</style>
</head>
<body>
<link rel="stylesheet" href="{link}">
<div id="bg"></div>
<img src="file:///stamp/img.png">
<iframe src="file:///stamp/frame.html"></iframe>
</body>
</html>"#
    );
    let document = Url::parse(REMOTE_PAGE).unwrap();
    for incremental in [false, true] {
        let net = Arc::new(RecordingNet::default());
        let mut harness = Harness::from_html_with(
            &html,
            HarnessOptions {
                base_url: Some(REMOTE_PAGE.to_string()),
                net_provider: Some(net.clone()),
                incremental: Some(incremental),
                ..Default::default()
            },
        );
        harness.pump();
        assert_eq!(harness.base().url().as_str(), REMOTE_PAGE);
        assert!(
            harness.base().author_stylesheets().count() >= 2,
            "the linked sheet was applied, so it can be reloaded (incremental: {incremental})"
        );

        harness.base_mut().reload_resource_by_href(link);
        harness.pump();

        let requests = net.requests.lock().unwrap();
        let mut kinds: Vec<String> = requests.iter().map(|(url, _)| file_name(url)).collect();
        kinds.sort();
        let expected = [
            "bg.png",
            "font.ttf",
            "frame.html",
            "img.png",
            "import.css",
            "link.css",
            "link.css",
        ];
        assert_eq!(expected.len(), 7);
        assert_eq!(kinds, expected, "incremental: {incremental}");
        for (url, initiator) in requests.iter() {
            assert_eq!(
                initiator.as_ref(),
                Some(&document),
                "{} (incremental: {incremental})",
                file_name(url)
            );
        }
    }
}

/// The fixture files, written under the test target's own temporary directory.
struct Fixture {
    dir: PathBuf,
}

impl Fixture {
    fn write() -> Self {
        let dir = PathBuf::from(concat!(
            env!("CARGO_TARGET_TMPDIR"),
            "/local_files_for_local_documents"
        ));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("probe.css"), "#probe { width: 37px; }").unwrap();
        std::fs::write(dir.join("imported.css"), "#probe { height: 23px; }").unwrap();
        std::fs::write(
            dir.join("frame.html"),
            "<!doctype html><html><body><p>frame</p></body></html>",
        )
        .unwrap();
        let mut png = Vec::new();
        {
            let mut encoder = png::Encoder::new(&mut png, 2, 3);
            encoder.set_color(png::ColorType::Rgba);
            encoder.set_depth(png::BitDepth::Eight);
            let mut writer = encoder.write_header().unwrap();
            writer.write_image_data(&[0x80; 2 * 3 * 4]).unwrap();
        }
        std::fs::write(dir.join("probe.png"), png).unwrap();
        Self { dir }
    }

    fn url(&self, name: &str) -> Url {
        Url::from_file_path(self.dir.join(name)).unwrap()
    }

    fn page(&self) -> String {
        format!(
            r#"<!doctype html>
<html>
<head>
<link rel="stylesheet" href="{css}">
<style>@import url("{imported}");</style>
</head>
<body>
<div id="probe"></div>
<img id="img" src="{png}">
<iframe id="frame" src="{frame}"></iframe>
</body>
</html>"#,
            css = self.url("probe.css"),
            imported = self.url("imported.css"),
            png = self.url("probe.png"),
            frame = self.url("frame.html"),
        )
    }
}

/// What the settled page read of its four local resources.
#[derive(Debug, PartialEq)]
struct Readings {
    width_37: bool,
    height_23: bool,
    image: Option<(u32, u32)>,
    sub_document: bool,
}

fn readings(harness: &Harness<blitz_html::HtmlDocument>) -> Readings {
    let probe = harness.layout_rect("#probe");
    let doc = harness.base();
    let element = |id: &str| {
        let doc: &BaseDocument = &doc;
        doc.get_element_by_id(id)
            .and_then(|node_id| doc.get_node(node_id))
            .and_then(|node| node.element_data())
    };
    Readings {
        width_37: probe.width == 37.0,
        height_23: probe.height == 23.0,
        image: element("img")
            .and_then(|element| element.raster_image_data())
            .map(|image| (image.width, image.height)),
        sub_document: element("frame").is_some_and(|element| element.sub_doc_data().is_some()),
    }
}

fn settled(base_url: &Url, html: &str, incremental: bool) -> Readings {
    let mut harness = Harness::from_html_with(
        html,
        HarnessOptions {
            base_url: Some(base_url.to_string()),
            net_provider: Some(Arc::new(ReadThrough::new())),
            incremental: Some(incremental),
            ..Default::default()
        },
    );
    assert!(harness.settle().is_ok(), "incremental: {incremental}");
    readings(&harness)
}

#[test]
fn a_remote_page_reads_no_local_file_and_a_file_page_reads_them_all() {
    let fixture = Fixture::write();
    let html = fixture.page();
    let file_page = fixture.url("page.html");
    let remote_page = Url::parse(REMOTE_PAGE).unwrap();
    assert!(fixture.dir.join("probe.css").is_file());
    assert!(fixture.dir.join("imported.css").is_file());
    assert!(fixture.dir.join("probe.png").is_file());
    assert!(fixture.dir.join("frame.html").is_file());

    for incremental in [false, true] {
        assert_eq!(
            settled(&file_page, &html, incremental),
            Readings {
                width_37: true,
                height_23: true,
                image: Some((2, 3)),
                sub_document: true,
            },
            "a file: page reads its local files (incremental: {incremental})"
        );
        assert_eq!(
            settled(&remote_page, &html, incremental),
            Readings {
                width_37: false,
                height_23: false,
                image: None,
                sub_document: false,
            },
            "a remote page reads none (incremental: {incremental})"
        );
    }
}
