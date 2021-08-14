
use crate::autocxxsubclass::{AutocxxSubclassHolder, AutocxxSubclassPeer};
use crate::MyWebContentsObserver;

#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("autocxx-subclass-demo/include/test.h");
        type WebContents;
        type GlobalRenderFrameHostId;
        type RenderFrameHost;
        type MyWebContentsObserverCpp;

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
        fn MyWebContentsObserverCpp_web_contents(
            self: Pin<&mut MyWebContentsObserverCpp>,
        ) -> *mut WebContents;

        fn MyWebContentsObserverCpp_make_unique(
            rs_peer: Box<MyWebContentsObserverHolder>,
        ) -> UniquePtr<MyWebContentsObserverCpp>;

        unsafe fn MyWebContentsObserverCpp_make_unique2(
            rs_peer: Box<MyWebContentsObserverHolder>,
            web_contents: *mut WebContents,
        ) -> UniquePtr<MyWebContentsObserverCpp>;

        fn MyWebContentsObserverCpp_remove_ownership(self: Pin<&mut MyWebContentsObserverCpp>);
    }

    extern "Rust" {
        type MyWebContentsObserverHolder;

        pub(crate) unsafe fn MyWebContentsObserver_RenderFrameCreated(
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
    pub fn make_unique(rs_peer: Box<MyWebContentsObserverHolder>) -> cxx::UniquePtr<Self> {
        ffi::MyWebContentsObserverCpp_make_unique(rs_peer)
    }
    pub unsafe fn make_unique2(
        rs_peer: Box<MyWebContentsObserverHolder>,
        web_contents: *mut ffi::WebContents,
    ) -> cxx::UniquePtr<Self> {
        ffi::MyWebContentsObserverCpp_make_unique2(rs_peer, web_contents)
    }
}

type MyWebContentsObserverHolder = AutocxxSubclassHolder<MyWebContentsObserver>;

impl AutocxxSubclassPeer for ffi::MyWebContentsObserverCpp {
    fn relinquish_ownership(self: std::pin::Pin<&mut Self>) {
        self.MyWebContentsObserverCpp_remove_ownership();
    }
}

#[allow(non_snake_case)]
pub fn MyWebContentsObserver_RenderFrameCreated(
    me: &MyWebContentsObserverHolder,
    render_frame_host: *mut ffi::RenderFrameHost,
) {
    if let Some(r) = me.get() {
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
    if let Some(r) = me.get() {
        r.as_ref()
            .borrow_mut()
            .RenderFrameDeleted(render_frame_host);
    }
}

#[allow(non_snake_case)]
pub fn MyWebContentsObserver_RelinquishOwnership(me: &mut MyWebContentsObserverHolder) {
    me.relinquish_ownership();
}

// Generated only where non-pure virtuals exist
#[allow(non_snake_case)]
pub(crate) trait MyWebContentsObserverDefaults {
    fn get(&mut self) -> std::pin::Pin<&mut ffi::MyWebContentsObserverCpp>;
    fn RenderFrameCreated(&mut self, render_frame_host: *mut ffi::RenderFrameHost) {
        unsafe { self.get().RenderFrameCreated_default(render_frame_host) }
    }
    fn RenderFrameDeleted(&mut self, render_frame_host: *mut ffi::RenderFrameHost) {
        unsafe { self.get().RenderFrameDeleted_default(render_frame_host) }
    }
    fn web_contents(&mut self) -> *mut ffi::WebContents {
        self.get().MyWebContentsObserverCpp_web_contents()
    }
}

impl MyWebContentsObserverDefaults for MyWebContentsObserver {
    fn get(&mut self) -> std::pin::Pin<&mut ffi::MyWebContentsObserverCpp> {
        self.get_peer()
    }
}