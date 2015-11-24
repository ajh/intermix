# Goals

* To allow a local client to start and connect to a local server
* To allow a remote client to connect to a server
* To allow multiple clients (remote or not) to connect to a server
* To allow a client to connect to multiple servers
* multiple clients will have their own panes and windows
* multiple clients will have their own modal state machine

# Design

# The Server will consist of:

* a list of running programs with their screen buffers
* a list of no-longer running programs' screen buffers
* a list of connected clients
* each client will have a list of programs that they are subscribed to

The server will send screen buffer event msgs for a program to all
subscribed clients. The server will receive requests to render stuff etc
from clients and sends those. The server will have commands to control
program subscription, list programs etc.

The server will only allow connections from the local system through a
pipe or something. Remote clients will have to ssh in and connect to
that pipe. Maybe the pipe could have a uri like
intermix://darkstar/tmp/some-server.pipe? Should be able to rely on OS
file authorization to control access to the pipe?

# The Client will consist of

* one or more windows
* panes within a window
* modal state machine
* a list of connected servers
* a list programs available on each server?
* a list of subscriptions for programs on each server

The client will receive input from the user. Depending on the mode,
different things can happen. Assuming that a program is in focus, keys
will be sent to the server. The client will be listening for messages
from the server. If the messages is about new screen buffer content, the
client will draw to the pane and window of the program.

# Technology choices

I'm thinking that unix domain sockets would be a good choice. That
solves the security question because the socket will be owned by a unix
user. Remove access can potentially happen through ssh using the -L or
-R feature (I should test this).
