
use rustler::{Env, Term};

mod task;
mod echoer;
mod atoms;

fn load(env: Env, _: Term) -> bool {
    env_logger::init();
    echoer::load(env);
    true
}

rustler::init!(
    "Elixir.EchoRustNif.Native",
    [
        echoer::start,
        echoer::stop,
        echoer::echo,
    ],
    load = load
);
