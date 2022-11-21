use fltk::{app, enums};
mod timeline_item_widget;
mod timeline;
mod chat_window;

pub mod constants;

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    app::background(236, 239, 241);
    app::set_font(enums::Font::Helvetica);
    app::set_frame_type(enums::FrameType::RFlatBox); // This is what makes everything look more modern
    app::set_frame_border_radius_max(10);
    app::set_scrollbar_size(constants::SCROLL_BAR_SIZE);
    app::set_visible_focus(false);

    let chat_window = chat_window::ChatWindow::new();

    chat_window.timeline.borrow_mut().add_messages();

    app.run().unwrap();

}
