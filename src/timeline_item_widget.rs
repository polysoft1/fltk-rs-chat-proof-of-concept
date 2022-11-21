use fltk::{frame, group, prelude::*, image, text::{self, TextBuffer}, enums};
use std::ops::{Deref, DerefMut};

#[derive(Clone)]
pub struct TimelineItemWidget {
    //profile_pic: image::Image,
    //messages: &mut Vec<text::TextBuffer>,
    text_display: text::TextDisplay,
    _background_fill: frame::Frame,
    inner_group: group::Group, // Handles the sizing of the message
    pub outer_group: group::Group, // Lets the message not take up the entire width of the pack
}

impl TimelineItemWidget {
    pub fn new(parent_width: i32, msg: &str) -> Self {
        let outer_group = group::Group::new(0, 0, parent_width, 50, "");
        let msg_group = group::Group::new(0, 0, parent_width, 50, "");
        //msg_group.set_frame(enums::FrameType::BorderFrame);
        //msg_group.set_color(enums::Color::from_rgb(200, 150, 150));
        let mut msg_bubble = frame::Frame::new(0, 0, parent_width - 30,50, "");
        // The background frame
        msg_bubble.set_frame(enums::FrameType::RFlatBox);
        msg_bubble.set_color(enums::Color::White);
        // Sender
        let mut msg_sender = frame::Frame::new(46, 0, 10, 30, "Message Title");
        msg_sender.set_label_size(13);
        msg_sender.set_label_font(enums::Font::HelveticaBold);
        // Msg body
        let mut msg_body = text::TextDisplay::new(5, 30, parent_width - 35, 10, "");
        let mut msg_body_buf = text::TextBuffer::default();
        msg_body_buf.append(msg);
        msg_body.set_buffer(msg_body_buf);
        msg_body.set_frame(enums::FrameType::FlatBox);
        msg_body.wrap_mode(text::WrapMode::AtBounds, 0);
        msg_body.set_text_size(13);
        
        //msg_body.resize(5, 30, parent_width - 32, msg_height);
        msg_group.end();
        outer_group.end();
        msg_group.resizable(&msg_body); // The outer group is resizeable

        let mut result = Self {
            text_display: msg_body,
            _background_fill: msg_bubble,
            inner_group: msg_group,
            outer_group: outer_group.clone(),
        };

        result.recalculate_size(parent_width);

        result
    }

    /// To be called on construction, and by the parent when it is resized.
    pub fn recalculate_size(&mut self, parent_width: i32) {
        if self.text_display.buffer().is_none() {
            // Set to reasonable size for an unknown size
            self.inner_group.resize(0, 0, parent_width - 30, 70);
            return;
        }
        let buf: TextBuffer = self.text_display.buffer().unwrap();
        let msg_len = buf.length();
        let num_lines = self.text_display.count_lines(0, msg_len - 3, true);
        let msg_height = num_lines * 17;
        let mut max_line_len = 1;
        let mut search_position = 0;
        loop {
            // Search for line lengths using the buffer to prevent unnecessary copying.
            let position_found = buf.find_char_forward(search_position, '\n');
            let line_len: i32 = match position_found {
                Some(pos) =>  pos - search_position,
                None => msg_len - search_position,
            };

            if line_len > max_line_len {
                max_line_len = line_len;
            }
            if position_found.is_none() {
                // Reached end
                break;
            } else {
                // Search from the correct location
                search_position = position_found.unwrap() + 1;
            }
        }
        let mut msg_width = self.text_display.col_to_x(max_line_len as f64) as i32;
        if msg_width > parent_width {
            msg_width = parent_width;
        }
        if msg_width < 50 {
            msg_width = 50;
        }
        //println!("recalculate_size called! New height: {}. New width: {}", msg_height, msg_width);
        self.outer_group.set_size(parent_width - 30, msg_height + 43);

        self.inner_group.set_size(msg_width - 30, msg_height + 43);
    }
}

impl Deref for TimelineItemWidget {
    type Target = group::Group;

    fn deref(&self) -> &Self::Target {
        &self.outer_group
    }
}

impl DerefMut for TimelineItemWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.outer_group
    }
}