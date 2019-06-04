# coolcats2
**Rust rewrite of Clutter/Coolcats, a fully distributed social messaging app built on Holochain**

This is a Clutter clone, as close to exact in functionality as I could make it.

The front-end is implemented in Rust with the Yew framework, and of course the back-end is Rust as well.

See https://github.com/holochain/clutter for the original.

See https://github.com/pythagorean/coolcats for the Python port that was made of this, both for the old Holochain Proto.

Assuming you have already installed the command line `hc` tool and the `holochain` conductor, you should also
make sure you have installed and are using an `8.x LTS version of nodejs`, and the `rust nightly-2019-01-24 toolchain` along with the stable rust branch, and `yarn` for managing and installing `node` packages.

Building the UI requires [cargo web](https://github.com/koute/cargo-web):

    cargo install cargo-web

To run the holochain portion

```
make dna-start
```

To run the standalone frontend

Open a new separate terminal (So the backend stays running in the other one)

Then

```
make ui-start
```

To run the Conductor (currently only single user mode is enabled due to error
when starting multiple Conductors, edit Makefile to uncomment or add more)

```
make conductor-start
```

To stop the Conductor

```
make conductor-stop
```

Not for any sort of production use whatsoever at this time, no warranty express or implied.

## License
[![License: GPL v3](https://img.shields.io/badge/License-GPL%20v3-blue.svg)](http://www.gnu.org/licenses/gpl-3.0)

This code is Copyright (C) 2019 by Michael Goldman to the extent it is a novel implementation, and rights are
assigned to The MetaCurrency Project (Eric Harris-Braun, Arthur Brock, et. al.) to the extent that it is derivative.
Currently this is GPLv3 licensed to all, other licenses are being considered by the project which they may
re-license or fork this code under. The author reserves the right to fork under other licenses as well.
