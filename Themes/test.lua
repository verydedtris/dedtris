local pieces = {
			[1] = {
				size = 4,
				template = [[
		0000
		1111
		0000
		0000
				]],
				color = {
					r = 68,
					g = 210,
					b = 242,
					a = 0xFF,
				},
			},
			[2] = {
				size = 3,
				template = [[
		100
		111
		000
				]],
				color = {
					r = 53,
					g = 39,
					b = 145,
					a = 0xFF,
				},
			},
			[3] = {
				size = 3,
				template = [[
		001
		111
		000
				]],
				color = {
					r = 227,
					g = 133,
					b = 61,
					a = 0xFF,
				},
			},
			[4] = {
				size = 2,
				template = [[
		11
		11
				]],
				color = {
					r = 242,
					g = 210,
					b = 68,
					a = 0xFF,
				},
			},
			[5] = {
				size = 3,
				template = [[
		011
		110
		000
				]],
				color = {
					r = 49,
					g = 186,
					b = 47,
					a = 0xFF,
				},
			},
			[6] = {
				size = 3,
				template = [[
		010
		111
		000
				]],
				color = {
					r = 142,
					g = 47,
					b = 186,
					a = 0xFF,
				},
			},
			[7] = {
				size = 3,
				template = [[
		110
		011
		000
				]],
				color = {
					r = 196,
					g = 47,
					b = 47,
					a = 0xFF,
				},
			}
		}

function spawn_piece()
	return pieces[0]
end

function load_config()
	return {
		width = 10,
		height = 20,
		pieces = pieces
	}
end
