use relm4::{ComponentParts, ComponentSender, adw::prelude::*, prelude::*};

pub struct FormModel {}

#[derive(Debug)]
pub enum FormInput {
    Show,
}

#[relm4::component(pub)]
impl Component for FormModel {
    type Input = FormInput;
    type Output = ();
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
        }
    }

    fn init(
        parent: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = FormModel {};
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>, root: &Self::Root) {
        match message {
            FormInput::Show => root.present(),
        }
    }
}
