local i = 1

function spawn_piece()
	p = pieces[i]

	i = i + 1
	if i > 7 then
		i = 1
	end

	return p
end
