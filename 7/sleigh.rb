@predecessors = Hash.new { |h, k| h[k] = [] }
@edges = Hash.new { |h, k| h[k] = [] }

File.readlines("input").each do |line|
  data = line.split(" ")
  from = data[1]
  to = data[7]
  @predecessors[to].push(from)
  @edges[from].push(to)
end

ready = ("A".."Z").find_all { |letter| @predecessors[letter] == [] }
ready.sort!

path = ""
current_time

until ready.length == 0
  current_letter = ready.shift
  path += current_letter
  edges = @edges[current_letter]
  options = edges.find_all { |edge| @predecessors[edge].all? { |letter| path.include?(letter) } }
  ready.concat(options)
  ready.sort!
end

puts path
