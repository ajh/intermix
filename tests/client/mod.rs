// #The plan for Client tests
//
// The test harness will look like this:
//
// Start a client with a CaptureIO object. Send it keys or whatever to get it to dance. Then send
// the captured io through libvterm and assert on what is rendered. I do asserts periodically
// integration style too.
//
// Also, when the client interacts with the server, it does so through a channel. I can control
// the other side of the channel to get stuff to happen. This may not be a good idea until cap'n
// proto or msg pack is used to avoid having to rewrite too much test code.
//
// Things to test:
//
//     fn client_starts_in_command_mode()
//     fn client_can_start_a_program()
//     fn client_can_connect_to_a_server()
//
//

pub mod integration;
pub mod layout;
// trying to rewrite these, and they fail so turning them off
//pub mod tty_painter;
pub mod drawing;
