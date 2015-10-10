# Intermix

Intermix is a prototype of a terminal emulator / multiplexer / cli
program. The goal is the work out the basic issues and designs in ruby
where code changes are easy, then eventually port it to rust.

## How to use

Run the ruby script bin/intermix passing in the command that will run
through the emulator as command line options, like:

    bin/intermix ls -G

This will run `ls -G` in the emulator.

# Status

* ls - works okay!
* ls -G - no colors
* less - nope
* vim - nope
* vtest - nope
* tmux - nope
* unicode - untested probably not working

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
