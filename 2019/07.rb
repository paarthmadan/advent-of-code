# Part 1

INTCODE = './05'
INPUT = '07_input'

def find_max_val
  max = -1
  [*0..4].permutation.each_with_index do |phase_setting, index|
    val = 0
    phase_setting.each { |phase| val = run([phase, val]) }
    max = val if val > max 
    puts (index.to_f / 119.to_f) * 100
  end
  max
end

def run(input)
  seed_input_file(input)
  run_command
end

def seed_input_file(input)
  `echo #{input[0]} > phase_setting && echo #{input[1]} >> phase_setting`
end

def run_command
  output = `#{INTCODE} #{INPUT} < phase_setting`
  output.chomp.to_i
end

find_max_val

