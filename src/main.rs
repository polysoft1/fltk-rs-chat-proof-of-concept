use fltk::{app, enums, prelude::*, window::Window, frame, text, image, group, button};

static mut CURSOR_SHOWN: bool = true;

fn cursor_blick_callback(mut editor: text::TextEditor, handle: app::TimeoutHandle) {
    unsafe {
        if editor.has_focus() {
            editor.show_cursor(CURSOR_SHOWN);
        }
        CURSOR_SHOWN = !CURSOR_SHOWN;
    }
    app::repeat_timeout3(0.5, handle);
}

const INITIAL_WINDOW_WIDTH: i32 = 290;
const SEND_BUTTON_SIZE: i32 = 40;

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    app::background(250, 250, 253);
    app::set_font(enums::Font::Helvetica);
    app::set_frame_type(enums::FrameType::RFlatBox);
    app::set_scrollbar_size(8);
    app::set_visible_focus(false);


    let mut main_window = Window::new(100, 100, INITIAL_WINDOW_WIDTH, 400, "Rust Fltk Demo");

    // Align everything with a vpack
    let mut vpack = group::Pack::new(0, 0, INITIAL_WINDOW_WIDTH, 400, "");

    let top_bar_group = group::Group::new(0, 0, INITIAL_WINDOW_WIDTH, 50, "");
    let mut background = frame::Frame::default().with_size(INITIAL_WINDOW_WIDTH, 50);
    background.set_color(enums::Color::from_rgb(233, 233, 236));
    background.set_align(enums::Align::Bottom);
    background.set_frame(enums::FrameType::FlatBox);

    let mut title_vpack = group::Pack::new(0, 0, 265, 50, "");
    let mut top_bar_hpack = group::Pack::new(0, 0, INITIAL_WINDOW_WIDTH, 30, "");

    // Chat title
    let mut chat_title = frame::Frame::default().with_size(INITIAL_WINDOW_WIDTH, 30);
    chat_title.set_label("Title");
    chat_title.set_label_size(20);
    chat_title.set_label_font(enums::Font::HelveticaBold);

    // Settings gear
    let mut settings_button = button::Button::new(0, 0, 25, 25, "");
    let mut settings_gear_img = image::SvgImage::load("icons/settings_gear.svg").unwrap();
    settings_gear_img.scale(15, 15, true, true);
    settings_button.set_image(Some(settings_gear_img));
    settings_button.set_frame(enums::FrameType::NoBox);
    settings_button.set_align(enums::Align::ImageBackdrop);
    top_bar_hpack.end();
    top_bar_hpack.resizable(&chat_title);
    top_bar_hpack.set_type(group::PackType::Horizontal);

    // Chat description
    let mut chat_description = frame::Frame::default().with_size(100, 15);
    chat_description.set_label("This is a descriptive description.");
    chat_description.set_label_size(10);

    title_vpack.end();
    title_vpack.set_type(group::PackType::Vertical);
    top_bar_group.end();
    top_bar_hpack.resizable(&title_vpack); // The title is the part that should resize.

    // Setup chat history
    // Use a scroll group, and add text to it.

    let mut scrollable_timeline = group::Scroll::new(0, 20, INITIAL_WINDOW_WIDTH, 300, "");
    
    //scrollable_timeline.add(&text1);
    
    scrollable_timeline.end();

    // Horizontal pack to align the stuff on the bottom
    let mut bottom_hpack = group::Pack::new(0, 0, INITIAL_WINDOW_WIDTH, 50, "");
    // Setup text box
    // First the buffer that stores the text
    let mut editor_buf = text::TextBuffer::default();
    editor_buf.set_tab_distance(4);
    let mut editor = text::TextEditor::new(0, 0, INITIAL_WINDOW_WIDTH - SEND_BUTTON_SIZE, 50, "");
    editor.set_buffer(editor_buf);
    editor.set_cursor_style(text::Cursor::Simple);
    editor.wrap_mode(text::WrapMode::AtBounds, 0);
    //editor.set_frame(enums::FrameType::FlatBox);
    bottom_hpack.resizable(&editor); // The editor is the part that should resize.
    app::add_timeout3(0.5, move |handle| {
        let editor_clone = editor.clone();
        cursor_blick_callback(editor_clone, handle);
    });

    // Setup send button
    let mut send_button = button::Button::new(0, 0, SEND_BUTTON_SIZE, 30, "");
    let mut send_img = image::SvgImage::load("icons/send.svg").unwrap();
    send_img.scale(30, 30, true, true);
    send_button.set_image(Some(send_img));
    send_button.set_frame(enums::FrameType::NoBox);
    send_button.set_align(enums::Align::ImageBackdrop);
    send_button.set_shortcut(enums::Shortcut::None | enums::Key::Enter);

    // Done with bottom
    bottom_hpack.end();
    bottom_hpack.set_type(group::PackType::Horizontal);
    vpack.end();
    vpack.set_type(group::PackType::Vertical);

    // Finish window
    main_window.make_resizable(true);
    main_window.end();
    main_window.show();

    app.run().unwrap();

}
