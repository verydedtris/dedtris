require "Profiles.default_bag_spawn.script"

function spawn_piece(state)
	return bag_spawn()
end

local score = 40

function on_place(state)
	local placed = _solveField(state)

	if #placed > 0 then
		score = score - #placed

		if score <= 0 then
			_finishGame(state)
		else
			print(string.format("Remaining: %i", score))
		end
	end
end

function init_game()
	return {
		width = 10,
		height = 20,

		start_piece = bag_spawn(),

		piece_tick = 1000,

		piece_view = {
			size = 5,
		},
		piece_hold = {
			enabled = true,
		},
	}
end
