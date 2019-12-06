require 'set'

class Planet
  def initialize
    @parent = nil
    @orbits = []
  end

  def add_planet_to_orbit(planet)
    @orbits << planet
  end

  def set_parent(planet)
    @parent = planet
  end

  attr_reader :orbits, :parent
end


def create_graph
  {}.tap do |map|
    File.readlines('06_input').each do |line|
      parent, child = line.split(")").map { |planet| map[planet.strip] ||= Planet.new }
      parent.add_planet_to_orbit(child)
      child.set_parent(parent)
    end
  end
end


def count_number_of_orbits(planet)
  return planet.orbits.count + planet.orbits.sum { |pl| count_number_of_orbits(pl) }
end

def pt_1(map)
  count = 0

  map.values.each { |v| count += count_number_of_orbits(v) }

  count
end

def traverse_to_top_and_create_map_for(planet)
  planet_map = {}
  count = 0
  until planet.nil?
    planet = planet.parent
    planet_map[planet] = count
    count += 1
  end

  planet_map
end

def find_closest_parent(a_map, b_map)
  b_map.keys.each do |key|
    first_key = a_map.keys.find { |k| k == key }
    return first_key if first_key
  end
end

def find_shared_root(a, b)
  a_map = traverse_to_top_and_create_map_for(a)
  b_map = traverse_to_top_and_create_map_for(b)

  key = find_closest_parent(a_map, b_map)
  a_map[key] + b_map[key]
end

def pt_2(map)
  santa, you = map["SAN"], map["YOU"]
  find_shared_root(santa, you)
end

map = create_graph
puts pt_1(map)
puts pt_2(map)


