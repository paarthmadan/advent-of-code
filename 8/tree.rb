class Node
  attr_accessor :children, :meta_data
end

@data = []
@index = 0

File.readlines("input").each do |line|
  @data = line.split(" ").map(&:to_i)
end

@meta_data_count = 0

# RETURN --> NODE
def traverse_data
  q = @data[@index]
  m = @data[@index + 1]
  @index += 2

  children_node = []
  q.times { |x| children_node << traverse_data }

  meta_data = []
  m.times do |y|
    meta_data << @data[@index]
    @meta_data_count += @data[@index]
    @index += 1
  end

  n = Node.new
  n.children = children_node
  n.meta_data = meta_data

  n
end

head = traverse_data
puts @meta_data_count

