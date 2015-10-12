# Intermix

Intermix is a prototype of a terminal emulator / multiplexer / cli
program. The goal is the work out the basic issues and designs in ruby
where code changes are easy, then eventually port it to rust.

## How to use

Run the ruby script bin/intermix.

Intermix will start in 'Command Mode'. `Ctrl-c` will exit, `Ctrl-p` will
enter Pry Mode. From within Pry Mode, a command can be run with
`split_and_run` like this:

    $ split_and_run 'irb'

Then press `Ctrl-d` to go back to command mode. Press `1` to select the irb
pane, then `i` to get into program mode for irb. Then irb can be
interacted with.

## Status

### terminal emulation

First pass working:

* ls - works okay!
* ls -G - no colors
* less - nope
* vim - nope
* vtest - nope
* tmux - nope
* unicode - untested probably not working

### running multiple processes at once

First pass working

### client server

Not started

### modal ui

First pass working

### shell

Not started

### keyboard interactions

Detecting and using keyup keydown events and timing. Not started

### screen transforms

First pass working

## Installation

Add this line to your application's Gemfile:

    gem 'intermix'

And then execute:

    $ bundle

Or install it yourself as:

    $ gem install intermix

## Development

After checking out the repo, run `bin/setup` to install dependencies.
Then, run `rake spec` to run the tests. You can also run `bin/console`
for an interactive prompt that will allow you to experiment.

To install this gem onto your local machine, run `bundle exec rake
install`. To release a new version, update the version number in
`version.rb`, and then run `bundle exec rake release`, which will create
a git tag for the version, push git commits and tags, and push the
`.gem` file to [rubygems.org](https://rubygems.org).

## Contributing

Bug reports and pull requests are welcome on GitHub at
https://github.com/[USERNAME]/intermix. This project is intended to be a
safe, welcoming space for collaboration, and contributors are expected
to adhere to the [Contributor Covenant](contributor-covenant.org) code
of conduct.

## License

The gem is available as open source under the terms of the [MIT
License](http://opensource.org/licenses/MIT).
