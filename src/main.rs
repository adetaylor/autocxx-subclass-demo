mod autocxxsubclass;
mod autogenerated;

use std::ptr::null_mut;

use autocxxsubclass::AutocxxSubclass;
use autogenerated::{ffi, MyWebContentsObserverDefaults};

pub struct SomeKindaEngine;

impl SomeKindaEngine {
    fn do_something(&self) {
        eprintln!("Hello! External Rust code responding to observer message")
    }
}

type MyWebContentsObserver = AutocxxSubclass<ffi::MyWebContentsObserverCpp, SomeKindaEngine>;

#[allow(non_snake_case)]
impl MyWebContentsObserver {
    pub(crate) fn RenderFrameCreated(&mut self, _render_frame_host: *mut ffi::RenderFrameHost) {
        self.data.do_something();
        let foo = self.web_contents(); // can also call methods on yourself
                                       // To release ownership from C++ side
                                       // Equivalent to 'delete this'
        self.delete_self();
    }
}

fn make_cpp_observer() -> cxx::UniquePtr<ffi::MyWebContentsObserverCpp> {
    MyWebContentsObserver::new_for_cpp(ffi::MyWebContentsObserverCpp::make_unique, SomeKindaEngine)
}

fn main() {
    let engine = SomeKindaEngine;
    MyWebContentsObserver::new_self_owned(ffi::MyWebContentsObserverCpp::make_unique, engine);
    let engine2 = SomeKindaEngine;
    let web_contents: *mut ffi::WebContents = null_mut(); // imagine this was real
    let my_web_contents_observing_component2 = MyWebContentsObserver::new(
        |rs| unsafe { ffi::MyWebContentsObserverCpp::make_unique2(rs, web_contents) },
        engine2,
    );
    assert_eq!(
        web_contents,
        my_web_contents_observing_component2
            .as_ref()
            .borrow_mut()
            .web_contents()
    );
}
