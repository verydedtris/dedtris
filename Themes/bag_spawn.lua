require "Themes.default.script"

local bag = {}

local function shuffle(bag)
	for i = #bag, 2, -1 do
		local j = math.random(i)
		bag[i], bag[j] = bag[j], bag[i]
	end

	return bag
end

function bag_spawn()
	if #bag == 0 then
		for i = 1, 2 * #pieces, 1 do
			bag[i] = (i - 1) % 7 + 1
		end
		shuffle(bag)
	end

	local p = bag[#bag]
	table.remove(bag, #bag)

	return pieces[p]
end
