# coolcats2
**Rust rewrite of Clutter/Coolcats, a fully distributed social messaging app built on Holochain**

This is a Clutter clone, as close to exact in functionality as I could make it.

The front-end is implemented in Rust with the Yew framework, and of course the back-end is Rust as well.

See https://github.com/holochain/clutter for the original.

See https://github.com/pythagorean/coolcats for the Python port that was made of this, both for the old Holochain Proto.

If you have not already installed the command line `hc` tool and the `holochain` conductor, or if you need to update
these to the version being used by this hApp, please run `make update-cli` and/or `make update-conductor` respectively.
You should first also make sure you have installed and are using an `8.x LTS version of nodejs`, and the 
`rust nightly-2019-01-24 toolchain` along with the stable rust branch, and `yarn` for managing and installing `node`
packages.

Building the UI requires [cargo web](https://github.com/koute/cargo-web):

    cargo install cargo-web

You can then start a multiuser server test by running:

    make start

You should have test instances you can access on http://localhost:8000, http://localhost:8001 and http://localhost:8001.

When you want to stop, run:

    make stop

If you want to test a single instance on http://localhost:8000, to run a standalone holochain portion, first run:

    make dna-start

Then to run the standalone frontend, open a new separate terminal (so the backend stays running in the other one), 
and run:

    make ui-start

Not for any sort of production use whatsoever at this time, no warranty express or implied. Please feel free to file
any issues on github.

## License
[![License: GPL v3](https://img.shields.io/badge/License-GPL%20v3-blue.svg)](http://www.gnu.org/licenses/gpl-3.0)

This code is Copyright (C) 2019 by Michael Goldman to the extent it is a novel implementation, and rights are
assigned to The MetaCurrency Project (Eric Harris-Braun, Arthur Brock, et. al.) to the extent that it is derivative.
Currently this is GPLv3 licensed to all, other licenses are being considered by the project which they may
re-license or fork this code under. The author reserves the right to fork under other licenses as well.
