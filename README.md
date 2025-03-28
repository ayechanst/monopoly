# Monopoly

A monopoly game written in Rust, and uses Bevy and egui as a UI.

- Heavy usage of Rust's type system
- Uses shared ownership and internal mutability
- Uses an MPSC channel (mutiple producers single consumer) to send data between core logic and Bevy

## Flow

A game starts player one rolls a dice from the UI. This message is sent through a channel
to the core logic where their space is matched against the outcome. They can buy a property,
auction it to other players, as well as all other outcomes. Once their turn is over, a snapshot
of the board is taken and sent back through the channel to update the UI.

## Models

The core of the game logic is here.

### board.rs

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

### space.rs

An enum containing all possible spaces to land on. Spaces that share logic such
as properties are in their own enum variant `Property(Properties)` on the space
enum. When a player takes a turn, `Board` matches their position against the
`space` enum

### properties.rs

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

### player.rs

This struct differentiates players, keeps their balance, a vector of their properties, more
information pertaining to their board position. There is logic for them to mortage their
properties and buy houses.

## Bevy

Bevy here does not use ECS fully but rather just does two things:

1. Displays the logic occuring within the models just described
2. Serves as a UI for players to choose what to do on their turn

### spawn_board.rs

A system spawns a board according to the state of the spaces taken in the board's snapshot.
It also spawns the players on GO!

### buttons.rs

These buttons send player inputs through a mpsc (multiple propducers single consumer) channel
to `backend_loop.rs` in the utils folder. The backend loop is what calls the impl functions
on the board.

# Conclusion

Although the game is not done, the amount of features that were added were more than I intended.
This game started as just the backend logic which could be played through the CLI, but as I grew
my Rust knowledge I began to build a front end for the game in Bevy. The hardest part of the project
was managing shared ownership and internal mutability. In the later stages architecture was the
biggest challenge when trying to make Bevy compatible with the non-Bevy flavored Rust I had already
written.
