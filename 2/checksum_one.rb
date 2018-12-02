two_count = 0
three_count = 0

File.readlines("input.txt").each do |line|
  letter_hash = Hash.new(0)
  line.each_char { |c| letter_hash[c] += 1 }
  two_count += 1 if letter_hash.has_value?(2)
  three_count += 1 if letter_hash.has_value?(3)
end

puts two_count * three_count
