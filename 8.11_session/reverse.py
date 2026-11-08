def reverse(a):
    left = 0
    right = len(a) - 1

    while left < right:
        temp = a[left]
        a[left] = a[right]
        a[right] = temp
        left += 1
        right -=1

artem = [5, 2, 8, 1, 9, 4, 0, -12]
reverse(artem)
print(artem)