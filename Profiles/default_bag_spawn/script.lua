require "Profiles.defaults"
require "Profiles.bag_spawn"

function spawn_piece(state)
	return bag_spawn()
end

function init_game()
	return {
		width = 10,
		height = 20,

		start_piece = bag_spawn(),

		piece_view = {
			size = 5,
		},
		piece_hold = {
			enabled = true,
		},
	}
end
