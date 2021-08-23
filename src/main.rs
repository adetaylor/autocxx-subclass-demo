mod autocxxsubclass;
mod autogenerated;
mod test;

use std::ptr::null_mut;

use autocxxsubclass::{AutocxxSubclass, AutocxxSubclassSelfOwned, CppPeerHolder};
use autogenerated::{WebContentsObserver, ffi};

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

impl AutocxxSubclassSelfOwned<ffi::MyWebContentsObserverCpp> for MyWebContentsObserver {}

#[allow(non_snake_case)]
impl MyWebContentsObserver {
    pub(crate) fn RenderFrameCreated(&mut self, _render_frame_host: *mut ffi::RenderFrameHost) {
        self.data.do_something();
        let _foo = self.web_contents(); // can also call methods on yourself
                                        // To release ownership from C++ side
                                        // Equivalent to 'delete this'
        self.delete_self();
    }
}

pub fn make_cpp_observer() -> cxx::UniquePtr<ffi::MyWebContentsObserverCpp> {
    MyWebContentsObserver::make_cpp_owned(
        MyWebContentsObserver {
            cpp_peer: Default::default(),
            data: SomeKindaEngine,
        },
        ffi::MyWebContentsObserverCpp::make_unique,
    )
}

fn main() {
    let engine = SomeKindaEngine;
    MyWebContentsObserver::make_self_owned(
        MyWebContentsObserver {
            cpp_peer: Default::default(),
            data: engine,
        },
        ffi::MyWebContentsObserverCpp::make_unique,
    );
    let engine2 = SomeKindaEngine;
    let web_contents: *mut ffi::WebContents = null_mut(); // imagine this was real
    let my_web_contents_observing_component2 = MyWebContentsObserver::make_rust_owned(
        MyWebContentsObserver {
            cpp_peer: Default::default(),
            data: engine2,
        },
        |rs| unsafe { ffi::MyWebContentsObserverCpp::make_unique2(rs, web_contents) },
    );
    assert_eq!(
        web_contents,
        my_web_contents_observing_component2
            .as_ref()
            .borrow_mut()
            .web_contents()
    );
    test_trait_arrangement();
}


trait A_supers {
    fn a_super(&self);
    fn b_super(&self);
}

trait A : A_supers {
    fn a(&self) {
        self.a_super()
    }
    fn b(&self) {
        self.b_super()
    }
}

struct C;

impl A_supers for C {
    fn a_super(&self) {
        println!("Calling super a");
    }
    fn b_super(&self) {
        println!("Calling super b");
    }
}

impl A for C {
    fn b(&self) {
        println!("Calling sub b")
    }
}

fn get_an_a() -> Box<dyn A> {
    Box::new(C)
}

fn test_trait_arrangement() {
    let c = C;
    c.a();
    c.b();
    let c2 = get_an_a();
    c2.a();
    c2.b();
}