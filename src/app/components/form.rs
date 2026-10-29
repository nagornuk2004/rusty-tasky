use relm4::{ComponentParts, ComponentSender, adw::prelude::*, prelude::*};

use crate::app::components::task_item::TaskModel;

pub struct FormModel {}

#[derive(Debug)]
pub enum FormInput {
    Show,
    Close,
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
            set_default_size: (300, 250),
            set_hide_on_close: true,
            gtk::Label::new(Some(&"Task Name")) {
                set_margin_start: 12,
                set_margin_top: 12,
                set_align: gtk::Align::Start,
            },
            #[name(create_task_entry)]
            gtk::Entry {
                set_margin_all: 12,
            },
            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_align: gtk::Align::End,
                #[name(create_task_button)]
                gtk::Button {
                    set_label: "Add",
                    set_margin_end: 12,
                    set_margin_start: 12,
                },
            },
        }
    }

    fn init(
        parent: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = FormModel {};
        let widgets = view_output!();

        let entry = widgets.create_task_entry.clone();
        let button_sender = sender.clone();
        
        widgets.create_task_entry.connect_activate(move |entry| {
            let buffer = entry.buffer();
            sender
                .output(FormOutput::CreateTask(TaskModel {
                    name: buffer.text().into(),
                }))
                .unwrap();
            buffer.delete_text(0, None);
            sender.input_sender().send(FormInput::Close).unwrap();
        });
        widgets.create_task_button.connect_clicked(move |_| {
            let buffer = entry.buffer();
            button_sender
                .output(FormOutput::CreateTask(TaskModel {
                    name: buffer.text().into(),
                }))
                .unwrap();
            buffer.delete_text(0, None);
            button_sender.input_sender().send(FormInput::Close).unwrap();
        });
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>, root: &Self::Root) {
        match message {
            FormInput::Show => root.present(),
            FormInput::Close => root.hide(),
        }
    }
}
