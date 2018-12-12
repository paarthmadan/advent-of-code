FILE = "input"
PADDING = 10000

input = []
File.readlines(FILE).each { |line| input << line }

# Parse Initial State
state = ("." * PADDING) + input[0].split(" ")[2] + ("." * PADDING)
notes = Hash.new("x")

input[2..-1].each do |code|
  data = code.split(" ")
  notes[data[0]] = data[2]
end

50000000000.times do |q|
  puts q
  new_state = ""
  state.each_char.with_index do |c,i|
    new_state << notes[state[(i - 2)..(i + 2)]]
  end
  state = new_state.gsub!("x", ".")
  puts state
end

puts state

total = 0
state.each_char.with_index do |c,i|
  total += (i - PADDING) if c == "#"
end

puts total
