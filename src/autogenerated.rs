use crate::autocxxsubclass::{AutocxxSubclass, AutocxxSubclassHolder, AutocxxSubclassPeer};
use crate::MyWebContentsObserver;



#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("autocxx-subclass-demo/include/test.h");
        type WebContents;
        type GlobalRenderFrameHostId;
        type RenderFrameHost;
        type MyWebContentsObserverCpp; // 1

        fn GetLastCommittedURL(self: &RenderFrameHost) -> UniquePtr<CxxString>;
        fn SaveImageAt(self: Pin<&mut RenderFrameHost>, x: u32, y: u32);
        //fn RenderFrameHost_FromId(id: &GlobalRenderFrameHostId) -> *mut RenderFrameHost;

        unsafe fn RenderFrameCreated_default(
            self: Pin<&mut MyWebContentsObserverCpp>,
            render_frame_host: *mut RenderFrameHost,
        );

        unsafe fn RenderFrameDeleted_default(
            self: Pin<&mut MyWebContentsObserverCpp>,
            render_frame_host: *mut RenderFrameHost,
        );
        fn MyWebContentsObserverCpp_web_contents( // 2a
            self: Pin<&mut MyWebContentsObserverCpp>,
        ) -> *mut WebContents;

        fn MyWebContentsObserverCpp_make_unique( // 1a
            rs_peer: Box<MyWebContentsObserverHolder>,
        ) -> UniquePtr<MyWebContentsObserverCpp>;

        unsafe fn MyWebContentsObserverCpp_make_unique2(
            rs_peer: Box<MyWebContentsObserverHolder>,
            web_contents: *mut WebContents,
        ) -> UniquePtr<MyWebContentsObserverCpp>;

        fn MyWebContentsObserverCpp_remove_ownership(self: Pin<&mut MyWebContentsObserverCpp>);
    }

    extern "Rust" {
        type MyWebContentsObserverHolder;   // 1

        pub(crate) unsafe fn MyWebContentsObserver_RenderFrameCreated( // 1a
            me: &MyWebContentsObserverHolder,
            render_frame_host: *mut RenderFrameHost,
        );
        pub(crate) unsafe fn MyWebContentsObserver_RenderFrameDeleted(
            me: &MyWebContentsObserverHolder,
            render_frame_host: *mut RenderFrameHost,
        );
        pub(crate) fn MyWebContentsObserver_RelinquishOwnership(
            me: &mut MyWebContentsObserverHolder,
        );
    }

    impl UniquePtr<MyWebContentsObserverCpp> {}
}

impl ffi::MyWebContentsObserverCpp {
    pub fn make_unique(rs_peer: AutocxxSubclassHolder<MyWebContentsObserver>) -> cxx::UniquePtr<Self> { // 1a // change
        ffi::MyWebContentsObserverCpp_make_unique(Box::new(MyWebContentsObserverHolder(rs_peer)))
    }
    pub unsafe fn make_unique2(
        rs_peer: AutocxxSubclassHolder<MyWebContentsObserver>,
        web_contents: *mut ffi::WebContents,
    ) -> cxx::UniquePtr<Self> {
        ffi::MyWebContentsObserverCpp_make_unique2(Box::new(MyWebContentsObserverHolder(rs_peer)), web_contents)
    }
}

pub struct MyWebContentsObserverHolder(pub AutocxxSubclassHolder<MyWebContentsObserver>); // 1

impl AutocxxSubclassPeer for ffi::MyWebContentsObserverCpp {  // 1a, just this line
    fn relinquish_ownership(self: std::pin::Pin<&mut Self>) {
        self.MyWebContentsObserverCpp_remove_ownership();
    }
}

#[allow(non_snake_case)]
pub fn MyWebContentsObserver_RenderFrameCreated( // 1a
    me: &MyWebContentsObserverHolder,
    render_frame_host: *mut ffi::RenderFrameHost,
) {
    if let Some(r) = me.0.get() {
        r.as_ref()
            .borrow_mut()
            .RenderFrameCreated(render_frame_host);
    }
}

#[allow(non_snake_case)]
pub fn MyWebContentsObserver_RenderFrameDeleted(
    me: &MyWebContentsObserverHolder,
    render_frame_host: *mut ffi::RenderFrameHost,
) {
    if let Some(r) = me.0.get() {
        r.as_ref()
            .borrow_mut()
            .RenderFrameDeleted(render_frame_host);
    }
}

#[allow(non_snake_case)]
pub fn MyWebContentsObserver_RelinquishOwnership(me: &mut MyWebContentsObserverHolder) {
    me.0.relinquish_ownership();
}

// Generated only where non-pure virtuals exist
#[allow(non_snake_case)]
pub(crate) trait MyWebContentsObserverDefaults: AutocxxSubclass<ffi::MyWebContentsObserverCpp> {  // 2a
    fn RenderFrameCreated(&mut self, render_frame_host: *mut ffi::RenderFrameHost) {
        unsafe { self.pin_peer().RenderFrameCreated_default(render_frame_host) }
    }
    fn RenderFrameDeleted(&mut self, render_frame_host: *mut ffi::RenderFrameHost) {
        unsafe { self.pin_peer().RenderFrameDeleted_default(render_frame_host) }
    }
    fn web_contents(&mut self) -> *mut ffi::WebContents {  // 2a
        self.pin_peer().MyWebContentsObserverCpp_web_contents()
    }
}

impl MyWebContentsObserverDefaults for MyWebContentsObserver {}
