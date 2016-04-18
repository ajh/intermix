@0x89c5d68f30ae3dcf;

# # server api
#
# ## programs
#
# * list programs
# * shows a program
# * creates program
# * destroys program
#
# ## program attributes
#
# * list program attrs
# * show a program attr
# * update program size attr
#
# ## program subscriptions
#
# * list subscriptions to program
# * show a subscription
# * create subscription
# * destroy subscription
#
# ## program inputs
#
# * create program input
struct Program {
  id @0 :Text;
  command @1 :Text;
  pid @2 :Int32;
}

interface Lull {
  getPrograms @0 () -> (program :Text);
}
