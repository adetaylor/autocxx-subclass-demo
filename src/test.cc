#include "autocxx-subclass-demo/include/test.h"
#include "autocxx-subclass-demo/src/main.rs.h"
#include "autocxx-subclass-demo/src/test.rs.h"

WebContentsObserver::WebContentsObserver() {}
WebContentsObserver::WebContentsObserver(WebContents* wc) : web_contents_(wc) {}
WebContents* WebContentsObserver::web_contents() const {
  return web_contents_;
}


WebContentsObserver::~WebContentsObserver() {}

std::unique_ptr<MyWebContentsObserverCpp> MyWebContentsObserverCpp_make_unique(rust::Box<MyWebContentsObserverHolder> obs) {
  return std::make_unique<MyWebContentsObserverCpp>(std::move(obs));
}

std::unique_ptr<MyWebContentsObserverCpp> MyWebContentsObserverCpp_make_unique2(rust::Box<MyWebContentsObserverHolder> obs, WebContents* web_contents) {
  return std::make_unique<MyWebContentsObserverCpp>(std::move(obs), web_contents);
}


void MyWebContentsObserverCpp::RenderFrameCreated(RenderFrameHost* rfh) {
  MyWebContentsObserver_RenderFrameCreated(*obs, rfh);
}
void MyWebContentsObserverCpp::RenderFrameDeleted(RenderFrameHost* rfh) {
  MyWebContentsObserver_RenderFrameDeleted(*obs, rfh);
}

void MyWebContentsObserverCpp::MyWebContentsObserverCpp_remove_ownership() {
  MyWebContentsObserver_RelinquishOwnership(*obs);
}


/// And the same for our test code


MyTestObserverCpp::MyTestObserverCpp(rust::Box<MyTestObserverHolder> _obs) : obs(std::move(_obs)) {
  mark_allocated();
}

MyTestObserverCpp::~MyTestObserverCpp() {
  mark_freed();
}


void MyTestObserverCpp::a() {
  MyTestObserver_a(*obs);
}

std::unique_ptr<MyTestObserverCpp> MyTestObserverCpp_make_unique(rust::Box<MyTestObserverHolder> _obs) {
  return std::make_unique<MyTestObserverCpp>(std::move(_obs));
}

void MyTestObserverCpp::MyTestObserverCpp_remove_ownership() {
   MyTestObserver_RelinquishOwnership(*obs);
}