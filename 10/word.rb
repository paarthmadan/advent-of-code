class Point
  def initialize(xp, yp, xv, yv)
    @xp = xp
    @yp = yp
    @xv = xv
    @yv = yv
  end

  def point
    [@xp, @yp]
  end

  def apply_velocity
    @xp += @xv
    @yp += @yv
  end

end

points = []
THRESHOLD = 20

File.readlines("input").each do |line|
  data = line.match(/position=<([^>]*)> velocity=<([^>]*)>/).captures
  data[0] = data[0].tr(" ", "").split(",").map(&:to_i)
  data[1] = data[1].tr(" ", "").split(",").map(&:to_i)
  points << Point.new(data[0][0], data[0][1], data[1][0], data[1][1])
end

50000.times do |_|
  h = Hash.new(".")
  x = Hash.new(0)
  y = Hash.new(0)

  max = [-Float::MAX.to_i, -Float::MAX.to_i]
  min = [Float::MAX.to_i, Float::MAX.to_i]
  points.each do |p|
    c = p.point
    h[c] = "#"
    max = [ [max[0], c[0]].max, [max[1], c[1]].max ]
    min = [ [min[0], c[0]].min, [min[1], c[1]].min ]
    x[c[0]] += 1
    y[c[1]] += 1
  end

  if(x.values.max > THRESHOLD && y.values.max > THRESHOLD)
    puts _
    min[1].upto(max[1]).each do |y|
      min[0].upto(max[0]).each do |x|
        print h[[x,y]]
      end
      puts
    end
    puts "––––––––––––––––––––––––––––––––––––––––"
  end

  points.each { |p| p.apply_velocity }
end
