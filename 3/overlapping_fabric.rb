fabric = Hash.new(".")
overlap_count = 0

amount = Hash.new(0)
real = Hash.new(0)

File.readlines("input.txt").each do |line|
  claim = line.split(" ")
  order = Integer(claim[0][1..-1])
  coordinates = claim[2][0..-2].split(",").map { |coordinate| coordinate.to_i }
  dimensions = claim[3].split("x").map { |coordinate| coordinate.to_i }
  amount[order] = dimensions[0] * dimensions[1]

  (coordinates[0]...(dimensions[0] + coordinates[0])).each do |i|
    (coordinates[1]...(dimensions[1] + coordinates[1])).each do |j|
      p = [i,j]
      if !(fabric[p] == "." || fabric[p] == "X")
        overlap_count += 1
        fabric[p] = "X"
      elsif fabric[p] == "."
        fabric[p] = order
      end
    end
  end
end

puts overlap_count

fabric.each do |key, value|
  unless value == "X"
    real[Integer(value)] += 1
  end
end


real.each do |key,value|
  puts key if (amount[key] -= value) == 0
end
