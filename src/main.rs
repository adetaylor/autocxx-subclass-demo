use cxx::{private::UniquePtrTarget, UniquePtr};
use std::{cell::RefCell, pin::Pin, ptr::null_mut, rc::{Rc, Weak}};

#[cxx::bridge]
mod ffi {
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
    fn make_unique(rs_peer: Box<MyWebContentsObserverHolder>) -> cxx::UniquePtr<Self> {
        ffi::MyWebContentsObserverCpp_make_unique(rs_peer)
    }
    unsafe fn make_unique2(
        rs_peer: Box<MyWebContentsObserverHolder>,
        web_contents: *mut ffi::WebContents,
    ) -> cxx::UniquePtr<Self> {
        ffi::MyWebContentsObserverCpp_make_unique2(rs_peer, web_contents)
    }
}

type MyWebContentsObserverHolder = AutocxxSubclassHolder<MyWebContentsObserver>;

impl AutocxxSubclassPeer for ffi::MyWebContentsObserverCpp {
    fn relinquish_ownership(self: Pin<&mut Self>) {
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
    fn get(&mut self) -> Pin<&mut ffi::MyWebContentsObserverCpp>;
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
    fn get(&mut self) -> Pin<&mut ffi::MyWebContentsObserverCpp> {
        self.get_peer()
    }
}

/////////////////
// Try to abstract the stuff below
//////////////

pub trait AutocxxSubclassPeer: UniquePtrTarget {
    fn relinquish_ownership(self: Pin<&mut Self>);
}

pub enum AutocxxSubclassHolder<T> {
    Owned(Rc<RefCell<T>>),
    Unowned(Weak<RefCell<T>>),
}

impl<T> AutocxxSubclassHolder<T> {
    fn get(&self) -> Option<Rc<RefCell<T>>> {
        match self {
            AutocxxSubclassHolder::Owned(strong) => Some(strong.clone()),
            AutocxxSubclassHolder::Unowned(weak) => weak.upgrade(),
        }
    }
    fn relinquish_ownership(&mut self) {
        if let AutocxxSubclassHolder::Owned(strong) = self {
            *self = AutocxxSubclassHolder::Unowned(Rc::downgrade(strong))
        }
    }
}

pub enum CppPeerHolder<CppPeer: AutocxxSubclassPeer> {
    Empty,
    Owned(Box<UniquePtr<CppPeer>>),
    Unowned(*mut CppPeer),
}

impl<CppPeer: AutocxxSubclassPeer> CppPeerHolder<CppPeer> {
    fn pin_mut(&mut self) -> Pin<&mut CppPeer> {
        match self {
            CppPeerHolder::Empty => panic!("Peer not set up"),
            CppPeerHolder::Owned(peer) => peer.pin_mut(),
            CppPeerHolder::Unowned(peer) => unsafe { Pin::new_unchecked(peer.as_mut().unwrap()) },
        }
    }
    fn set_owned(&mut self, peer: UniquePtr<CppPeer>) {
        *self = Self::Owned(Box::new(peer));
    }
    fn set_unowned(&mut self, peer: &mut UniquePtr<CppPeer>) {
        *self = Self::Unowned(unsafe { std::pin::Pin::<&mut CppPeer>::into_inner_unchecked(peer.pin_mut())} );
    }
}

/// A subclass of a C++ type.
///
/// This actually consists of two objects: this object itself and a C++-side
/// peer. The ownership relationship between those two things can work in three
/// different ways:
/// 1. Neither object is owned by Rust. The C++ peer is owned by a C++
///    [`UniquePtr`] held elsewhere in C++. That C++ peer then owns
///    this Rust-side object via a strong [`Rc`] reference. This is the
///    ownership relationship set up by [`AutocxxSubclass::new_for_cpp`].
/// 2. The object pair is owned by Rust. Specifically, by a strong
///    [`Rc`] reference to this Rust-side object. In turn, the Rust-side object
///    owns the C++-side peer via a [`UniquePtr`]. This is what's set up by
///    [`AutocxxSubclass::new`]. The C++-side peer _does not_ own the Rust
///    object; it just has a weak pointer. (Otherwise we'd get a reference)
///    loop and nothing would ever be freed.
/// 3. The object pair is self-owned and will stay around forever until
///    [`delete_self`] is called. In this case there's a strong reference
///    from the C++ to the Rust and from the Rust to the C++. This is useful
///    for cases where the subclass is listening for events, and needs to
///    stick around until a particular event occurs then delete itself.
pub struct AutocxxSubclass<CppPeer: AutocxxSubclassPeer, T: Unpin> {
    cpp_peer: CppPeerHolder<CppPeer>,
    data: T,
}

impl<CppPeer: AutocxxSubclassPeer, T: Unpin> AutocxxSubclass<CppPeer, T> {
    /// Return a reference to the C++ part of this object pair.
    /// This can be used to register listeners, etc.
    pub fn get_peer(&mut self) -> Pin<&mut CppPeer> {
        self.cpp_peer.pin_mut()
    }

