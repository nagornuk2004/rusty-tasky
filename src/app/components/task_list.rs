use gtk::prelude::*;
use relm4::factory::FactoryVecDeque;
use relm4::{ComponentParts, ComponentSender, SimpleComponent, gtk};

use crate::app::components::task_item::TaskModel;

pub struct TaskListModel {
    tasks: FactoryVecDeque<TaskModel>,
}

#[derive(Debug)]
pub enum TaskListInput {
    CreateTask(TaskModel),
}

#[derive(Debug)]
pub enum TaskListOutput {}

#[relm4::component(pub)]
impl SimpleComponent for TaskListModel {
    type Input = TaskListInput;
    type Output = TaskListOutput;
    type Init = Vec<TaskModel>;

    view! {
        #[root]
        gtk::ScrolledWindow {
            set_hscrollbar_policy: gtk::PolicyType::Never,
            set_min_content_height: 360,
            set_vexpand: true,

            #[local_ref]
            task_list_box -> gtk::ListBox {}
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut tasks = FactoryVecDeque::builder().launch_default().detach();
        tasks.extend(init);
        let model = TaskListModel { tasks };
        let task_list_box = model.tasks.widget();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            TaskListInput::CreateTask(task) => {
                self.tasks.guard().push_back(task);
            }
        }
    }
}
