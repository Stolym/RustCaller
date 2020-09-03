/*

Objectif:

    struct XNXX {
        data: i32,
    }

    enum Thing {
        Number(i32),
        Name(String),
        Shared(XNXX)
    }

    Struct CallerEvent {
        list: HashMap<String, fn(Vec<Thing>)>,
        queue: Vec<Thing>
    }

    Thread --> when object mounting add new thread who launch process of this class
*/


use std::collections::HashMap;
use std::thread;

struct CallerEvent {
    list: HashMap<String, fn(Vec<i32>)>,
    queue_name: Vec<String>,
    queue_args: Vec<Vec<i32>>,
}

impl CallerEvent {
    fn new() -> CallerEvent {
        CallerEvent{
            list: HashMap::new(),
            queue_name: Vec::new(),
            queue_args: Vec::new()
        }
    }

    fn add(&mut self, name: String, func: fn(Vec<i32>)) {
        self.list.insert(name, func);
    }

    fn call(&mut self, name: String, args: Vec<i32>) {
        match self.list.get(&name) {
            Some(_func) => { self.queue_name.push(name); self.queue_args.push(args); },
            None => println!("Function {}: isn't found.", name)
        }
    }

    fn process(&mut self) {
        for elem in self.queue_name.iter() {
            let name: String = elem.to_string();
            match self.list.get(&name) {
                Some(_func) => { _func(self.queue_args.remove(0)); },
                None => println!("Function {}: isn't found.", elem)
            }
        }
        self.queue_name.clear();
    }
}

fn main() {
    let mut caller = CallerEvent::new();

    caller.add("add".to_string(), add);
    caller.add("sub".to_string(), sub);
    caller.add("mul".to_string(), mul);
    caller.add("div".to_string(), div);
    
    loop {
        caller.call("add".to_string(), vec![10, 10]);
        caller.call("sub".to_string(), vec![10, 10]);
        caller.call("mul".to_string(), vec![10, 10]);
        caller.call("div".to_string(), vec![10, 10]);
        caller.process();
    }
}

fn add(args: Vec<i32>)
{
    println!("Addition: {}", args[0] + args[1]);
}

fn sub(args: Vec<i32>)
{
    println!("Soustraction: {}", args[0] - args[1]);
}

fn mul(args: Vec<i32>)
{
    println!("Multiplication: {}", args[0] * args[1]);
}

fn div(args: Vec<i32>)
{
    println!("Division: {}", args[0] / args[1]);
}