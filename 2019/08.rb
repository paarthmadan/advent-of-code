w = 25
h = 6

layers = []

$<.first.split('').each_with_index do |n, i|
  if (i) % (w * h) == 0
    layers << []
  end
  layers.last << n.chomp.to_i
end

min_layer = layers.min_by { |layer| layer.count { |x| x.zero? } }

one = 0
two = 0

min_layer.each do |x|
  if x == 1
    one += 1
  elsif x == 2
    two += 1
  end
end

puts one * two

final_layer = Array.new(w * h, 2)

layers.each do |layer|
  h.times do |height|
    w.times do |weight|
      c = height * w + weight
      final_layer[c] = layer[c] if final_layer[c] == 2
    end
  end
end

final_layer.each_with_index do |n, i|
  puts if i % w*3 == 0
  print n, n, n
end
