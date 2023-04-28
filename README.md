# Gita Greeter

Divine Bhagavad-gita verses greeter on command line. 🙏. All verses are stored on _./bhagavad.sqlite_ database.

The project depends on your HOME variable and thus it required bhagavad.sqlite to be stored in _$HOME/_ directory. i.e your home directory.

https://user-images.githubusercontent.com/49165465/235080579-ccccd410-4ebc-4f32-b090-e98c791c3820.mp4

## Setup

You can either symlink the binary file and place _bhagavad.sqlite_ to your $HOME directory, and then execute the binary from your .bashrc, .zshrc.

## Build

As of now, the only way to build project is using rust toolchains.

```sh
    git clone https://github.com/aniketkharel/gita-greeter

    cd gita-greeter

    cargo build --release

    # or

    make build
```
