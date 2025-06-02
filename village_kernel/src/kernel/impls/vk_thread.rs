use crate::kernel::traits::vk_kernel::Thread;

pub struct ConcreteThread;

impl Thread for ConcreteThread {
    fn create_task(&self) -> i32 {
        0
    }
    
    fn get_task_id(&self) -> i32 {
        0
    }

    fn start_task(&self, tid: i32) -> bool {
        false
    }

    fn stop_task(&self, tid: i32) -> bool {
        false
    }

    fn wait_for_task(&self, tid: i32) -> bool {
        false
    }

    fn exit_blocked(&self, tid: i32) -> bool {
        false
    }

    fn delete_task(&self, tid: i32) -> bool {
        false
    }

    fn is_task_alive(&self, tid: i32) -> bool {
        false
    }

    fn get_tasks(&self) {

    }

    fn change_state(&self) {

    }

    fn sleep(&self) {

    }

    fn blocked(&self) {

    }
    
    fn task_exit(&self) {

    }

    fn save_task_psp(&self, psp: u32) {

    }

    fn get_task_psp(&self) -> u32 {
        0
    }
    
    fn select_next_task(&self) {

    }

    fn idle_task(&self) {

    }
}
