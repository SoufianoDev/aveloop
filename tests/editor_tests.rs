use std::cell::RefCell;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use aveloop::application::ports::{DirEntry, ProjectSource, ProjectWriter, SourceError};
use aveloop::presentation::workspace::Workspace;
use gpui_kit::test::TestWindowExt;
use gpui_kit::{
    px, size, AppContext as _, Bounds, Focusable as _, Point, TestAppContext, WindowBounds,
    WindowOptions,
};

const SOURCE: &str = "int main(void) {\n    return 0;\n}\n";

#[cfg(target_os = "macos")]
const UNDO: &str = "cmd-z";
#[cfg(not(target_os = "macos"))]
const UNDO: &str = "ctrl-z";

#[derive(Default)]
struct FakeStore {
    files: RefCell<BTreeMap<PathBuf, String>>,
    writes: RefCell<Vec<(PathBuf, String)>>,
}

impl ProjectSource for FakeStore {
    fn canonicalize(&self, path: &Path) -> Result<PathBuf, SourceError> {
        self.files
            .borrow()
            .contains_key(path)
            .then(|| path.to_path_buf())
            .ok_or_else(|| SourceError::new(path, "not found"))
    }

    fn entries(&self, _: &Path) -> Result<Vec<DirEntry>, SourceError> {
        Ok(Vec::new())
    }

    fn read_text(&self, file: &Path) -> Result<String, SourceError> {
        self.files
            .borrow()
            .get(file)
            .cloned()
            .ok_or_else(|| SourceError::new(file, "not found"))
    }
}

impl ProjectWriter for FakeStore {
    fn write_text(&self, file: &Path, text: &str) -> Result<(), SourceError> {
        self.writes
            .borrow_mut()
            .push((file.to_path_buf(), text.to_string()));
        self.files
            .borrow_mut()
            .insert(file.to_path_buf(), text.to_string());
        Ok(())
    }
}

#[derive(Clone)]
struct Shared(Rc<FakeStore>);

impl ProjectSource for Shared {
    fn canonicalize(&self, path: &Path) -> Result<PathBuf, SourceError> {
        self.0.canonicalize(path)
    }

    fn entries(&self, directory: &Path) -> Result<Vec<DirEntry>, SourceError> {
        self.0.entries(directory)
    }

    fn read_text(&self, file: &Path) -> Result<String, SourceError> {
        self.0.read_text(file)
    }
}

impl ProjectWriter for Shared {
    fn write_text(&self, file: &Path, text: &str) -> Result<(), SourceError> {
        self.0.write_text(file, text)
    }
}

#[gpui_kit::test]
fn editor_opens_edits_saves_and_tracks_unsaved_changes(cx: &mut TestAppContext) {
    cx.update(gpui_kit::init);

    let store = Rc::new(FakeStore::default());
    let source_path = PathBuf::from("/project/main.c");
    store
        .files
        .borrow_mut()
        .insert(source_path.clone(), SOURCE.to_string());

    let bounds = Bounds {
        origin: Point::default(),
        size: size(px(900.), px(600.)),
    };
    let (window_handle, workspace) = cx.update(|cx| {
        let store = store.clone();
        gpui_kit::open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            cx,
            move |window, cx| {
                cx.new(|cx| {
                    Workspace::with_ports(
                        Box::new(Shared(store.clone())),
                        Box::new(Shared(store)),
                        window,
                        cx,
                    )
                })
            },
        )
        .expect("open test window")
    });

    cx.update_window(window_handle, |_, window, cx| {
        workspace.update(cx, |workspace, cx| {
            workspace.open_path(&source_path, window, cx)
        });
        workspace.read_with(cx, |workspace, cx| {
            assert_eq!(workspace.editor().read(cx).value().to_string(), SOURCE);
            assert_eq!(workspace.document_path(), Some(source_path.as_path()));
            assert!(!workspace.is_dirty(cx));
        });

        let handle = workspace.read_with(cx, |workspace, cx| {
            workspace.editor().read(cx).focus_handle(cx)
        });
        handle.focus(window, cx);
        window.render_frame(cx);
        window.input("// edit", cx);

        workspace.read_with(cx, |workspace, cx| {
            assert!(workspace.is_dirty(cx));
            assert_ne!(workspace.editor().read(cx).value().to_string(), SOURCE);
        });

        window.press(UNDO, cx);
        workspace.read_with(cx, |workspace, cx| {
            assert_eq!(
                workspace.editor().read(cx).value().to_string(),
                SOURCE,
                "undo should restore the saved text"
            );
            assert!(!workspace.is_dirty(cx));
        });

        window.input("edited", cx);
        let edited = workspace.read_with(cx, |workspace, cx| {
            workspace.editor().read(cx).value().to_string()
        });

        workspace.update(cx, |workspace, cx| workspace.save(window, cx));
        workspace.read_with(cx, |workspace, cx| assert!(!workspace.is_dirty(cx)));
        assert_eq!(store.writes.borrow().len(), 1);
        assert_eq!(store.writes.borrow()[0].0, source_path);
        assert_eq!(store.writes.borrow()[0].1, edited);

        let other_path = PathBuf::from("/project/other.c");
        workspace.update(cx, |workspace, cx| workspace.save_as(&other_path, cx));
        assert_eq!(store.writes.borrow().len(), 2);
        workspace.read_with(cx, |workspace, _| {
            assert_eq!(workspace.document_path(), Some(other_path.as_path()));
        });

        workspace.update(cx, |workspace, cx| workspace.save_as(Path::new(""), cx));
        workspace.update(cx, |workspace, cx| {
            workspace.open_path(Path::new("/project/missing.c"), window, cx)
        });
        workspace.read_with(cx, |workspace, _| {
            assert_eq!(workspace.document_path(), Some(other_path.as_path()));
        });
        assert_eq!(store.writes.borrow().len(), 2);
    })
    .unwrap();
}

#[gpui_kit::test]
fn untitled_document_save_does_not_write_an_implicit_path(cx: &mut TestAppContext) {
    cx.update(gpui_kit::init);

    let store = Rc::new(FakeStore::default());
    let bounds = Bounds {
        origin: Point::default(),
        size: size(px(900.), px(600.)),
    };
    let (window_handle, workspace) = cx.update(|cx| {
        let store = store.clone();
        gpui_kit::open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            cx,
            move |window, cx| {
                cx.new(|cx| {
                    Workspace::with_ports(
                        Box::new(Shared(store.clone())),
                        Box::new(Shared(store)),
                        window,
                        cx,
                    )
                })
            },
        )
        .expect("open test window")
    });

    cx.update_window(window_handle, |_, window, cx| {
        workspace.update(cx, |workspace, cx| workspace.save(window, cx));
    })
    .unwrap();

    assert!(store.writes.borrow().is_empty());
}
