# Game Boy emulator

## Run tests

Install [nextest](https://nexte.st/) and [just](https://github.com/casey/just)

Run tests:

```bash
just test
```

### Integration tests

Download opcode tests:

```bash
just fetch-opcodes-tests
```

Run tests:

```bash
just test-integration
```


## License

This project is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT).

## Acknowledgements

- [Pan Docs](https://gbdev.io/pandocs)
- [Rednex Game Boy Development System](https://rgbds.gbdev.io/docs/v0.7.0)
- [Game Boy CPU (SM83) instruction set](https://gbdev.io/gb-opcodes/optables/)
- [Game Boy(tm) CPU Manual](http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf)
- [Game Boy CPU (SM83) Tests](https://github.com/adtennant/GameboyCPUTests)
