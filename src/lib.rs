use stride_plugin::{
    emit,
    event::{HostEvent, PluginEvent},
    plugin, Plugin,
};

#[derive(Default)]
struct TestPlugin {}

impl Plugin for TestPlugin {
    fn init() -> Self {
        Self::default()
    }

    // Simple plugin that clones every task that is created with a suffix.
    fn event(&mut self, event: HostEvent) -> bool {
        let HostEvent::TaskCreate { task: Some(task) } = event.clone() else {
            return true;
        };

        let mut task = task.as_ref().clone();
        task.title += " (Cloned)";
        emit(&PluginEvent::TaskCreate { task });

        true
    }
}

// Register plugin.
plugin!(TestPlugin);
