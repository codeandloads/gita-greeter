# Gita Greeter

Divine Bhagavad-gita verses greeter on command line. 🙏. All verses are stored on *./bhagavad.sqlite* database.

The project depends on your HOME variable and thus it required bhagavad.sqlite to be stored in _$HOME/_ directory. i.e your home directory.

## Build

As of now, the only way to build project is using rust toolchains.

```sh
    git clone https://github.com/aniketkharel/gita-greeter

    cd gita-greeter

    cargo build --release

    # or

    make build
```
