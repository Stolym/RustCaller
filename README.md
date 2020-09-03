# Welcome on this little project,
### here you can find an event caller that uses generic functions that will be useful for your projects, thus facilitating your way of coding.

```rust
  //Make a new caller
  CallerEvent::new()
  //Add function in pool
  caller.add("any".to_string(), func);
  //Call function and add this in queue
  caller.call("any".to_string(), vec![10, 10]);
  //Process all queue
  caller.process();
```
