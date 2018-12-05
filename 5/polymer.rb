private

def match?(x, y)
  (x != y) && (x.upcase == y.upcase)
end

public

def react(x)
  changed = false
  (x.length - 2).downto(0).each do |index|
    if match?(x[index], x[index + 1])
      x.slice!(index..index + 1)
      changed = true
    end
  end
  (changed) ? react(x) : x.length
end

polymer = ""
File.readlines("input.txt").each { |line| polymer += line.chomp }

puts react(polymer)

min = Float::MAX.to_i
("a".."z").each { |l| min = [min, react(polymer.gsub(/["#{l}#{l.upcase}"]/, ""))].min }

puts min
