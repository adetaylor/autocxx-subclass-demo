use std::{
    cell::RefCell,
    pin::Pin,
    rc::{Rc, Weak},
};

use cxx::{memory::UniquePtrTarget, UniquePtr};

pub trait AutocxxSubclassPeer: UniquePtrTarget {
    fn relinquish_ownership(self: Pin<&mut Self>);
}

pub enum AutocxxSubclassHolder<T> {
    Owned(Rc<RefCell<T>>),
    Unowned(Weak<RefCell<T>>),
}

impl<T> AutocxxSubclassHolder<T> {
    pub fn get(&self) -> Option<Rc<RefCell<T>>> {
        match self {
            AutocxxSubclassHolder::Owned(strong) => Some(strong.clone()),
            AutocxxSubclassHolder::Unowned(weak) => weak.upgrade(),
        }
    }
    pub fn relinquish_ownership(&mut self) {
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

impl<CppPeer: AutocxxSubclassPeer> Default for CppPeerHolder<CppPeer> {
    fn default() -> Self {
        CppPeerHolder::Empty
    }
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
        *self = Self::Unowned(unsafe {
            std::pin::Pin::<&mut CppPeer>::into_inner_unchecked(peer.pin_mut())
        });
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
pub trait AutocxxSubclass<CppPeer: AutocxxSubclassPeer> {
    fn get_peer(&mut self) -> &mut CppPeerHolder<CppPeer>;

    /// Return a reference to the C++ part of this object pair.
    /// This can be used to register listeners, etc.
    fn pin_peer(&mut self) -> Pin<&mut CppPeer> {
        self.get_peer().pin_mut()
    }

    /// Relinquishes ownership from the C++ side. If there are no outstanding
    /// references from the Rust side, this will result in the destruction
    /// of this subclass instance. TODO - make this available at compile
    /// time only in the `new_self_owned` case.
    fn delete_self(&mut self) {
        self.pin_peer().relinquish_ownership()
    }

    /// Creates a new instance of this subclass. This instance is owned by the
    /// returned [`cxx::UniquePtr`] and is thus suitable to be passed around
    /// in C++.
    fn make_cpp_owned<PeerConstructor, Subclass>(
        me: Subclass,
        peer_constructor: PeerConstructor,
    ) -> UniquePtr<CppPeer>
    where
        Subclass: AutocxxSubclass<CppPeer>,
        PeerConstructor: FnOnce(Box<AutocxxSubclassHolder<Subclass>>) -> UniquePtr<CppPeer>,
    {
        let me = Rc::new(RefCell::new(me));
        let holder = Box::new(AutocxxSubclassHolder::Owned(me.clone()));
        let mut cpp_side = peer_constructor(holder);
        me.as_ref()
            .borrow_mut()
            .get_peer()
            .set_unowned(&mut cpp_side);
        cpp_side
    }

    fn make_owning_peer<PeerConstructor, Subclass, PeerBoxer>(
        me: Subclass,
        peer_constructor: PeerConstructor,
        peer_boxer: PeerBoxer,
    ) -> Rc<RefCell<Subclass>>
    where
        Subclass: AutocxxSubclass<CppPeer>,
        PeerConstructor: FnOnce(Box<AutocxxSubclassHolder<Subclass>>) -> UniquePtr<CppPeer>,
        PeerBoxer: FnOnce(Rc<RefCell<Subclass>>) -> AutocxxSubclassHolder<Subclass>,
    {
        let me = Rc::new(RefCell::new(me));
        let holder = Box::new(peer_boxer(me.clone()));
        let cpp_side = peer_constructor(holder);
        me.as_ref().borrow_mut().get_peer().set_owned(cpp_side);
        me
    }

    /// Creates a new instance of this subclass. This instance is not owned
    /// by C++, and therefore will be deleted when it goes out of scope in
    /// Rust.
    fn make_rust_owned<PeerConstructor, Subclass>(
        me: Subclass,
        peer_constructor: PeerConstructor,
    ) -> Rc<RefCell<Subclass>>
    where
        Subclass: AutocxxSubclass<CppPeer>,
        PeerConstructor: FnOnce(Box<AutocxxSubclassHolder<Subclass>>) -> UniquePtr<CppPeer>,
    {
        Self::make_owning_peer(me, peer_constructor, |me| {
            AutocxxSubclassHolder::Unowned(Rc::downgrade(&me))
        })
    }

    /// Creates a new instance of this subclass which owns itself.
    /// This is useful
    /// for observers (etc.) which self-register to listen to events.
    /// If an event occurs which would cause this to want to unregister,
    /// use [`AutocxxSubclass::delete_self`].
    /// The return value may be useful to register this, etc. but can ultimately
    /// be discarded without destroying this object.
    fn make_self_owned<PeerConstructor, Subclass>(
        me: Subclass,
        peer_constructor: PeerConstructor,
    ) -> Rc<RefCell<Subclass>>
    where
        CppPeer: AutocxxSubclassPeer,
        Subclass: AutocxxSubclass<CppPeer>,
        PeerConstructor: FnOnce(Box<AutocxxSubclassHolder<Subclass>>) -> UniquePtr<CppPeer>,
    {
        Self::make_owning_peer(me, peer_constructor, AutocxxSubclassHolder::Owned)
    }
}
