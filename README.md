# skymarcher

`skymarcher` is a command-line interface to various data source related to Magic The Gathering.

## Installation

TODO

## Usage

Once installed, you can try this command to grab a random card information:

`sk random`

It will output the card information coming from [scryfall.com](https://scryfall.com/), for example:

```
Island
Basic Land — Island
({T}: Add {U}.)
```

The `card` command allows you to provide a set trigram and a card number in this set:

`sk card iko 13`

will produce:

```
Flourishing Fox {W}
Creature — Fox
Whenever you cycle another card, put a +1/+1 counter on Flourishing Fox.
Cycling {1} ({1}, Discard this card: Draw a card.)
1/1
```

## Development

In order to build your code in release mode, use:

`cargo build --release | mv ./target/release/skymarcher ./target/release/sk`

at the root of the project directory.
