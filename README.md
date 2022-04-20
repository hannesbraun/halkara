# Halkara

Halkara is a simple command line utility to play content on Audius.

## Usage

```
USAGE:
    halkara [OPTIONS] [URLS]

OPTIONS:
    -g, --genre <GENRE>      Selects the trending tracks for a specified genre
    -h, --help               Print help information
        --max-length         The maximum length for a track (longer tracks won't be played)
        --min-length         The minimum length for a track (shorter tracks won't be played)
    -o, --order <ORDER>      The order in which to play the trending tracks [possible values: asc,
                             desc, rand]
    -t, --time <TIME>        Selects the trending tracks over a specified time range
        --ui <UI>            The user interface variant to use [possible values: compact, log,
                             ncurses]
    -V, --version            Print version information
        --volume <VOLUME>    The volume in dBFS
```

### Example

Playing the trending tracks within the genre "Electronic" in descending order
```bash
halkara --genre Electronic --order desc
```

### Controls

The following keys can be hit while Halkara is running to trigger some actions:

- `q`: quit the application
- `<space>`: play/pause
- `+`: increase volume
- `-`: decrease volume

You need to press enter after pressing those keys.

## Ncurses

An additional ncurses-based exists but won't be added to the build by default. Its state is rather incomplete. Handling user input does not work due to multithreading issue with ncurses. The same applies for resizing the terminal window. After a resize, the user interface may look weird.
Try it out if you're curious :)

### Building

Add the `ncurses` feature to the build and you should be good to go.
```bash
cargo build --release --features ncurses
```

See https://github.com/jeaye/ncurses-rs/issues/191 if you have trouble getting ncurses to compile on openSUSE (or maybe other distros as well).

## License

Halkara is released under the BSD 2-Clause License. For more information see [LICENSE](LICENSE).

## Author

Hannes Braun (hannesbraun@mail.de)
