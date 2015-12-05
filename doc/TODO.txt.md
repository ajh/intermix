Todo
====

* [x] client - server architecture in same process
* [x] client takes options to control window size
* [ ] clean up hacks and add more tests
* [ ] pane layout engine
* [ ] pane transforms
* [ ] client - server in separate processes
* [ ] client connecting to multiple servers
* [ ] modal UI design
* [ ] v1 multiplexer (see mock.txt.md)
* [ ] lsof for programs to view open files
* [ ] raw keyup keydown input behavior
* [ ] shell
* [ ] cleanup todos and fixmes
* [ ] servers should have ids ("some server")
* [ ] Split up ClientMsg into smaller enums suited to their specific
  channel
* [ ] figure out performance implications about moving values between enums
* [ ] dwarf fortress style background pictures
* [ ] investigate whether vectors store the dynamic data on the stack or
  heap. The message enums may be inefficient.

Client Server Work
==================

* [x] implement server ProgramStart handler
* [ ] implement a mode
* [ ] implement debug for ClientMsg and ServerMsg
* [ ] Use a different enum to communicate between
  client::stdin\_read\_worker and client::input\_worker. DRY with
  VteWorkerMsg stuff.
* [ ] use cap'n proto instead of channel to communicate between client
      and server
* [ ] fix shutdown behavior

Benchmarks
==========

* pasting lots of input
* drawing while scrolling through many pages of text

Consider MVC Design
===================
