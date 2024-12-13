use crate::models::{player::Player, spaces::properties::properties::Properties};
use std::cell::Ref;

pub fn debug_property(player: Ref<'_, Player>, properties: Properties) {
    println!(
        "Player {:?} landed on {:?}",
        player.player_number, properties
    );
}

pub fn debug_buy_property(player: Ref<'_, Player>, properties: Properties) {
    println!(
        "Player {:?} bought: {:?}, and has {:?} money left",
        player.player_number, properties, player.money
    );
}

pub fn debug_rent(owner: Ref<'_, Player>, renter: Ref<'_, Player>) {
    println!(
        "BOOM! Player {:?} (${:?}) landed on {:?}'s (${:?}) property",
        renter.player_number, renter.money, owner.player_number, owner.money,
    );
}
