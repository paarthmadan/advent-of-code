@last = []

def generate_new_recipe_score
  one = @scores[@elf_one]
  two = @scores[@elf_two]
  new = one + two
  new.to_s.chars.map(&:to_i).each do |i|
    @scores << i
    @last << i
  end
  @elf_one = new_position(@elf_one, one + 1)
  @elf_two = new_position(@elf_two, two + 1)
end

def new_position(old_index, steps)
  (old_index + steps) % @scores.length
end

def reset
  @scores = [3, 7]
  @elf_one = 0
  @elf_two = 1
  @last = [3, 7]
end

def calculate_recipe_with_repetitions(reps)
  reset

  until @scores.length > (reps + 10)
    generate_new_recipe_score
  end
  p @scores[(reps)...(reps + 10)]
end

calculate_recipe_with_repetitions(846601)

def find_number_of_recipes_for(pattern)
  pattern = pattern.to_s.chars.map(&:to_i)
  reset
  until @last == pattern
    generate_new_recipe_score
    unless @last.length <= pattern.length
      until @last.length == pattern.length
        @last.shift
      end
    end
  end
  p @scores.length - pattern.length
end

find_number_of_recipes_for(51589)
find_number_of_recipes_for(01245)
find_number_of_recipes_for(92510)
find_number_of_recipes_for(59414)
find_number_of_recipes_for(846601)
