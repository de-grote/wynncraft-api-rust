# Wynncraft API
Rust api wrapper for the wynncraft api.

Check out the [Wynncraft Api (WAPI) Docs](https://docs.wynncraft.com/docs/) for more details on endpoints and modules.

The modules of this crate are layed out in the same way as the wynncraft modules are, with function names similair to the endpoint names in the documentation (but not always the same).

This librairy is still in early development, so expect breaking changes with (almost) every update for now.

If you find a bug or a panic accures, feel free to open an issue.

### Supported api versions

| crate version | WAPI version |
| ------------- | ------------ |
| 0.0.1         | 3.3          |

## Features
* Types for results of the api calls.
* Functions to call the api.
* ItemQuery type to search the item db.
* Enums for fields that can only have a set number of values.

### Feature flags
* ``BTree`` to use ``BTreeMap`` and ``BTreeSet`` instead of ``HashMap`` and ``HashSet``.
* ``no_panic`` in the case that this librairy panics, that means that the code isn't working, this flag is a workaround to return an error instead of terminating.

## Things to be added to the librairy

### Will be added
* Actual documentation for functions and types.
* Automatic caching.
* Auto ratelimiter.

### Open questions
* Better types for uuids.
* Better ItemQuery interface.

## List of api inconsistencies
The api and documentation isn't really consistant, here are some things that I noticed while working with the api. (This is just for fun, don't blame the devs for this lol)

- Defence is spelled like `defense` only in the skill points of a character, even though it is documented as defence in the docs.
- A character has a field `preEconomy` which is not documented.
- `ElementalDamageBonusRaw` is serialized as PascalCase instead of camelCase like all other search fields.
- There exists a stat called `agility` that no item has (that is different from `rawAgility` that is what you would expect).
- If you pass in a result limit that is negative in the leaderboards api, than there is no limit and you get all the data (or an error since the server doesn't like it and dies).
- The last page of the paginated item api just errors instead of giving the last values (which are some crafting ingridients).
- In the search module the discoveries say they only have an x and z but they actually also have an y coordinate.
