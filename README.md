# Monopoly

A monopoly game written in Rust, and uses Bevy and egui as a UI.

## Models

The core of the game logic is here.

#### board.rs

```rust
#[derive(Debug, Clone)]
pub struct Board {
    pub spaces: Vec<SpaceRef>,
    pub players: Vec<PlayerRef>,
}
```

`Board` keeps a vector of `SpaceRef` enums and `PlayerRef` structs.
It has several functions allowing:

1. Buying properties
2. Matching a player's position to an outcome
3. Snapshotting the board state for Bevy to render
4. Rolling dice
5. Passing the turn
6. Getting the active player

#### space.rs

An enum containing all possible spaces to land on. Spaces that share logic such
as properties are in their own enum variant `Property(Properties)` on the space
enum. When a player takes a turn, `Board` matches their position against the
`space` enum

#### properties.rs

An enum inside `space` that lists all property categories

```rust
#[derive(Debug, PartialEq, Clone)]
pub enum Properties {
    ColoredProperty(ColoredProperties),
    Utility(Utilities),
    Railroad(Railroads),
}
```

These enums share logic so the following functionality is implemented on all variants:

1. Check if itself is for sale
2. Buy property
3. Auction property
4. Get the current owner
5. Pay rent
6. Mortgage
7. Buy a house

These functions allow a player to interact with a properties on their turn. If they land on
a property, a check is made to see if its owned. If its not they are prompted with the choice
to buy, or to auction. If the property is owned, they pay rent. Players can also manage their
other properties on their turn, although trading is not implemented yet.

**Utilities and Railraods**

Beyond the enum `Properties`, these property types stop sharing functionality with the
`ColoredProperties`. These unique properties calculate their rent differently and cannot
have houses built on them, so further specification is needed.
