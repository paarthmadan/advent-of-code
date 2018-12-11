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

hash_count = 0

p map.keys

original_coordinates = map.keys

(min_y..max_y).each do |y|
  (min_x..max_x).each do |x|
    md_total = 0
    original_coordinates.each do |coordinate|
      md = (coordinate[0] - x).abs + (coordinate[1] - y).abs
      md_total += md
      if x == 4 && y == 3
        puts "#{md} -- "
      end
    end
    if(md_total < 10000)
      map[[x,y]] = "#"
      hash_count += 1
    end
  end
end

(min_y..max_y).each do |y|
  (min_x..max_x).each do |x|
    print (map[[x,y]] == "") ? "." : map[[x,y]]
  end
  puts
end

puts hash_count
