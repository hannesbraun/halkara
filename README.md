# Halkara

Halkara is a simple command line utility to play the tracks that are currently trending on Audius.

## Usage

```
USAGE:
    halkara [OPTIONS]

OPTIONS:
    -g, --genre <GENRE>      Selects the trending tracks for a specified genre
    -h, --help               Print help information
    -o, --order <ORDER>      The order in which to play the trending tracks [possible values: asc,
                             desc, rand]
    -t, --time <TIME>        Selects the trending tracks over a specified time range
    -v                       Sets the level of verbosity
    -V, --version            Print version information
        --volume <VOLUME>    The volume in dBFS
```

### Example

Playing the trending tracks within the genre "Electronic" in descending order
```bash
halkara --genre Electronic --order desc
```

## License

Halkara is released under the BSD 2-Clause License. For more information see [LICENSE](LICENSE).

## Author

Hannes Braun (hannesbraun@mail.de)
