pieces = {
	[1] = {
		size = 4,
		template = [[0000111100000000]],
		color = {
			r = 68,
			g = 210,
			b = 242,
			a = 0xFF,
		},
	},
	[2] = {
		size = 3,
		template = [[100111000]],
		color = {
			r = 53,
			g = 39,
			b = 145,
			a = 0xFF,
		},
	},
	[3] = {
		size = 3,
		template = [[001111000]],
		color = {
			r = 227,
			g = 133,
			b = 61,
			a = 0xFF,
		},
	},
	[4] = {
		size = 2,
		template = [[1111]],
		color = {
			r = 242,
			g = 210,
			b = 68,
			a = 0xFF,
		},
	},
	[5] = {
		size = 3,
		template = [[011110000]],
		color = {
			r = 49,
			g = 186,
			b = 47,
			a = 0xFF,
		},
	},
	[6] = {
		size = 3,
		template = [[010111000]],
		color = {
			r = 142,
			g = 47,
			b = 186,
			a = 0xFF,
		},
	},
	[7] = {
		size = 3,
		template = [[110011000]],
		color = {
			r = 196,
			g = 47,
			b = 47,
			a = 0xFF,
		},
	}
}

local function choose_piece()
	local r = math.random(1, #(pieces))
	return pieces[r]
end

function spawn_piece(state)
	return choose_piece()
end

function init_game()
	return {
		width = 10,
		height = 20,

		start_piece = choose_piece(),

		piece_view = {
			size = 5,
		},
		piece_hold = {
			enabled = true,
		},
	}
end

function on_place(state)
	_solveField(state)
end

math.randomseed(os.time())
