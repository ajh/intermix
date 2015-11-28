Todo
====

* [ ] pane transforms
* [ ] client - server
* [ ] client connecting to multiple servers
* [ ] modal UI design
* [ ] v1 multiplexer (see mock.txt.md)
* [ ] lsof for programs to view open files
* [ ] raw keyup keydown input behavior
* [ ] shell

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
