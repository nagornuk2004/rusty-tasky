use relm4::{ComponentParts, ComponentSender, adw::prelude::*, prelude::*};

use crate::app::components::task_item::TaskModel;

pub struct FormModel {}

#[derive(Debug)]
pub enum FormInput {
    Show,
}

#[derive(Debug)]
pub enum FormOutput {
    CreateTask(TaskModel),
}

#[relm4::component(pub)]
impl Component for FormModel {
    type Input = FormInput;
    type Output = FormOutput;
    type Init = adw::ApplicationWindow;
    type CommandOutput = ();

    view! {
        #[root]
        gtk::Dialog {
            set_title: Some("Add a task"),
            set_modal: true,
            set_transient_for: Some(&parent),
            set_default_size: (400, 300),
            set_hide_on_close: true,
            #[name(create_task_entry)]
            gtk::Entry,
        }
    }

    fn init(
        parent: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = FormModel {};
        let widgets = view_output!();
        widgets.create_task_entry.connect_activate(move |entry| {
            let buffer = entry.buffer();
            sender
                .output(FormOutput::CreateTask(TaskModel {
                    name: buffer.text().into(),
                }))
                .unwrap();
            buffer.delete_text(0, None);
        });
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>, root: &Self::Root) {
        match message {
            FormInput::Show => root.present(),
        }
    }
}
