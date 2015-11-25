# Design approach

For now, I'm just going to list the operations I need, then sort them
into mode groups as best I can.

Modes are a client concept. They don't exist on the server.

# Commands

* keys to go a program
* navigate through programs, up / down / select
* pane mgmt - navigate through panes, close them
* window mgmt - open new window, navigate through windows left right,
  select window

# Grammar

It'd be nice to come up with a grammar similar to vim's motions and text
objects. It could use verbs and nouns, where the same verbs are
interchangeable between panes, windows, programs etc.

I'm not sure the interaction with intermix is complicated enough to get
a benefit from this.

# Rust Design

Requirements for a mode object.

* polymorphic - an insert mode behaves differently than a normal mode,
  but has the same methods
* gets passed around as client state

I think the options are:

* trait object
* generics
* enum

Generics are awkward because then state::State would have to be generic
and rebuild from scratch each time the mode changes (State<NormalMode> can't
be assigned as InsertMode).

enum's don't handle the polymorphism elegantly.

Trait objects don't work because then State doesnt have the Sized
marker, which is needed everywhere.

So I'll try this:

State will have a Mode struct with info common to all modes that will
get synced for all client threads.

Input worker will additionally have modes that implement the
modal functionality.
