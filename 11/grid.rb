PUZZLE_INPUT = 9995

class Integer
  def digits(base: 10)
    quotient, remainder = divmod(base)
    quotient == 0 ? [remainder] : [*quotient.digits(base: base), remainder]
  end
end

def calculate_power(x, y, serial_number)
  rack_id = x + 10
  power_level = rack_id * y
  power_level += serial_number
  power_level *= rack_id
  power_level = power_level.to_i.digits.reverse[2] - 5
end


@map = Hash.new()

1.upto(300) do |y|
  1.upto(300) do |x|
    @map[[x,y]] = calculate_power(x, y, PUZZLE_INPUT)
  end
end

@memoize = Hash.new()

def calculate_grid(x, y, size)
  total = 0
  unless size == 1
    total += @memoize[[x,y]]
    size.times do |i|
      temp_x = x + (size - 1)
      total += @map[[temp_x, y + i]]
    end
    (size - 1).times do |j|
      temp_y = y + (size - 1)
      total += @map[[x + j, temp_y]]
    end
  else
    y.upto(y + (size - 1)) do |yy|
      x.upto(x + (size - 1)) do |xx|
        total += @map[[xx, yy]]
      end
    end
  end
  @memoize[[x,y]] = total
  total
end

max_value = -Float::MAX.to_i
max = []

1.upto(3) do |size|
  puts size
  1.upto(300 - size) do |y|
    1.upto(300 - size) do |x|
      c = calculate_grid(x, y, size)
      if max_value < c
        max_value = c
        max = [x, y, size]
      end
    end
  end
end

puts max
