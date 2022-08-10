use gloo_worker::{Registrable};
use frontend::agent_calculator::Calculator;

fn main() {
    Calculator::registrar().register();
}
