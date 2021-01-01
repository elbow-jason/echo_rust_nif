use crate::atoms;

use crate::task;
use rustler::{Atom, Encoder, Env, OwnedEnv, LocalPid, ResourceArc};
use std::sync::Mutex;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use log::trace;

struct Ref(Mutex<Sender<Command>>);

impl Ref {
    fn new(tx: Sender<Command>) -> ResourceArc<Ref> {
        ResourceArc::new(Ref(Mutex::new(tx)))
    }
}

enum Command {
    Echo(LocalPid, String),
    Stop,
}

pub fn load(env: Env) -> bool {
    rustler::resource!(Ref, env);
    true
}

#[rustler::nif(name = "echoer_start")]
fn start() -> (Atom, ResourceArc<Ref>) {
    let (tx, rx) = channel::<Command>(1000);
    spawn_echoer(rx);
    (atoms::ok(), Ref::new(tx))
}

#[rustler::nif(name = "echoer_stop")]
fn stop(resource: ResourceArc<Ref>) -> Atom {
    send(resource, Command::Stop);
    atoms::ok()
}

#[rustler::nif(name = "echoer_echo")]
fn echo(env: Env, resource: ResourceArc<Ref>, message: String) -> Atom {
    let command = Command::Echo(env.pid(), message);
    send(resource.clone(), command);
    atoms::ok()
}

fn send(resource: ResourceArc<Ref>, command: Command) {
    let lock = resource.0.lock().expect("Failed to obtain a lock");
    let sender = lock.clone();

    task::spawn(async move {
        match sender.send(command).await {
            Ok(_) => (),
            Err(_err) => trace!("send error"),
        }
    });
}


fn spawn_echoer(mut rx: Receiver<Command>) {
    task::spawn(async move {
        let mut env = OwnedEnv::new();

        loop {
            match rx.recv().await {
                Some(Command::Echo(pid, msg)) => {
                    env.send_and_clear(&pid, move |env| {
                        (atoms::echo(), msg).encode(env)
                    });
                }
                Some(Command::Stop) => break,
                None => continue,
            }
        }
    });
}