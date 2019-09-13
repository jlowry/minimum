

use minimum::task::WriteAllTaskImpl;
use crate::resources;

use minimum::task::TaskConfig;
use minimum::ResourceMap;
use minimum::TaskContextFlags;

pub struct FrameworkUpdateActionQueue;
pub type FrameworkUpdateActionQueueTask = minimum::WriteAllTask<FrameworkUpdateActionQueue>;
impl WriteAllTaskImpl for FrameworkUpdateActionQueue {
    fn configure(config: &mut TaskConfig) {
        config.this_runs_during_phase::<minimum::task::PhaseEndFrame>();
    }

    fn run(_context_flags: &TaskContextFlags, resource_map: &mut ResourceMap) {
        let mut framework_action_queue = resource_map.fetch_mut::<resources::FrameworkActionQueue>();
        framework_action_queue.process_queue(resource_map);
    }
}
