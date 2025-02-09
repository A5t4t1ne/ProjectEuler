max_num = 17

num = max_num
loop do
  if (1..max_num).all? { |i| (num % i).zero? }
    puts num
    break
  end
  num += 1
end

