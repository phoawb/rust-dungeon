use rust_dungeon::Doors;
use rust_dungeon::TILE_SIZE;
use rust_dungeon::VIEW_SIZE;
use sfml::system::Vector2f;

pub fn player_collision_w_walls(
    player_position: Vector2f,
    upper_left_corner_coordinates: Vector2f,
    room_doors: Doors,
) -> Vector2f {
    let mut updated_player_position: Vector2f = player_position;
    let x_padding = 10.0;
    let y_padding = 32.0;
    let collision_padding = Vector2f::new(15.0, 20.0);

    if room_doors.up
        && player_position.x
            >= upper_left_corner_coordinates.x + TILE_SIZE.x * 11.0 - collision_padding.x
        && player_position.x
            <= upper_left_corner_coordinates.x + TILE_SIZE.x * 12.0 + collision_padding.x
        && player_position.y < upper_left_corner_coordinates.y + TILE_SIZE.y + y_padding
    {
        return updated_player_position;
    }
    if room_doors.down
        && player_position.x
            >= upper_left_corner_coordinates.x + TILE_SIZE.x * 11.0 - collision_padding.x
        && player_position.x
            <= upper_left_corner_coordinates.x + TILE_SIZE.x * 12.0 + collision_padding.x
        && player_position.y
            > upper_left_corner_coordinates.y + VIEW_SIZE.y - TILE_SIZE.y - y_padding
    {
        return updated_player_position;
    }

    if room_doors.left
        && player_position.x < upper_left_corner_coordinates.x + TILE_SIZE.x + x_padding
        && player_position.y
            >= upper_left_corner_coordinates.y + TILE_SIZE.y * 6.0 - collision_padding.y
        && player_position.y
            <= upper_left_corner_coordinates.y + TILE_SIZE.y * 7.0 + collision_padding.y
    {
        return updated_player_position;
    }
    if room_doors.right
        && player_position.x
            >= upper_left_corner_coordinates.x + VIEW_SIZE.x - TILE_SIZE.x - x_padding
        && player_position.y
            >= upper_left_corner_coordinates.y + TILE_SIZE.y * 6.0 - collision_padding.y
        && player_position.y
            <= upper_left_corner_coordinates.y + TILE_SIZE.y * 7.0 + collision_padding.y
    {
        return updated_player_position;
    }
    updated_player_position = collision_w_walls(player_position, upper_left_corner_coordinates);
    updated_player_position
}

pub fn collision_w_walls(position: Vector2f, upper_left_corner_coordinates: Vector2f) -> Vector2f {
    let mut updated_position: Vector2f = position;
    let x_padding = 10.0;
    let y_padding = 32.0;

    if position.x < upper_left_corner_coordinates.x + TILE_SIZE.x + x_padding {
        let delta_x = upper_left_corner_coordinates.x + TILE_SIZE.x + x_padding - position.x;
        updated_position.x = position.x + delta_x;
    } else if position.x > upper_left_corner_coordinates.x + VIEW_SIZE.x - TILE_SIZE.x - x_padding {
        let delta_x =
            position.x - (upper_left_corner_coordinates.x + VIEW_SIZE.x - TILE_SIZE.x - x_padding);
        updated_position.x = position.x - delta_x;
    }

    if position.y < upper_left_corner_coordinates.y + TILE_SIZE.y {
        let delta_y = upper_left_corner_coordinates.y + TILE_SIZE.y - position.y;
        updated_position.y = position.y + delta_y;
    } else if position.y > (upper_left_corner_coordinates.y + VIEW_SIZE.y - TILE_SIZE.y - y_padding)
    {
        let delta_y =
            position.y + TILE_SIZE.y + y_padding - (upper_left_corner_coordinates.y + VIEW_SIZE.y);
        updated_position.y = position.y - delta_y;
    }

    updated_position
}

pub fn projectile_collision_w_walls(
    position: Vector2f,
    upper_left_corner_coordinates: Vector2f,
) -> bool {
    let x_padding = 10.0;
    let y_padding = 32.0;

    if position.x < upper_left_corner_coordinates.x + TILE_SIZE.x + x_padding
        || position.x > upper_left_corner_coordinates.x + VIEW_SIZE.x - TILE_SIZE.x - x_padding
    {
        return true;
    }

    if position.y < upper_left_corner_coordinates.y + TILE_SIZE.y
        || position.y > (upper_left_corner_coordinates.y + VIEW_SIZE.y - TILE_SIZE.y - y_padding)
    {
        return true;
    }

    false
}
