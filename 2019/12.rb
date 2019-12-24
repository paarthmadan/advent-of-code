class Moon
  class << self
    def apply_gravity(a, b)
      if a.x > b.x
        a.dx -= 1
        b.dx += 1
      elsif a.x < b.x
        b.dx -= 1
        a.dx += 1
      end

      if a.y > b.y
        a.dy -= 1
        b.dy += 1
      elsif a.y < b.y
        b.dy -= 1
        a.dy += 1
      end

      if a.z > b.z
        a.dz -= 1
        b.dz += 1
      elsif a.z < b.z
        b.dz -= 1
        a.dz += 1
      end
    end
  end

  attr_reader :x, :y, :z, :dx, :dy, :dz
  attr_writer :dx, :dy, :dz

  def initialize(x, y, z)
    @x = x
    @y = y
    @z = z
    @dx = 0
    @dy = 0
    @dz = 0
  end

  def apply_velocity
    @x += @dx
    @y += @dy
    @z += @dz
  end

  def to_s
    format("pos=<x=%d, y=%d, z=%d>, vel=<x=%d, y=%d, z=%d>", x, y, z, dx, dy, dz)
  end

  def total_energy
    potential_energy * kinetic_energy
  end

  def potential_energy
    positions.sum(&:abs)
  end

  def kinetic_energy
    velocities.sum(&:abs)
  end

  private

  def positions
    [x, y, z]
  end

  def velocities
    [dx, dy, dz]
  end
end

moons = []

$stdin.readlines.map(&:strip).each do |line|
  args = *line.scan(/<x=(.*) y=(.*), z=(.*)>/).first.map(&:to_i)
  moons << Moon.new(*args)
end

1000.times do
  moons.combination(2).each { |pair| Moon.apply_gravity(*pair) }
  moons.each { |moon| moon.apply_velocity }
  moons.each { |moon| puts moon }
  puts
end

puts moons.sum(&:total_energy)
