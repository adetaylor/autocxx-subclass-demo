#pragma once
#include "rust/cxx.h"
#include <memory>
#include <string>
#include <cstdint>

typedef int GlobalRenderFrameHostId;

class WebContents {
  std::string uri;
};

class RenderFrameHost {
  public:
        std::unique_ptr<std::string> GetLastCommittedURL()  const;
        void SaveImageAt(uint32_t x, uint32_t y);
        static RenderFrameHost* FromId(const GlobalRenderFrameHostId& id);
};

class WebContentsObserver {
   public:
  // Frames and Views ----------------------------------------------------------

  // Called when a RenderFrame for |render_frame_host| is created in the
  // renderer process. Use |RenderFrameDeleted| to listen for when this
  // RenderFrame goes away.
  virtual void RenderFrameCreated(RenderFrameHost*) {}

  // Called when a RenderFrame for |render_frame_host| is deleted or the
  // renderer process in which it runs it has died. Use |RenderFrameCreated| to
  // listen for when RenderFrame objects are created.
  virtual void RenderFrameDeleted(RenderFrameHost*) {}

    WebContents* web_contents() const;

 protected:
  // Use this constructor when the object is tied to a single WebContents for
  // its entire lifetime.
  explicit WebContentsObserver(WebContents* web_contents);

  // Use this constructor when the object wants to observe a WebContents for
  // part of its lifetime.  It can then call Observe() to start and stop
  // observing.
  WebContentsObserver();

  virtual ~WebContentsObserver();

 private:

  WebContents* web_contents_ = nullptr;

};

RenderFrameHost* RenderFrameHost_FromId(const GlobalRenderFrameHostId& id) {
  return RenderFrameHost::FromId(id);
}


struct MyWebContentsObserverHolder; // Rust type

class MyWebContentsObserverCpp : public WebContentsObserver {
public:
  MyWebContentsObserverCpp(rust::Box<MyWebContentsObserverHolder> _obs) : obs(std::move(_obs)) {}
  MyWebContentsObserverCpp(rust::Box<MyWebContentsObserverHolder> _obs, WebContents* web_contents) : WebContentsObserver(web_contents), obs(std::move(_obs)) {}
  void RenderFrameCreated_default(RenderFrameHost* rfh) {
    WebContentsObserver::RenderFrameCreated(rfh);
  }
  void RenderFrameDeleted_default(RenderFrameHost* rfh) {
    WebContentsObserver::RenderFrameDeleted(rfh);
  }
  WebContents* MyWebContentsObserverCpp_web_contents() {
    return web_contents();
  }
  virtual void RenderFrameCreated(RenderFrameHost* rfh);
  virtual void RenderFrameDeleted(RenderFrameHost* rfh);
  void MyWebContentsObserverCpp_remove_ownership();
  rust::Box<MyWebContentsObserverHolder> obs;
};



std::unique_ptr<MyWebContentsObserverCpp> MyWebContentsObserverCpp_make_unique(rust::Box<MyWebContentsObserverHolder> obs);
std::unique_ptr<MyWebContentsObserverCpp> MyWebContentsObserverCpp_make_unique2(rust::Box<MyWebContentsObserverHolder> obs, WebContents* web_contents);