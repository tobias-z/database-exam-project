# Contributing

## Understanding

This repository is a mono repo, and contains multiple projects inside it.

It is intended for you to work on a specific project at a time, and it is most likely better for your IDE's performance if you only open that one.

## Branching strategy

- `main`: All code on main is considered production ready, and should therefore always have had CI run on the code before a merge is allowed into this branch.
- `develop`: All development code is branched out from the develop branch, and should be merged back into the develop branch. Develop acts as a barrier to entry before allowing code to be merged with main.

- Types of branches:

  - `feature`: branched out from develop, and is used for new features
  - `bugfix`: branched out from develop, and is used for none critical bug fixes
  - `hotfix`: branched out from main, and is used for critical bug fixes
  - `docs`: branched out from main, and should not contain code changes but rather documentation or comments

  these types of branches are prefixed like so: `feature/something-awesome`

## Directory structure

- `lib`: this directory contains libraries not intended to be run, but rather imported in other applications. They should be installed locally, and imported through the group id `dk.groupa`. When importing them use a command like `mvn install` to install it locally. In CI all libraries are available to you.
- `core`: this directory contains applications that are supposed to be runnable. They include a Dockerfile, that will be built during CI and deployed
- `hack`: contains things like scripts, and other things that make life better
- `.github`: contains CI related configuration files. It is set up so that a new project can get a pipeline created for it using prebuilt actions
