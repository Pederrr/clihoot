use actix::{Context, Handler};
use actix_rt::System;

use crate::{messages::teacher::HardStop, teacher::init::Teacher};

impl Handler<HardStop> for Teacher {
    type Result = ();

    fn handle(&mut self, _msg: HardStop, _: &mut Context<Self>) {
        System::current().stop();
    }
}
