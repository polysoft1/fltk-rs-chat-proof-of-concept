use fltk::{group, prelude::*, enums, app};
use std::ops::{Deref, DerefMut};
use std::{rc::Rc, rc::Weak, cell::RefCell};
use crate::{constants};
use crate::timeline_item_widget::TimelineItemWidget;

#[derive(Clone)]
pub struct Timeline {
    //pub scrollable: group::Scroll,
    pub scrollable: group::Scroll,
    pub pack: group::Pack,
    pub items: Vec<TimelineItemWidget>,
    //self_ptr: rc::Weak<cell::RefCell<Timeline>>,
}

impl Timeline {
    pub fn new(initial_width: i32) -> Rc<RefCell<Self>> {

        Rc::new_cyclic(|self_ptr: &Weak<RefCell<Self>>| {
            // Use a scroll group, and add text to it.
            let mut timeline_scrollable = group::Scroll::new(0, 20, initial_width, 300, "");
            timeline_scrollable.set_type(group::ScrollType::Vertical);
            // Order it with a pack.
            let mut vpack = group::Pack::default()
                .with_size(initial_width - constants::SCROLL_BAR_SIZE - 4, 300);
                //.center_of(&timeline_scrollable);

            
            vpack.resizable(&vpack);

            vpack.set_spacing(20);
            vpack.end();
            vpack.set_type(group::PackType::Vertical);
            
            timeline_scrollable.end();
            timeline_scrollable.resizable(&vpack);
            timeline_scrollable.scrollbar().set_selection_color(enums::Color::from_rgb(200, 200, 200));

            let mut vpack_clone = vpack.clone();
            let self_ptr_clone = self_ptr.clone();

            timeline_scrollable.resize_callback(move |scroll: &mut group::Scroll, x: i32, y: i32, w: i32, h: i32| {
                let timeline_option: Option<Rc<RefCell<Timeline>>> = self_ptr_clone.upgrade();
                if timeline_option.is_none() {
                    eprintln!("Timeline is none. Resize callback still called.");
                    return;
                }
                //let initial_pos_percent = scroll.yposition() as f32 / vpack_clone.height() as f32;
                vpack_clone.resize(x, y, w, vpack_clone.height());

                let cell: &RefCell<Timeline> = &timeline_option.unwrap();

                let items = &mut cell.borrow_mut().items;

                // Now resize all of the items in it
                for item in items {
                    // Check if is TimelineItemWidget
                    item.recalculate_size(w);
                }

                scroll.redraw();
                
                // Reposition
                /*if (initial_pos_percent > 0f32) {
                    scroll.scroll_to(0, (vpack_clone.height() as f32 * initial_pos_percent) as i32);
                }*/
            });

            let vpack_clone = vpack.clone();

            timeline_scrollable.handle(move |scrollable, ev| match ev {
                enums::Event::MouseWheel => {
                    //text_display.do_callback();
                    let mut scroll_amount: i32 = 8;
                    match app::event_dy() {
                        app::MouseWheel::Up => {
                            // Do leave as is
                        },
                        app::MouseWheel::Down => {
                            scroll_amount *= -1;
                        },
                        _ => {
                            scroll_amount = 0;
                        }
                    }
                    if scroll_amount != 0 {
                        let new_y = scrollable.yposition() + scroll_amount;
                        if new_y >= 0 && new_y < vpack_clone.height() - scrollable.height() + 100 {
                            scrollable.scroll_to(0, new_y);
                        }
                    }
                    true
                }
                _ => false,
            });

            RefCell::new(Timeline { scrollable: timeline_scrollable, pack: vpack, items: Vec::new()/*, self_ptr: self_ptr.clone()*/ })
        })
    }

    pub fn add_messages(&mut self) {
        let mut msg_body = String::new();
        msg_body.push_str("Start of msg.");

        for _ in 1..40 {
            msg_body.push_str(" Appended!");
            self.add_message(&msg_body);
        }
        self.add_message("This\nis\na\nnarrow\nbut\nlong\nmessage.\nHopefully\nthe\nbubble\nstays\nnarrow.");
    }

    pub fn add_message(&mut self, msg: &str) {
        let new_item = TimelineItemWidget::new(constants::INITIAL_WINDOW_WIDTH, &*msg);
        self.pack.add(&new_item.outer_group);
        self.items.push(new_item);
    }
}


impl Deref for Timeline {
    type Target = group::Scroll;

    fn deref(&self) -> &Self::Target {
        &self.scrollable
    }
}

impl DerefMut for Timeline {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.scrollable
    }
}