use serde_json::Value;

pub trait EventHandler: Send + Sync {
    fn abort(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn afterprint(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn animationend(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn animationiteration(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn animationstart(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn beforeprint(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn beforeunload(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn blur(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn canplay(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn canplaythrough(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn change(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn click(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn contextmenu(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn copy(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn cut(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn dblclick(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn drag(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn dragend(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn dragenter(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn dragleave(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn dragover(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn dragstart(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn drop(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn durationchange(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn ended(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn error(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn focus(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn focusin(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn focusout(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn fullscreenchange(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn fullscreenerror(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn hashchange(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn input(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn invalid(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn keydown(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn keypress(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn keyup(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn load(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn loadeddata(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn loadedmetadata(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn loadstart(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn message(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn mousedown(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn mouseenter(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn mouseleave(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn mousemove(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn mouseover(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn mouseout(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn mouseup(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn mousewheel(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn offline(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn online(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn open(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn pagehide(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn pageshow(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn paste(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn pause(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn play(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn playing(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn popstate(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn progress(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn ratechange(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn resize(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn reset(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn scroll(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn search(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn seeked(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn seeking(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn select(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn show(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn stalled(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn storage(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn submit(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn suspend(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn timeupdate(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn toggle(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn touchcancel(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn touchend(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn touchmove(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn touchstart(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn transitionend(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn unload(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn volumechange(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn waiting(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn wheel(&self, _event: Value, _ctx: &razer_ws::Sender) {}
    fn eval(&self, _event: Value, _ctx: &razer_ws::Sender) {}
}
