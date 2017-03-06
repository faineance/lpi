extern crate scoped_threadpool;
use scoped_threadpool::Pool;

use std::collections::HashMap;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;


struct Value((Sender<Box<Value>>, Receiver<Box<Value>>));
type Env = HashMap<String, Value>; // Receiver is not clonable otherwise I would've used a immutable map

#[derive(Debug)]
enum Term {
    New(String, Box<Term>),
    Parallel(Box<Term>, Box<Term>),
    Send(String, String, Box<Term>),
    Receive(String, String, Box<Term>),
    Nil,
}

fn eval(env: &mut Env, pi: Term) {
    use self::Term::*;
    let mut pool = Pool::new(2);
    match pi {
        Nil => return,
        New(name, pi) => {
            let chan = mpsc::channel();
            env.insert(name, Value(chan));
            eval(env, *pi);
        }
        Parallel(a, b) => {
            // pool.scoped(|scoped| {
            //     scoped.execute(move || eval(env, *a));
            //     scoped.execute(move || eval(env, *b));
            // });
            eval(env, *a);
            eval(env, *b);
        }
        Send(dest, message, pi) => {
            let chan = env.get(&message).unwrap();
            let &Value((tx, _)) = env.get(&dest).unwrap();
            tx.send(Box::new(*chan)).unwrap();
            eval(env, *pi);
        }
        Receive(src, bind, pi) => {
            eval(env, *pi);
        }
    }
}

fn main() {
    println!("Hello, world!");
}
