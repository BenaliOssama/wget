use crate::parser::WgetCli;

pub trait Executer {
    fn execute(&self);
    fn download(&self);
    fn apply_speed_limit(&self);
    fn mirror(&self);
}

impl Executer for WgetCli {
    fn execute(&self) {
        self.download();
        self.apply_speed_limit();
        self.mirror();
    }

    fn apply_speed_limit(&self) {
        
    }

    fn download(&self) {
        
    }

    fn mirror(&self) {
        
    }
}