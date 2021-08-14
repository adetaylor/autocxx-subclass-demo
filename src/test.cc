#include "autocxx-subclass-demo/include/test.h"
#include "autocxx-subclass-demo/src/main.rs.h"

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
