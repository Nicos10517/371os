import numpy as np
from PIL import Image as im
import requests
from io import BytesIO
import os

#This is definitely my least organized code so far lol sorry about that

size = (720, 400)

catresponse = requests.get('https://cd-public.github.io/ai101/images/photo-cat.jpg')
rainbowresponse = requests.get('https://cd-rs.github.io/os/img/rainbow.jpg')
mountainresponse = requests.get('https://www.leadvilletwinlakes.com/wp-content/themes/yootheme/cache/ba/View-of-Mount-Massive-LCTP-Cropped-scaled-ba58e696.webp')

img = im.open(BytesIO(catresponse.content)).convert('RGB').resize((80, 25), im.LANCZOS)
rac = np.array(img).reshape(-1, 3).astype(float)

bars = im.open('bars.ppm')
palette = np.array(bars)[0][::45].astype(float)

diff = rac[:, np.newaxis, :] - palette
indices = np.argmin(np.sum(diff**2, axis=2), axis=1)

hex_values = [f"0x{i:02X}" for i in indices]

with open(r'C:\Users\nicom\371os\07\src\colors\img.rs', 'w') as f:
    f.write(f"pub const CAT_DATA: [u8; 2000] = [{', '.join(hex_values)}];")


print("yay")


# catimg = im.open(BytesIO(catresponse.content))
# rainbowimg = im.open(BytesIO(rainbowresponse.content))
# mountainimg = im.open(BytesIO(mountainresponse.content))

# catimg_bytes = np.array(catimg).astype(np.uint8)
# rainbowimg_bytes = np.array(rainbowimg).astype(np.uint8)
# mountainimg_bytes = np.array(mountainimg).astype(np.uint8)

# fcat = im.fromarray(catimg_bytes)
# fmount = im.fromarray(mountainimg_bytes)
# frain = im.fromarray(rainbowimg_bytes)

# rc= fcat.resize(size, im.LANCZOS)
# rm = fmount.resize(size, im.LANCZOS)
# rr = frain.resize(size, im.LANCZOS)

# rac = np.array(rc)
# ram = np.array(rm)
# rar = np.array(rr)

# bars = im.open('bars.ppm')

# bars_load = np.array(bars)
# pixel_list = bars_load.reshape(-1,3)

# palette = np.unique(pixel_list, axis = 0)

# height = rac.shape[0]
# width = rac.shape[1]

# def color_match(pixel, palette):
#     diff = palette.astype(float) - pixel.astype(float)
#     distances = np.sum(diff**2, axis=1)
#     return palette[np.argmin(distances)]


# new_cat = np.zeros((height, width, 3), dtype=np.uint8)
# new_mount = np.zeros((height, width, 3), dtype=np.uint8)
# new_rain = np.zeros((height, width, 3), dtype=np.uint8)


# for y in range(height):
#     for x in range(width):
#         current_pixel = rac[y, x]
#         new_cat[y, x] = color_match(current_pixel, palette)

# im.fromarray(new_cat).show()

# output = r'C:\Users\nicom\371os\07\src\colors'
# file_path = os.path.join(output, 'img.rs')
# data = new_cat.flatten()
# content = ", ".join(map(str, data))