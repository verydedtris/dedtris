function a()
	spawn_piece = b
	return pieces[1]
end

function b()
	spawn_piece = c
	return pieces[2]
end

function c()
	spawn_piece = d
	return pieces[3]
end

function d()
	spawn_piece = e
	return pieces[4]
end

function e()
	spawn_piece = f
	return pieces[5]
end

function f()
	spawn_piece = g
	return pieces[6]
end

function g()
	spawn_piece = a
	return pieces[7]
end

spawn_piece = a

