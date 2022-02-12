local bag = {}

local function regen_bag()
	for i = 1, 2 * #pieces, 1 do
		bag[i] = (i - 1) % 7 + 1
	end
end

function spawn_piece(state)
	if #bag == 0 then
		regen_bag()
	end

	local p = bag[#bag]
	table.remove(bag, #bag)

	return pieces[p]
end
