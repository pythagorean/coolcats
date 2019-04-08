# coolcats2
**Rust rewrite of Clutter/Coolcats, a fully distributed social messaging app built on Holochain**

This is a Clutter clone, as close to exact in functionality as I could make it.

The front-end is implemented in Rust with the Yew framework, and of course the back-end is Rust as well.

See https://github.com/holochain/clutter for the original.

See https://github.com/pythagorean/coolcats for the Python port that was made of this, both for the old Holochain Proto.

Assuming you have already installed the command line *hc* tool and the *holochain* conductor, you should also
make sure you have installed and are using an *8.x LTS version of nodejs*, and the *rust nightly-2019-01-24 toolchain*, along
with *yarn* for managing and installing *node* packages. You will also need the
[http-server](https://www.npmjs.com/package/http-server) package that can be installed via *yarn*:

    yarn global add http-server

You can then start a multiuser server test, by unpacking *n3h* in a
parallel directory and/or editing the Makefile to specify the location where it is installed, and running:

    make startnet
    
You should have test instances you can access on http://localhost:8000 and http://localhost:8001, additional instances
can be easily added in the Makefile but performance may drag at this time.

When you want to stop, just ^C and then to cleanup any unstopped proecesses, run:

    make stopnet

Not for any sort of production use whatsoever at this time, no warrantee express or implied.

This code is Copyright (C) 2019 by Michael Goldman to the extent it is a novel implementation, and rights are assigned
to The MetaCurrency Project (Eric Harris-Braun, Arthur Brock, et. al.) to the extent that it is derivative. Currently
this is GPLv3 licensed to all, other licenses are being considered by the project which they may re-license this code under.
