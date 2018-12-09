PLAYERS = 416
LAST_MARBLE = 7197500

scores = Hash.new(0)

game = []
game.push(0)

current_marble = 0
i = 0

(1..LAST_MARBLE).each do |m|
  puts m
  current_player = (m % PLAYERS == 0) ? PLAYERS : m % PLAYERS
  if m % 23 != 0
    if game[i + 1] != nil
      game.insert(i + 2, m)
      i += 2
    else
      game.insert(1, m)
      i = 1
    end
  else
    scores[current_player] += game.delete_at(i - 7)
    scores[current_player] += m

    i = (i - 7 < 0) ? game.length + (i - 6) : i - 7
  end
end

puts "<------------------------>"
puts scores.values.max
puts "<------------------------>"
