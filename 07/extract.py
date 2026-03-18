import numpy as np
from PIL import Image as im
import requests
from io import BytesIO

def main(url):

    response = requests.get(url)
    img = im.open(BytesIO(response.content))
    img_arr =

    """
    size = (720, 400)

    catresponse = requests.get('https://cd-public.github.io/ai101/images/photo-cat.jpg')
    rainbowresponse = requests.get('https://cd-rs.github.io/os/img/rainbow.jpg')
    mountainresponse = requests.get('https://www.leadvilletwinlakes.com/wp-content/themes/yootheme/cache/ba/View-of-Mount-Massive-LCTP-Cropped-scaled-ba58e696.webp')

    catimg = im.open(BytesIO(catresponse.content))
    rainbowimg = im.open(BytesIO(rainbowresponse.content))
    mountainimg = im.open(BytesIO(mountainresponse.content))

    catimg_bytes = np.array(catimg).astype(np.uint8)
    rainbowimg_bytes = np.array(rainbowimg).astype(np.uint8)
    mountainimg_bytes = np.array(mountainimg).astype(np.uint8)

    fcat = im.fromarray(catimg_bytes)
    fmount = im.fromarray(mountainimg_bytes)
    frain = im.fromarray(rainbowimg_bytes)

    rc= fcat.resize(size, im.LANCZOS)
    rm = fmount.resize(size, im.LANCZOS)
    rr = frain.resize(size, im.LANCZOS)

    rac = np.array(rc)
    ram = np.array(rm)
    rar = np.array(rr)"""



if __name__ == "__main__":
    main()
        