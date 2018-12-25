def create_opcode(&process)
  Proc.new do |register, operations|
    yield(register, operations)
  end
end

addr = create_opcode { |reg, op| reg[op[3]] = reg[op[1]] + reg[op[2]] }
addi = create_opcode { |reg, op| reg[op[3]] = reg[op[1]] + op[2] }

mulr = create_opcode { |reg, op| reg[op[3]] = reg[op[1]] * reg[op[2]] }
muli = create_opcode { |reg, op| reg[op[3]] = reg[op[1]] * op[2] }

banr = create_opcode { |reg, op| reg[op[3]] = reg[op[1]] & reg[op[2]] }
bani = create_opcode { |reg, op| reg[op[3]] = reg[op[1]] & op[2] }

borr = create_opcode { |reg, op| reg[op[3]] = reg[op[1]] | reg[op[2]] }
bori = create_opcode { |reg, op| reg[op[3]] = reg[op[1]] | op[2] }

setr = create_opcode { |reg, op| reg[op[3]] = reg[op[1]] }
seti = create_opcode { |reg, op| reg[op[3]] = op[1] }

gtir = create_opcode { |reg, op| reg[op[3]] = (op[1] > reg[op[2]]) ? 1 : 0 }
gtri = create_opcode { |reg, op| reg[op[3]] = (reg[op[1]] > op[2]) ? 1 : 0 }
gtrr = create_opcode { |reg, op| reg[op[3]] = (reg[op[1]] > reg[op[2]]) ? 1 : 0 }

eqir = create_opcode { |reg, op| reg[op[3]] = (op[1] == reg[op[2]]) ? 1 : 0 }
eqri = create_opcode { |reg, op| reg[op[3]] = (reg[op[1]] == op[2]) ? 1 : 0 }
eqrr = create_opcode { |reg, op| reg[op[3]] = (reg[op[1]] == reg[op[2]]) ? 1 : 0 }

OPCODES = [addr, addi, mulr, muli, banr, bani, borr, bori, setr, seti, gtir, gtri, gtrr, eqir, eqri, eqrr]
OPCODES_MAP = Hash.new { |h, k| h[k] = [] }

def determine_opcode_count(before, operations, after)
  count = 0

  OPCODES.each.with_index do |opcode, index|
    register = before.dup
    opcode.call(register, operations)
    if register == after
      count += 1
      OPCODES_MAP[index] << operations[0]
    end
  end

  count
end

input = []
File.readlines("part_one").each { |line| input << line }

three_count = 0

(input.length / 4).times do |i|
  m = i * 4
  before = eval(input[m].split(":")[1].strip!)
  operation = input[m + 1].split(" ").map(&:to_i)
  after = eval(input[m + 2].split(":")[1].strip!)
  three_count += 1 if determine_opcode_count(before, operation, after) >= 3
end

puts three_count

def print_map
  OPCODES_MAP.each do |key, value|
     freq = value.inject(Hash.new(0)) { |h, v| h[v] += 1; h }
     p "#{key} --> #{freq}"
   end
  puts "––––––––––––––––––––––––––––"
end

def delete(q)
  OPCODES_MAP.each { |key, value| value.tap {|x| x.delete(q)}}
  print_map
end

delete(10)
delete(13)
delete(5)
delete(12)
delete(6)
delete(0)
delete(2)
delete(3)
delete(7)
delete(1)
delete(9)
delete(11)
delete(14)
delete(8)
