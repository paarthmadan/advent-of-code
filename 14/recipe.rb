class Integer
  def digits(base: 10)
    quotient, remainder = divmod(base)
    quotient == 0 ? [remainder] : [*quotient.digits(base: base), remainder]
  end
end


def generate_new_recipe_score
  one = @scores[@elf_one]
  two = @scores[@elf_two]
  new = (one + two).digits
  @scores += new
  @elf_one = new_position(@elf_one, one + 1)
  @elf_two = new_position(@elf_two, two + 1)
end

def new_position(old_index, steps)
  (old_index + steps) % @scores.length
end

def calculate_recipe_with_repetitions(reps)
  @scores = [3, 7]
  @elf_one = 0
  @elf_two = 1

  until @scores.length > (reps + 10)
    generate_new_recipe_score
    p @scores.length
  end
  p @scores[(reps)...(reps + 10)]
end

calculate_recipe_with_repetitions(846601)
