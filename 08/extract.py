import numpy as np
from PIL import Image

IMG_NAME = 'C:/Users/nicom/371os/07/bars.ppm'

img = np.array(Image.open(IMG_NAME))

for i in np.nditer(img):
    print(hex(i))