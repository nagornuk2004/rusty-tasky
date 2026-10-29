pub(crate) mod components;

use gtk::prelude::*;
use relm4::{
    ComponentParts, ComponentSender, SimpleComponent,
    gtk::{self},
    prelude::*,
};

use components::form::FormModel;

use crate::app::components::{form::FormInput, task_item::TaskModel, task_list::TaskListModel};

pub struct AppModel {
    form: Controller<FormModel>,
    task_list: Controller<TaskListModel>,
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
                #[local_ref]
                task_list_box -> gtk::ScrolledWindow {}
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let tasks = vec![
            TaskModel {name: "Do dishes".to_string()},
            TaskModel {name: "Do homework".to_string()},
            TaskModel {name: "Go buy food".to_string()},
        ];
        let model = AppModel {
            form: FormModel::builder().launch(root.clone()).detach(),
            task_list: TaskListModel::builder().launch(tasks).detach()
        };
        let task_list_box = model.task_list.widget();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppMsg::NewTask => self.form.sender().send(FormInput::Show).unwrap(),
        }
    }
}
