input = nil
frequency_count = 0

File.readlines("input.txt").each do |line|
  frequency_count += Integer(line)
end

puts frequency_count

