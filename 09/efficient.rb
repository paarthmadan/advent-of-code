class Node
  attr_accessor :left, :right, :value
  def initialize(value)
    @value = value
  end
end

PLAYERS = 416
LAST_MARBLE = 71975

scores = Hash.new(0)
head = Node.new(0)

head.right = head
head.left = head

curr = head

(1..LAST_MARBLE).each do |m|
  current_player = (m % PLAYERS == 0) ? PLAYERS : m % PLAYERS
  marble = Node.new(m)

  if m % 23 != 0
    p = curr.right
    n = p.right

    p.right = marble
    n.left = marble

    marble.left = p
    marble.right = n

    curr = marble
  else
    remove = curr
    7.times { |_| remove = remove.left }
    scores[current_player] += marble.value
    scores[current_player] += remove.value
    remove.left.right = remove.right
    remove.right.left = remove.left

    curr = remove.right
  end
end

puts scores.values.max
