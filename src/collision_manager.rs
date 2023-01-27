use rust_dungeon::VIEW_SIZE;
use sfml::system::Vector2f;

pub fn player_collision_w_walls(
    player_position: Vector2f,
    upper_left_corner_coordinates: Vector2f,
) -> Vector2f {
    let mut updated_player_position: Vector2f = player_position;
    let tile_size = 32.0;
    let x_padding = 10.0;
    let y_padding = 32.0;

    if player_position.x < upper_left_corner_coordinates.x + tile_size + x_padding {
        let delta_x = upper_left_corner_coordinates.x + tile_size + x_padding - player_position.x;
        updated_player_position.x = player_position.x + delta_x;
    } else if player_position.x
        > upper_left_corner_coordinates.x + VIEW_SIZE.x - tile_size - x_padding
    {
        let delta_x = player_position.x
            - (upper_left_corner_coordinates.x + VIEW_SIZE.x - tile_size - x_padding);
        updated_player_position.x = player_position.x - delta_x;
    }

    if player_position.y < upper_left_corner_coordinates.y + tile_size {
        let delta_y = upper_left_corner_coordinates.y + tile_size - player_position.y;
        updated_player_position.y = player_position.y + delta_y;
    } else if player_position.y
        > (upper_left_corner_coordinates.y + VIEW_SIZE.y - tile_size - y_padding)
    {
        let delta_y = player_position.y + tile_size + y_padding
            - (upper_left_corner_coordinates.y + VIEW_SIZE.y);
        updated_player_position.y = player_position.y - delta_y;
    }

    updated_player_position
}
