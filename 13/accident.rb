FILE = "input"

class Cart
  attr_reader :x, :y, :dx, :dy, :rotation_count

  RIGHT = {">" => "v", "v" => "<", "<" => "^", "^" => ">"}
  LEFT = {">" => "^", "v" => ">", "<" => "v", "^" => "<"}

  def initialize(x, y, character)
    @x = x
    @y = y
    @character = character
    set_direction(character)
    @rotation_count = 1
  end

  def move
    @x += dx
    @y += dy
    self.current_pos
  end

  def to_s
    "Currently at (#{@x}, #{@y}) with current velocity as (#{@dx}, #{@dy})."
  end

  def current_pos
    [@x, @y]
  end

  def current_direction
    @character
  end

  def change_direction(intersection)
    case intersection
    when "\\"
      if %w(< >).include?(current_direction)
        set_direction(RIGHT[current_direction])
      else
        set_direction(LEFT[current_direction])
      end
    when "/"
      if %w(< >).include?(current_direction)
        set_direction(LEFT[current_direction])
      else
        set_direction(RIGHT[current_direction])
      end
    when "+"
      case @rotation_count % 3
      when 1
        set_direction(LEFT[current_direction])
      when 2
        set_direction(current_direction)
      when 0
        set_direction(RIGHT[current_direction])
      end
      @rotation_count += 1
    end
  end

  def set_direction(character)
    if %w(< >).include?(character)
      @dy = 0
      @dx = (character == ">") ? 1 : -1
    else
      @dx = 0
      @dy = (character == "^") ? -1 : 1
    end
    @character = character
  end
end

carts = []
special_points = {}

File.readlines(FILE).each.with_index do |line, y|
  line.each_char.with_index do |c, x|
    case c
    when *%w(< > v ^)
      carts << Cart.new(x, y, c)
    when *%w(\\ / +)
      special_points[[x, y]] = c
    else
    end
  end
end

accident = false

until accident
  visited = Hash.new(0)
  carts.each do |cart|
    new_pos = cart.move
    if special_points[new_pos]
      cart.change_direction(special_points[new_pos])
    end
    visited[new_pos] += 1
    if(visited[new_pos] > 1)
      accident = true
      p new_pos
    end
  end
end

