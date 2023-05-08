local RegEx = require("@lune/regex")

do
	local text = "I categorically deny having triskaidekaphobia."
	assert(RegEx.new([[\b\w{13}\b]]):IsMatch(text))
end

do
	local text = "I categorically deny having triskaidekaphobia."
	local match = RegEx.new([[\b\w{13}\b]]):Find(text)

	assert(match.Start == 2)
	assert(match.End == 15)
	assert(match.Length == 13)
	assert(match.Text == "categorically")
	assert(not match.Empty)
end

do
	local text = "Retroactively relinquishing remunerations is reprehensible."
	local matches = RegEx.new([[\b\w{13}\b]]):FindAll(text)

	assert(#matches == 4)
end

do
	local re = RegEx.new([['([^']+)'\s+\((\d{4})\)]])
	local text = "Not my favorite movie: 'Citizen Kane' (1941)."
	local caps = re:Captures(text)

	assert(caps:Get(1).Text == "'Citizen Kane' (1941)")
	assert(caps:Get(2).Text == "Citizen Kane")
	assert(caps:Get(3).Text == "1941")
end

do
	local re = RegEx.new([['(?P<title>[^']+)'\s+\((?P<year>\d{4})\)]])
	local text = "Not my favorite movie: 'Citizen Kane' (1941)."
	local caps = re:Captures(text)

	assert(caps:Name("title").Text == "Citizen Kane")
	assert(caps:Name("year").Text == "1941")
	assert(caps:Get(1).Text == "'Citizen Kane' (1941)")
end
