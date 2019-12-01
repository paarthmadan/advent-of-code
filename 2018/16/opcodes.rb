def create_opcode(&process)
  Proc.new do |register, operations|
    register[operations[3]] = yield(register, operations)
  end
end

addr = create_opcode { |reg, op| reg[op[1]] + reg[op[2]] }
addi = create_opcode { |reg, op| reg[op[1]] + op[2] }

mulr = create_opcode { |reg, op| reg[op[1]] * reg[op[2]] }
muli = create_opcode { |reg, op| reg[op[1]] * op[2] }

banr = create_opcode { |reg, op| reg[op[1]] & reg[op[2]] }
bani = create_opcode { |reg, op| reg[op[1]] & op[2] }

borr = create_opcode { |reg, op| reg[op[1]] | reg[op[2]] }
bori = create_opcode { |reg, op| reg[op[1]] | op[2] }

setr = create_opcode { |reg, op| reg[op[1]] }
seti = create_opcode { |reg, op| op[1] }

gtir = create_opcode { |reg, op| (op[1] > reg[op[2]]) ? 1 : 0 }
gtri = create_opcode { |reg, op| (reg[op[1]] > op[2]) ? 1 : 0 }
gtrr = create_opcode { |reg, op| (reg[op[1]] > reg[op[2]]) ? 1 : 0 }

eqir = create_opcode { |reg, op| (op[1] == reg[op[2]]) ? 1 : 0 }
eqri = create_opcode { |reg, op| (reg[op[1]] == op[2]) ? 1 : 0 }
eqrr = create_opcode { |reg, op| (reg[op[1]] == reg[op[2]]) ? 1 : 0 }

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

CHART = [13, 9, 14, 15, 1, 8, 12, 11, 3, 7, 5, 6, 10, 4, 0, 2]
final_reg = [0, 0, 0, 0]

File.readlines("part_two").each do |line|
  operation = line.strip!.split(" ").map(&:to_i)
  OPCODES[CHART[operation[0]]].call(final_reg, operation)
end

p final_reg
