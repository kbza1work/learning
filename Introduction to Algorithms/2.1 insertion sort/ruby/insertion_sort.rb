#!/usr/bin/env ruby

class InsertionSort
  def self.insertion_sort(sequence)
    for j in 1...sequence.length
    element = sequence[j]
    i = j - 1
    while i >= 0 && sequence[i] > element
      sequence[i + 1] = sequence[i]
      i -= 1
    end
    sequence[i + 1] = element
    end
    sequence
  end
end

sequence = ARGV.map{|string| string.to_i}
sequence = InsertionSort::insertion_sort(sequence)
puts sequence.join(" ")
