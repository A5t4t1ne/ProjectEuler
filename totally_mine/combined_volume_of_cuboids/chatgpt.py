import numpy as np

def generate_sequence():
    S = np.zeros(300000, dtype=int)
    for k in range(1, 56):
        S[k-1] = (100003 - 200003 * k + 300007 * k**3) % 1000000
    for k in range(56, 300001):
        S[k-1] = (S[k-24-1] + S[k-55-1]) % 1000000
    return S

def generate_cuboids(S):
    cuboids = []
    for n in range(1, 50001):
        x0 = S[6*n-6] % 10000
        y0 = S[6*n-5] % 10000
        z0 = S[6*n-4] % 10000
        dx = 1 + (S[6*n-3] % 399)
        dy = 1 + (S[6*n-2] % 399)
        dz = 1 + (S[6*n-1] % 399)
        cuboids.append((x0, y0, z0, x0+dx, y0+dy, z0+dz))
    return cuboids

# A helper function to compute the union volume of cuboids using a sweep line algorithm.
from sortedcontainers import SortedDict

def compute_combined_volume(cuboids):
    events = []
    for x0, y0, z0, x1, y1, z1 in cuboids:
        events.append((x0, 1, y0, y1, z0, z1))  # Start of a cuboid
        events.append((x1, -1, y0, y1, z0, z1))  # End of a cuboid

    events.sort()  # Sort events by x-coordinate, breaking ties by type (start before end)

    def add_interval(active, start, end, delta):
        active[start] = active.get(start, 0) + delta
        if active[start] == 0:
            del active[start]
        active[end] = active.get(end, 0) - delta
        if active[end] == 0:
            del active[end]

    def compute_active_length(active):
        length = 0
        prev = None
        count = 0
        for pos in sorted(active.keys()):
            if count > 0:
                length += pos - prev
            count += active[pos]
            prev = pos
        return length

    active_yz = SortedDict()
    prev_x = None
    volume = 0

    for x, typ, y0, y1, z0, z1 in events:
        if prev_x is not None and active_yz:
            active_z = SortedDict()
            prev_y = None
            yz_area = 0

            for y, delta in active_yz.items():
                if prev_y is not None and compute_active_length(active_z):
                    yz_area += y - prev_y
                if delta > 0:
                    add_interval(active_z, z0, z1, delta)
                prev_y = y

            volume += yz_area * (x - prev_x)

        add_interval(active_yz, y0, y1, typ)
        prev_x = x

    return volume

# Main calculation
S = generate_sequence()
cuboids = generate_cuboids(S)
combined_volume = compute_combined_volume(cuboids)
print(combined_volume)

