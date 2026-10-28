pub(crate) mod components;

use gtk::prelude::*;
use relm4::{
    ComponentParts, ComponentSender, SimpleComponent,
    gtk::{self},
    prelude::*,
};

use components::form::FormModel;

use crate::app::components::form::FormInput;

pub struct AppModel {
    form: Controller<FormModel>,
}

#[derive(Debug)]
pub enum AppMsg {
    NewTask,
}

#[relm4::component(pub)]
impl SimpleComponent for AppModel {
    type Input = AppMsg;
    type Output = ();
    type Init = Vec<String>;

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
                    pack_start = &gtk::Button {
                        set_icon_name: "list-add-symbolic",
                        set_tooltip_text: Some("Add Item"),
                        connect_clicked[sender] => move |_| {sender.input(AppMsg::NewTask)}
                    }
                },
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AppModel {
            form: FormModel::builder().launch(root.clone()).detach(),
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppMsg::NewTask => self.form.sender().send(FormInput::Show).unwrap(),
        }
    }
}
