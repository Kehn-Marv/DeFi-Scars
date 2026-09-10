#!/usr/bin/env python3
"""Generate the Global Memories app icon: a neon-cyan globe on a dark rounded tile.

Renders at 4x then downsamples for clean anti-aliasing. Outputs build/icon.png
(1024x1024 master); .icns and .ico are produced by the surrounding shell script.
"""
import math
from PIL import Image, ImageDraw, ImageFilter

S = 1024          # final size
SS = 4            # supersample factor
N = S * SS        # working size

CYAN = (0, 243, 255)

img = Image.new("RGBA", (N, N), (0, 0, 0, 0))
draw = ImageDraw.Draw(img)

# --- Rounded tile background with a radial-ish dark gradient ---
radius = int(N * 0.22)
# Base vertical gradient from #0a0a12 to pure black
bg = Image.new("RGBA", (N, N), (0, 0, 0, 255))
bgd = ImageDraw.Draw(bg)
for y in range(N):
    t = y / N
    r = int(10 * (1 - t))
    g = int(10 * (1 - t))
    b = int(18 * (1 - t))
    bgd.line([(0, y), (N, y)], fill=(r, g, b, 255))
# Mask to rounded rect
mask = Image.new("L", (N, N), 0)
ImageDraw.Draw(mask).rounded_rectangle([0, 0, N - 1, N - 1], radius=radius, fill=255)
img.paste(bg, (0, 0), mask)
draw = ImageDraw.Draw(img)

# --- Globe geometry ---
cx, cy = N / 2, N / 2
R = N * 0.33

# Soft cyan glow behind the globe
glow = Image.new("RGBA", (N, N), (0, 0, 0, 0))
gd = ImageDraw.Draw(glow)
gd.ellipse([cx - R * 1.25, cy - R * 1.25, cx + R * 1.25, cy + R * 1.25],
           fill=(0, 243, 255, 70))
glow = glow.filter(ImageFilter.GaussianBlur(radius=N * 0.04))
img = Image.alpha_composite(img, glow)
draw = ImageDraw.Draw(img)

# Globe disc (dark navy fill)
draw.ellipse([cx - R, cy - R, cx + R, cy + R], fill=(2, 12, 28, 255))

lw = max(2, int(N * 0.006))   # line width

def arc_line(bbox, start, end):
    draw.arc(bbox, start, end, fill=CYAN, width=lw)

# Outer rim
draw.ellipse([cx - R, cy - R, cx + R, cy + R], outline=CYAN, width=int(lw * 1.6))

# Latitude lines (flattened ellipses at fractional heights)
for frac in (-0.66, -0.33, 0.0, 0.33, 0.66):
    y = cy + frac * R
    # half-width of the globe at this latitude
    hw = R * math.cos(math.asin(max(-1, min(1, frac))))
    rh = R * 0.16 * (1 - abs(frac) * 0.3)   # vertical thickness of the ellipse
    draw.ellipse([cx - hw, y - rh, cx + hw, y + rh], outline=CYAN, width=lw)

# Longitude lines (meridians as vertical ellipses of varying width)
for frac in (-0.66, -0.33, 0.33, 0.66):
    rw = R * abs(frac)
    draw.ellipse([cx - rw, cy - R, cx + rw, cy + R], outline=CYAN, width=lw)
# Central vertical meridian
draw.line([(cx, cy - R), (cx, cy + R)], fill=CYAN, width=lw)

# A bright marker "pin" dot on the globe (nod to the app's memory markers)
pin_x = cx + R * 0.34
pin_y = cy - R * 0.42
pr = N * 0.018
# glow
pglow = Image.new("RGBA", (N, N), (0, 0, 0, 0))
ImageDraw.Draw(pglow).ellipse([pin_x - pr * 2.4, pin_y - pr * 2.4,
                               pin_x + pr * 2.4, pin_y + pr * 2.4],
                              fill=(0, 243, 255, 150))
pglow = pglow.filter(ImageFilter.GaussianBlur(radius=N * 0.012))
img = Image.alpha_composite(img, pglow)
draw = ImageDraw.Draw(img)
draw.ellipse([pin_x - pr, pin_y - pr, pin_x + pr, pin_y + pr], fill=(255, 255, 255, 255))

# Downsample
final = img.resize((S, S), Image.LANCZOS)
final.save("build/icon.png")

# Windows .ico (multi-size)
final.save("build/icon.ico", sizes=[(16, 16), (32, 32), (48, 48), (64, 64),
                                     (128, 128), (256, 256)])
print("wrote build/icon.png and build/icon.ico")
