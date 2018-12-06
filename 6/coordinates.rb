map = Hash.new("")
distance_map = Hash.new(Float::MAX.to_i)

current_symbol = 1

min_y = Float::MAX.to_i
min_x = Float::MAX.to_i
max_y = Float::MIN.to_i
max_x = Float::MIN.to_i

File.readlines("input").each do |line|
  data = line.split(", ")
  x = Integer(data[0])
  y = Integer(data[1])
  map[[x,y]] = current_symbol
  current_symbol += 1
  min_y = [min_y, y].min
  min_x = [min_x, x].min
  max_y = [max_y, y].max
  max_x = [max_x, x].max
  distance_map[[x,y]] = -1
end

puts min_y, min_x, max_y, max_x


map.keys.each do |coordinate|
  value = map[coordinate]
  puts value
  (min_y..max_y).each do |y|
    (min_x..max_x).each do |x|
      md = (Math.sqrt((coordinate[0] - x) ** 2) + Math.sqrt((coordinate[1] - y) ** 2)).to_i
      if distance_map[[x,y]] == md
        map[[x,y]] = "."
      elsif distance_map[[x,y]] > md
        distance_map[[x,y]] = md
        map[[x,y]] = value
      end
    end
  end
end

(min_y..max_y).each do |y|
  (min_x..max_x).each do |x|
    print (map[[x,y]] == "") ? "." : map[[x,y]]
  end
  puts
end

number_hash = Hash.new(0)
(map.values).each { |v| number_hash[Integer((v == ".") ? "-1" : v)] += 1 }

puts number_hash.values.sort
