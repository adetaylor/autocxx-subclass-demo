mod autocxxsubclass;
mod autogenerated;

use std::{default, ptr::null_mut};

use autocxxsubclass::{AutocxxSubclass, CppPeerHolder, make_cpp_owned};
use autogenerated::{ffi, MyWebContentsObserverDefaults};

use crate::autocxxsubclass::{make_rust_owned, make_self_owned};

pub struct SomeKindaEngine;

impl SomeKindaEngine {
    fn do_something(&self) {
        eprintln!("Hello! External Rust code responding to observer message")
    }
}

pub struct MyWebContentsObserver {
    cpp_peer: CppPeerHolder<ffi::MyWebContentsObserverCpp>,
    data: SomeKindaEngine,
}

impl AutocxxSubclass<ffi::MyWebContentsObserverCpp> for MyWebContentsObserver {
    fn get_peer(&mut self) -> &mut CppPeerHolder<ffi::MyWebContentsObserverCpp> {
        &mut self.cpp_peer
    }
}



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

pub fn make_cpp_observer() -> cxx::UniquePtr<ffi::MyWebContentsObserverCpp> {
    make_cpp_owned(MyWebContentsObserver {
        cpp_peer: Default::default(),
        data: SomeKindaEngine
    }, ffi::MyWebContentsObserverCpp::make_unique)
}

fn main() {
    let engine = SomeKindaEngine;
    make_self_owned(MyWebContentsObserver {
        cpp_peer: Default::default(),
        data: engine
    }, ffi::MyWebContentsObserverCpp::make_unique);
    let engine2 = SomeKindaEngine;
    let web_contents: *mut ffi::WebContents = null_mut(); // imagine this was real
    let my_web_contents_observing_component2 = make_rust_owned(MyWebContentsObserver {
        cpp_peer: Default::default(),
        data: engine2
    }, |rs| unsafe { ffi::MyWebContentsObserverCpp::make_unique2(rs, web_contents) });
    assert_eq!(
        web_contents,
        my_web_contents_observing_component2
            .as_ref()
            .borrow_mut()
            .web_contents()
    );
}
