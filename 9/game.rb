PLAYERS = 416
LAST_MARBLE = 71975

scores = Hash.new(0)

game = []
game.push(0)

current_marble = 0

(1..LAST_MARBLE).each do |m|
  current_player = (m % PLAYERS == 0) ? PLAYERS : m % PLAYERS
  i = game.index(current_marble)
  if m % 23 != 0
    if game[i + 1] != nil
      game.insert(i + 2, m)
    else
      game.insert(1, m)
    end
    current_marble = m
  else
    current_marble = game[i - 6]
    scores[current_player] += game.delete_at(i - 7)
    scores[current_player] += m
  end
end

puts scores.values.max
