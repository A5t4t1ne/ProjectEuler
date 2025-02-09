palindrome = []

min_digit = 100
max_digit = 999

for i in range(min_digit, max_digit + 1):
    for j in range(min_digit, max_digit + 1):
        product = j * i
        sproduct = str(product)
        
        if sproduct[::-1] == sproduct:
            palindrome.append(product)

print(sorted(palindrome))
print(sorted(palindrome)[-1])
