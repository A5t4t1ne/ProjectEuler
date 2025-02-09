import numpy as np
from sortedcontainers import SortedList

def generate_sequence(n):
    S = [0] * n
    for k in range(1, 56):
        S[k - 1] = (100003 - 200003 * k + 300007 * k**3) % 1000000
    for k in range(56, n + 1):
        S[k - 1] = (S[k - 25] + S[k - 56]) % 1000000
    return S

def calculate_combined_volume(S, num_cuboids):
    events = []
    for n in range(1, num_cuboids + 1):
        x0 = S[6*n - 6] % 10000
        y0 = S[6*n - 5] % 10000
        z0 = S[6*n - 4] % 10000
        dx = 1 + (S[6*n - 3] % 399)
        dy = 1 + (S[6*n - 2] % 399)
        dz = 1 + (S[6*n - 1] % 399)
        x1, y1, z1 = x0 + dx, y0 + dy, z0 + dz
        
        events.append((z0, 0, x0, x1, y0, y1))
        events.append((z1, 1, x0, x1, y0, y1))

    events.sort()
    
    active = SortedList()
    total_volume = 0
    prev_z = events[0][0]

    for z, event_type, x0, x1, y0, y1 in events:
        if z > prev_z:
            total_volume += calculate_area(active) * (z - prev_z)
        
        if event_type == 0:  # Start of cuboid
            active.add((x0, x1, y0, y1))
        else:  # End of cuboid
            active.remove((x0, x1, y0, y1))
        
        prev_z = z

    return total_volume

def calculate_area(intervals):
    if not intervals:
        return 0
    
    events = []
    for x0, x1, y0, y1 in intervals:
        events.append((y0, 0, x0, x1))
        events.append((y1, 1, x0, x1))
    events.sort()

    active = SortedList()
    total_area = 0
    prev_y = events[0][0]

    for y, event_type, x0, x1 in events:
        if active:
            total_area += (y - prev_y) * calculate_length(active)
        
        if event_type == 0:  # Start of interval
            active.add((x0, x1))
        else:  # End of interval
            active.remove((x0, x1))
        
        prev_y = y

    return total_area

def calculate_length(intervals):
    if not intervals:
        return 0
    
    merged = [intervals[0]]
    for start, end in intervals[1:]:
        if start <= merged[-1][1]:
            merged[-1] = (merged[-1][0], max(merged[-1][1], end))
        else:
            merged.append((start, end))
    
    return sum(end - start for start, end in merged)

def main():
    S = generate_sequence(300000)
    
    volume_100 = calculate_combined_volume(S, 100)
    print(f"Combined volume of first 100 cuboids: {volume_100}")
    assert volume_100 == 723581599, "Verification failed for first 100 cuboids"

    volume_50000 = calculate_combined_volume(S, 50000)
    print(f"Combined volume of all 50000 cuboids: {volume_50000}")

if __name__ == "__main__":
    main()

