class Node
  attr_accessor :children, :meta_data

  def initialize(children, meta_data)
    @children = children
    @meta_data = meta_data
  end

  def value
    if children.length == 0
      meta_data.inject(0) { |sum,x| sum + x } if children.length == 0
    else
      meta_data.inject(0) { |sum, data| sum + ((children[data - 1]) ? children[data - 1].value : 0) }
    end
  end

  def sum
    meta_data.inject(0) { |sum, x| sum + x } + children.inject(0) { |sum, child| sum + child.sum }
  end
end

@data = []
@index = 0

File.readlines("input").each { |line| @data = line.split(" ").map(&:to_i) }

def traverse_data
  h = @data[@index..@index + 1]
  @index += 2

  children_node = []
  h[0].times { |x| children_node << traverse_data }

  meta_data = []
  h[1].times { |y| meta_data << @data[@index + y] }

  @index += h[1]

  n = Node.new(children_node, meta_data)
end

head = traverse_data

puts head.sum
puts head.value

