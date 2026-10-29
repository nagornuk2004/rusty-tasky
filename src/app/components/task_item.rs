use gtk::prelude::*;
use relm4::{
    FactorySender, RelmWidgetExt,
    factory::FactoryView,
    gtk,
    prelude::{DynamicIndex, FactoryComponent},
};

#[derive(Debug)]
pub struct TaskModel {
    pub name: String,
}

#[derive(Debug)]
pub enum TaskInput {}

#[derive(Debug)]
pub enum TaskOutput {}

#[relm4::factory(pub)]
impl FactoryComponent for TaskModel {
    type ParentWidget = gtk::ListBox;
    type Input = TaskInput;
    type Output = TaskOutput;
    type Init = TaskModel;
    type CommandOutput = ();

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            gtk::Label {
                set_label: &self.name,
                set_halign: gtk::Align::Start,
                set_margin_all: 12,
            }
        }
    }

    fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        init
    }

    fn init_widgets(
        &mut self,
        _index: &DynamicIndex,
        root: Self::Root,
        _returned_widget: &<Self::ParentWidget as FactoryView>::ReturnedWidget,
        _sender: FactorySender<Self>,
    ) -> Self::Widgets {
        let widgets = view_output!();
        widgets
    }

    fn update(&mut self, message: Self::Input, _sender: FactorySender<Self>) {
        match message {}
    }
}
