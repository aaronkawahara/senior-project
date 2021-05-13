import sys, getopt
import math
from ctypes import c_uint16

def main(argv):
    usage = "make_array_from_bmp.py -i <image_name> -o <output_file>"
    image_name = ""
    fname = ""
    try:
        opts, args = getopt.getopt(argv, "hi:o:")
    except getopt.GetoptError:
        print(usage)
        sys.exit(2)
    
    if len(opts) != 2:
        print(usage)
        sys.exit(2)

    for opt, arg in opts:
        if opt == "-h":
            print(usage)
            sys.exit()
        elif opt == "-i":
            image_name = arg
        elif opt == "-o":
            fname = arg
        else:
            print(usage)
            sys.exit(2)      

    width, height, pixel_data = get_raw_image_data(image_name)
    write_array_to_file(fname, image_name, width, height, pixel_data)

def get_raw_image_data(image_name):
    with open(image_name, "rb") as image:
        width = c_uint16.from_buffer_copy(image.read(2)).value
        height = c_uint16.from_buffer_copy(image.read(2)).value
        pixel_data = bytearray(image.read())

    return width, height, pixel_data

def write_array_to_file(fname, image_name, width, height, pixel_data):
    array_len = len(pixel_data)
    indent = "    "
    chars_per_byte = 5
    line_length = 100
    bytes_per_line = int((line_length - len(indent)) / chars_per_byte)
    total_lines = math.ceil(array_len / bytes_per_line)
    image_name = image_name.split("\\")[-1][0: -4].upper()

    with open(fname, 'a') as f:
        f.write("pub const {}_IMAGE_WIDTH: u16 = {:d};\n".format(image_name, width))
        f.write("pub const {}_IMAGE_HEIGHT: u16 = {:d};\n".format(image_name, height))
        f.write("pub static {}_IMAGE_DATA: [u8; {:d}] = [\n".format(image_name, array_len))

        for curr_line in range(total_lines):
            f.write(indent)

            for j in range(clamp(array_len - curr_line * bytes_per_line, 0, bytes_per_line)):
                pixel = pixel_data[j + bytes_per_line * curr_line]
                f.write("{:3d}, ".format(pixel))

            curr_line+=1
            f.write("\n")

        f.write("];\n\n")

def clamp(val, min, max):
    if val > max:
        return max

    if val < min:
        return min

    return val

if __name__ == "__main__":
    main(sys.argv[1:])