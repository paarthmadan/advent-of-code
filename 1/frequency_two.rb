frequency_count = 0

frequency_table = Hash.new(0)
frequency_table[0] = 1
found_duplicate = false

until found_duplicate
  File.readlines("input.txt").each do |line|
    frequency_count += Integer(line)
    frequency_table[frequency_count] += 1

    if frequency_table[frequency_count] == 2
      found_duplicate = true
      puts frequency_count
      break
    end

  end
end


