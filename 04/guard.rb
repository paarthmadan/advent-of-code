require 'date'

records = Hash.new { |h, k| h[k] = [] }

File.readlines("input.txt").each do |line|
  data = line.split("]")
  date_time = DateTime.parse(data[0][1..-1])
  date = Date.parse(date_time.strftime("%Y-%m-%d"))
  time = date_time.strftime("%H%M").to_i

  records[date].push([time, data[1].strip])
end


sorted_dates = records.keys.sort

sorted_dates.each do |date|
  records[date].sort_by! { |a| a[0] }
end


guard_sleep_history = Hash.new { |h, k| h[k] = Array.new(60, 0) }

temporary_guard = -1

records.each do |date, record|
  record.each_with_index do |entry, index|
    time = entry[0]
    action = entry[1]
    temporary_guard = action.tr!('^0-9', '').to_i if action.include?("#")
    if action == "falls asleep"
      wakes_up_time = record[index + 1][0]
      (time...wakes_up_time).each do |t|
        guard_sleep_history[temporary_guard][t] += 1
      end
    end
  end
end

max = guard_sleep_history.max_by { |k, v| v.inject(:+)}

max_guard = max[0]
max_minute = guard_sleep_history[max_guard].each_with_index.max[1]

puts max_guard * max_minute
