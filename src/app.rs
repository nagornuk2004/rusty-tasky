use relm4::{gtk, SimpleComponent, ComponentSender, ComponentParts, prelude::*};
use gtk::prelude::*;

pub struct AppModel {}

#[derive(Debug)]
pub enum AppMsg {}

#[relm4::component(pub)]
impl SimpleComponent for AppModel {
    type Input = AppMsg;
    type Output = ();
    type Init = ();

    view! {
        #[root]
        adw::ApplicationWindow {
            set_default_size: (1000, 800),
            set_size_request: (500, 400),
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                adw::HeaderBar {
                    #[wrap(Some)]
                    set_title_widget = &gtk::Label {
                        set_markup: "<b>Rusty Tasky</b>",
                    },
                },
            }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AppModel {};
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {

        }
    }
}