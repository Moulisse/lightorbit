pub mod math;

use math::Vec2;
use spacetimedb::{reducer, table, Identity, ReducerContext, Table};

#[table(name = player, public)]
pub struct Player {
    #[primary_key]
    identity: Identity,
    name: Option<String>,
    online: bool,
    // Set by the server
    position: Option<Vec2>,
    // Set by the client
    direction: Option<Vec2>,
    flag: Option<Vec2>,
}

/// Takes a name and checks if it's acceptable as a player's name.
fn validate_name(name: String) -> Result<String, String> {
    if name.is_empty() {
        Err("Names must not be empty".to_string())
    } else {
        Ok(name)
    }
}

#[reducer(client_connected)]
/// Called when a client connects to a SpacetimeDB database server
pub fn client_connected(ctx: &ReducerContext) {
    if let Some(player) = ctx.db.player().identity().find(ctx.sender) {
        // If this is a returning player, i.e. we already have a `Player` with this `Identity`,
        // set `online: true`, but leave `name` and `identity` unchanged.
        ctx.db.player().identity().update(Player {
            online: true,
            ..player
        });
    } else {
        // If this is a new player, create a `Player` row for the `Identity`,
        // which is online, but hasn't set a name.
        ctx.db.player().insert(Player {
            name: None,
            identity: ctx.sender,
            online: true,
            position: None,
            direction: None,
            flag: None,
        });
    }
}

#[reducer(client_disconnected)]
// Called when a client disconnects from SpacetimeDB database server
pub fn identity_disconnected(ctx: &ReducerContext) {
    if let Some(player) = ctx.db.player().identity().find(ctx.sender) {
        ctx.db.player().identity().update(Player {
            online: false,
            ..player
        });
    } else {
        // This branch should be unreachable,
        // as it doesn't make sense for a client to disconnect without connecting first.
        log::warn!(
            "Disconnect event for unknown player with identity {:?}",
            ctx.sender
        );
    }
}

#[reducer]
/// Clients invoke this reducer to set their player names.
pub fn set_name(ctx: &ReducerContext, name: String) -> Result<(), String> {
    let name = validate_name(name)?;
    if let Some(player) = ctx.db.player().identity().find(ctx.sender) {
        ctx.db.player().identity().update(Player {
            name: Some(name),
            ..player
        });
        Ok(())
    } else {
        Err("Cannot set name for unknown player".to_string())
    }
}

#[reducer]
/// Client can set their direction, removing their flag
pub fn set_direction(ctx: &ReducerContext, direction: Vec2) -> Result<(), String> {
    let player = ctx
        .db
        .player()
        .identity()
        .find(&ctx.sender)
        .ok_or("Player not found")?;
    ctx.db.player().identity().update(Player {
        direction: Some(direction.normalized()),
        flag: None,
        ..player
    });
    Ok(())
}

#[reducer]
/// Client can set their flag, removing their direction
pub fn set_flag(ctx: &ReducerContext, flag: Vec2) -> Result<(), String> {
    let player = ctx
        .db
        .player()
        .identity()
        .find(&ctx.sender)
        .ok_or("Player not found")?;
    ctx.db.player().identity().update(Player {
        direction: None,
        flag: Some(flag.normalized()),
        ..player
    });
    Ok(())
}
