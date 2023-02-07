# Experiment
This repository is prepared for evaluating the effort to refactor an application that runs on a single nodes into an application that runs on multiple nodes.

## Getting Started
- Cloning the repository into your own machine.
- Running all tests: `cargo test`
    - You should see the following output:
    ```
    test reminder::ready_reminder_server::tests::cmp_works ... ok
    test reminder::ready_reminder_server::tests::lt_works ... ok
    test reminder::ready_reminder_server::tests::heap_works ... ok
    ```
- Executing the provided single node application: `cargo run --example ready_reminder_single`
    - You should see the following output:
    ```
    The first event is: None
    The first event is: Some(Entry { content: "Hellow World!", time: Instant { t: 729558368703422 } })
    The first event is: Some(Entry { content: "Goodbye World!", time: Instant { t: 729560368697475 } })
    ```

## Explanation of the Provided Template Code
- `./experiment/src/reminder/ready_reminder_server.rs` contains:
    - the implementation of an event entry (`Entry`) that will become ready at some time in the future.
    - the implementation of a server (`ReadyReminderServer`) that stores events that will become ready at some time in the future. Events can be submitted to the server, and the server can be queried to see if any events are ready.
- `./experiment/examples/ready_reminder/single.rs` is an application that runs on a single node, which specifies times at which some events will become ready, then checks to see if those events are ready.

## Task
- Please refactor this application to run on multiple nodes. The `ReadyReminderServer` should run on a node, accepting requests from client(s) nodes that submit or extract events.
- Feel free to choose any framework for refactoring this application.

## Evaluation

Please work independently of the other participants. 

Please keep track of the amount of time that you spend on various programming tasks (including but not limited to reading documentations, seeking support, design, implementation, testing, and debugging). For each task please record observations if pertinent (especially levels of satisfaction or frustration with the process).
