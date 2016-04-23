Todo
====

* [x] clear screen after starting
* [x] allow changing selected program
* [ ] allow maximizing selected program
* [x] fix all the warning msgs
* [x] client - server architecture in same process
* [x] client takes options to control window size
* [x] layout engine
* [x] clean up hacks and add more tests
* [ ] pane transforms
* [ ] client - server in separate processes
* [ ] client connecting to multiple servers
* [x] modal UI design
* [ ] lsof for programs to view open files
* [ ] raw keyup keydown input behavior
* [ ] shell
* [x] cleanup todos and fixmes
* [x] servers should have ids ("some server")
* [ ] Split up ClientMsg into smaller enums suited to their specific
  channel
* [x] figure out performance implications about moving values between enums
* [ ] dwarf fortress style background pictures
* [x] alias some `vterm_sys` types like ScreenSize for ease of use

Client Server Work
==================

* [x] implement server ProgramStart handler
* [x] implement a mode
* [x] implement debug for ClientMsg and ServerMsg
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
