use gtk::glib;

#[must_use] // To make Clippy happy
pub fn main() -> glib::ExitCode {
    mellow::init_globals();
    mellow::ui::Application::run()
}

pub fn load_css() {
    let provider = gtk::CssProvider::new();
    let css = "
        list.lyrics-list {
            background-color: transparent;
            color: #fff;
        }
    ";
    provider.load_from_bytes(&glib::Bytes::from(css.as_bytes()));
    if let Some(display) = gtk::gdk::Display::default() {
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}
