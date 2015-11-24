# Launch intermix

    
    
    
    
    
    $

# run ls

    $ ls
    COPYING      Cargo.lock   Cargo.toml   Gemfile      Gemfile.lock
    Guardfile    README.md    doc          log          log4rs.toml  src
    target
    
    ====================================================================
    
    $

# run ls target

  
    $ ls
    COPYING      Cargo.lock   Cargo.toml   Gemfile      Gemfile.lock
    Guardfile    README.md    doc          log          log4rs.toml  src
    target
    
    ====================================================================
    
    $
    debug release

    ====================================================================

    $

# control-b

This will exit shell-mode (?) and enter nav-mode (?). The bottom pane
will be selected by default. Navigation can go up or down. Acceleration
stuff would be cool here to help navigate.

    $ ls
    COPYING      Cargo.lock   Cargo.toml   Gemfile      Gemfile.lock
    Guardfile    README.md    doc          log          log4rs.toml  src
    target
    
    ====================================================================
    
    $
    debug release

    ====================================================================

    $

# enter shell-mode again for bottom pane, and enter vim

I dont want to draw this but intermix will detect that the program isn't
exiting and enter in full screen. Control-b will enter nav-mode again.
