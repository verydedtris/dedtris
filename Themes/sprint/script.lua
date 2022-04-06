require "Themes.default_bag_spawn.script"

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
