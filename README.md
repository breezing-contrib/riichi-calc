# Riichi Calc

Library to calculate the score of a hand in riichi mahjong.

## Example

```rust
let input = Input::new(
    pi_input, // information about the tiles in hand
    field,    // information about the field
    status,   // player's status
);

let result = input.calc_hand();
if result.is_err() {
    panic!("invalid input");
}
let result = result.unwrap();
```

Expected output includes the winning hand, found yaku, and score result:

```txt
Output {
    winning_hand: WinningHand {
        hand: [
            Janto(
                Tile {
                    number: 9,
                    tile_type: Souzu,
                },
            ),
            Shuntsu(
                Tile {
                    number: 5,
                    tile_type: Souzu,
                },
                false,
            ),
            Shuntsu(
                Tile {
                    number: 1,
                    tile_type: Manzu,
                },
                false,
            ),
            Shuntsu(
                Tile {
                    number: 5,
                    tile_type: Manzu,
                },
                false,
            ),
            Shuntsu(
                Tile {
                    number: 2,
                    tile_type: Pinzu,
                },
                false,
            ),
        ],
        winning_tile: Tile {
            number: 5,
            tile_type: Souzu,
        },
        red_tile: 0,
    },
    found_result: FoundYaku(
        FoundYaku {
            dora: [
                YakuEntry {
                    kind: Dora,
                    value: 1,
                },
                YakuEntry {
                    kind: UraDora,
                    value: 1,
                },
            ],
            ii_han: [
                YakuEntry {
                    kind: Riichi,
                    value: 1,
                },
                YakuEntry {
                    kind: Pinfu,
                    value: 1,
                },
            ],
            ryan_han: [],
            san_han: [],
            roku_han: [],
        },
    ),
    score_result: ScoreResult {
        points: Ron(11600),
        actual_points: Ron(12200),
        detail: ScoreDetail {
            han: 4,
            fu: 30,
        },
    },
}
```

Use `YakuEntry::name()` to get the localized display name for each yaku. By default names are Japanese; enable the `zh-cn` feature to use simplified Chinese names.
