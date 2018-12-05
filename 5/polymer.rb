private

def match?(x, y)
  (x != y) && (x.upcase == y.upcase)
end

public

def remove_reactive_pair(x)
  made_change = false
  (x.length - 2).downto(0).each do |index|
    if match?(x[index], x[index + 1])
      x.slice!(index..index + 1)
      made_change = true
    end
  end
  remove_reactive_pair(x) if made_change
  x
end

polymer = ""

File.readlines("input.txt").each { |line| polymer += line.chomp }

puts remove_reactive_pair(polymer).length

current_min = Float::MAX.to_i
("a".."z").each { |letter| current_min = [current_min, remove_reactive_pair(polymer.gsub(/["#{letter}#{letter.upcase}"]/, "")).length].min }

puts current_min
