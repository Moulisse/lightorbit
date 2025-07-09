pub mod math;

use std::time::Duration;

use math::Vec2;
use spacetimedb::{reducer, table, Identity, ReducerContext, ScheduleAt, Table};

#[table(name = player, public)]
pub struct Player {
    #[primary_key]
    pub identity: Identity,
    pub name: Option<String>,
    pub online: bool,

    pub entity_id: u32,
}

#[table(name = entity, public)]
pub struct Entity {
    #[auto_inc]
    #[primary_key]
    pub entity_id: u32,
    // Set by the server
    pub position: Vec2,
    // Set by the client
    pub direction: Option<Vec2>,
}

/// Takes a name and checks if it's acceptable as a player's name.
fn validate_name(name: String) -> Result<String, String> {
    if name.is_empty() {
        Err("Names must not be empty".to_string())
    } else {
        Ok(name)
    }
}

/// Called when a client connects to a SpacetimeDB database server
#[reducer(client_connected)]
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
        let entity = ctx.db.entity().insert(Entity {
            entity_id: 0,
            position: Vec2 { x: 0.0, z: 0.0 },
            direction: None,
        });

        ctx.db.player().insert(Player {
            name: None,
            identity: ctx.sender,
            online: true,
            entity_id: entity.entity_id,
        });
    }
}

// Called when a client disconnects from SpacetimeDB database server
#[reducer(client_disconnected)]
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

/// Clients invoke this reducer to set their player names.
#[reducer]
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

/// Client can set their direction, removing their flag
#[reducer]
pub fn set_direction(ctx: &ReducerContext, direction: Option<Vec2>) -> Result<(), String> {
    let player = ctx
        .db
        .player()
        .identity()
        .find(&ctx.sender)
        .ok_or("Player not found")?;

    if let Some(entity) = ctx.db.entity().entity_id().find(player.entity_id) {
        ctx.db.entity().entity_id().update(Entity {
            direction,
            ..entity
        });
        Ok(())
    } else {
        Err("Entity not found".to_string())
    }
}

/// First, we declare the table with scheduling information.
#[table(name = move_all_entities_timer, scheduled(move_all_entities))]
pub struct MoveAllEntitiesTimer {
    #[primary_key]
    #[auto_inc]
    scheduled_id: u64,
    scheduled_at: spacetimedb::ScheduleAt,
}

/// Then, we declare the scheduled reducer.
#[reducer]
pub fn move_all_entities(ctx: &ReducerContext, _timer: MoveAllEntitiesTimer) -> Result<(), String> {
    if ctx.sender != ctx.identity() {
        return Err(
            "Reducer `scheduled` may not be invoked by clients, only via scheduling.".into(),
        );
    }
    for entity in ctx.db.entity().iter() {
        if entity.direction.is_some() {
            ctx.db.entity().entity_id().update(Entity {
                position: Vec2 {
                    x: entity.position.x + entity.direction.unwrap().x * 100.0,
                    z: entity.position.z + entity.direction.unwrap().z * 100.0,
                },
                ..entity
            });
        }
    }
    Ok(())
}

/// Now, we want to actually start scheduling reducers.
#[reducer(init)]
pub fn init(ctx: &ReducerContext) -> Result<(), String> {
    log::info!("Initializing...");
    ctx.db
        .move_all_entities_timer()
        .try_insert(MoveAllEntitiesTimer {
            scheduled_id: 0,
            scheduled_at: ScheduleAt::Interval(Duration::from_millis(1000).into()),
        })?;
    Ok(())
}
