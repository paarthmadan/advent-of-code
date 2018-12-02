ids = []
File.readlines("input.txt").each { |line| ids << line.chomp }

ids.length.times do |i|
  (i + 1...ids.length).each do |j|
    if ids[i].length == ids[j].length
      difference = Hash.new(0)
      (0..ids[i].length).each do |c|
        if(ids[i][c] != ids[j][c])
          difference[c] += 1
        end
      end
      if difference.values.length == 1
        ids[i].slice!(difference.keys[0])
        puts ids[i]
      end
    end
  end
end