    fn new_owning_peer<PeerConstructor, F>(
        f: PeerConstructor,
        data: T,
        boxer: F,
    ) -> Rc<RefCell<Self>>
    where
        PeerConstructor: FnOnce(Box<AutocxxSubclassHolder<Self>>) -> cxx::UniquePtr<CppPeer>,
        F: FnOnce(Rc<RefCell<AutocxxSubclass<CppPeer, T>>>) -> AutocxxSubclassHolder<Self>,
    {
        let me = Rc::new(RefCell::new(AutocxxSubclass {
            cpp_peer: CppPeerHolder::Empty,
            data,
        }));
        let holder = Box::new(boxer(me.clone()));
        let cpp_side = f(holder);
        me.as_ref().borrow_mut().cpp_peer.set_owned(cpp_side);
        me
    }

    /// Creates a new instance of this subclass. This instance is owned by the
    /// returned [`cxx::UniquePtr`] and is thus suitable to be passed around
    /// in C++.
    #[must_use]
    pub fn new_for_cpp<PeerConstructor>(f: PeerConstructor, data: T) -> UniquePtr<CppPeer>
    where
        PeerConstructor: FnOnce(Box<AutocxxSubclassHolder<Self>>) -> cxx::UniquePtr<CppPeer>,
    {
        let me = Rc::new(RefCell::new(AutocxxSubclass {
            cpp_peer: CppPeerHolder::Empty,
            data,
        }));
        let holder = Box::new(AutocxxSubclassHolder::Owned(me.clone()));
        let mut cpp_side = f(holder);
        me.as_ref().borrow_mut().cpp_peer.set_unowned(&mut cpp_side);
        cpp_side
    }

    /// Relinquishes ownership from the C++ side. If there are no outstanding
    /// references from the Rust side, this will result in the destruction
    /// of this subclass instance. TODO - make this available at compile
    /// time only in the `new_self_owned` case.
    pub fn delete_self(&mut self) {
        self.get_peer().relinquish_ownership()
    }

    /// Creates a new instance of this subclass which owns itself.
    /// This is useful
    /// for observers (etc.) which self-register to listen to events.
    /// If an event occurs which would cause this to want to unregister,
    /// use [`AutocxxSubclass::delete_self`].
    /// The return value may be useful to register this, etc. but can ultimately
    /// be discarded without destroying this object.
    pub fn new_self_owned<PeerConstructor>(f: PeerConstructor, data: T) -> Rc<RefCell<Self>>
    where
        PeerConstructor: FnOnce(Box<AutocxxSubclassHolder<Self>>) -> cxx::UniquePtr<CppPeer>,
    {
        Self::new_owning_peer(f, data, AutocxxSubclassHolder::Owned)
    }

    /// Creates a new instance of this subclass. This instance is not owned
    /// by C++, and therefore will be deleted when it goes out of scope in
    /// Rust.
    #[must_use]
    pub fn new<PeerConstructor>(f: PeerConstructor, data: T) -> Rc<RefCell<Self>>
    where
        PeerConstructor: FnOnce(Box<AutocxxSubclassHolder<Self>>) -> cxx::UniquePtr<CppPeer>,
    {
        Self::new_owning_peer(f, data, |me| {
            AutocxxSubclassHolder::Unowned(Rc::downgrade(&me))
        })
    }
}

/////////////////
// Manual....
//////////////

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
