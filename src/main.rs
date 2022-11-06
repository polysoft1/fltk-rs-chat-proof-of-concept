use fltk::{app, enums, prelude::*, window::Window, text, image, group, button};

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


fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    app::background(250, 250, 250);
    app::set_scrollbar_size(8);

    let mut main_window = Window::new(100, 100, 280, 400, "Rust Fltk Demo");
    // Setup chat history
    // Use a scroll group, and add text to it.

    //let mut scrollable_timeline = group::Scroll::new(0, 20, 280, 330, "Scrollable");
    let mut text1 = text::TextDisplay::default();
    let mut text1buf = text::TextBuffer::default();
    text1buf.append("Item 1 Text!");
    text1.set_buffer(text1buf);
    //scrollable_timeline.add(&text1);
    
    //scrollable_timeline.end();

    // Horizontal pack to align the stuff on the bottom
    let mut hpack = group::Pack::new(0, 350, 280, 50, "");
    // Setup text box
    // First the buffer that stores the text
    let mut buf = text::TextBuffer::default();
    buf.set_tab_distance(4);
    let mut editor = text::TextEditor::new(0, 0, 240, 50, "");
    editor.set_buffer(buf.clone());
    editor.set_cursor_style(text::Cursor::Simple);
    editor.wrap_mode(text::WrapMode::AtBounds, 0);
    hpack.resizable(&editor); // The editor is the part that should resize.
    app::add_timeout3(0.5, move |handle| {
        let editor_clone = editor.clone();
        cursor_blick_callback(editor_clone, handle);
    });

    // Setup send button
    let mut send_button = button::Button::new(0, 0, 40, 40, "");
    let mut send_img = image::SvgImage::load("icons/send.svg").unwrap();
    send_img.scale(30, 30, true, true);
    send_button.set_image(Some(send_img));
    send_button.set_frame(enums::FrameType::NoBox);
    send_button.set_align(enums::Align::ImageBackdrop);

    // Done with bottom
    hpack.end();
    hpack.set_type(group::PackType::Horizontal);

    // Finish window
    main_window.make_resizable(true);
    main_window.end();
    main_window.show();

    app.run().unwrap();

}
