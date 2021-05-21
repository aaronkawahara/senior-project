import sys, getopt
import math
from ctypes import c_uint16

line_length = 100
simple_image_def =\
    "pub trait SimpleImage {\n" +\
    "    const HEIGHT: u16;\n" +\
    "    const WIDTH: u16;\n\n" +\
    "    fn data_address(&self) -> u32;\n" +\
    "    fn data_address_offset(&self, x: u16, y: u16) -> u32 {\n" +\
    "        self.data_address() + (x + y * Self::WIDTH) as u32\n" +\
    "    }\n}\n\n"

def main(argv):
    usage = "make_array_from_bmp.py -o <output_file> <image_names> "
    fname = ""
    try:
        opts, image_names = getopt.getopt(argv, "hi:o:")
    except getopt.GetoptError:
        print(usage)
        sys.exit(2)
    
    if len(opts) != 1:
        print(usage)
        sys.exit(2)

    for opt, arg in opts:
        if opt == "-h":
            print(usage)
            sys.exit()
        elif opt == "-o":
            fname = arg
        else:
            print(usage)
            sys.exit(2)      
    
    fout = open(fname, "w")
    fout.write(simple_image_def);

    image_names = image_names[0].split(" ")

    for iname in image_names:
        width, height, pixel_data = get_raw_image_data(iname)
        write_array_to_file(fout, iname, width, height, pixel_data)

    fout.close()

def get_raw_image_data(image_name):
    with open(image_name, "rb") as image:
        width = c_uint16.from_buffer_copy(image.read(2)).value
        height = c_uint16.from_buffer_copy(image.read(2)).value
        pixel_data = bytearray(image.read())

    return width, height, pixel_data

def write_array_to_file(fout, image_name, width, height, pixel_data):
    array_len = len(pixel_data)
    indent = "    "
    chars_per_byte = 5
    bytes_per_line = int((line_length - len(indent)) / chars_per_byte)
    total_lines = math.ceil(array_len / bytes_per_line)
    image_name = image_name.split("\\")[-1][0: -4]
    image_name = image_name.split("_")
    data_name = []

    for i in range(len(image_name)):
        data_name.append(image_name[i].upper())
        image_name[i] = image_name[i].capitalize()

    image_name = "".join(image_name)
    data_name = "_".join(data_name)

    fout.write("#[derive(Clone, Copy)]\n");
    fout.write("pub struct {}Image;\n".format(image_name));
    fout.write("impl SimpleImage for {}Image {{\n".format(image_name));
    fout.write("    const WIDTH: u16 = {:d};\n".format(width));
    fout.write("    const HEIGHT: u16 = {:d};\n".format(height));
    fout.write("    fn data_address(&self) -> u32 {\n");
    fout.write("        unsafe {{ core::mem::transmute::<&u8, u32>(&{}_IMAGE_DATA[0]) }}\n".format(data_name));
    fout.write("    }\n");
    fout.write("}\n");
    fout.write("\n");
    fout.write("pub static {}_IMAGE_DATA: [u8; {:d}] = [\n".format(data_name, array_len))

    for curr_line in range(total_lines):
        fout.write(indent)

        for j in range(clamp(array_len - curr_line * bytes_per_line, 0, bytes_per_line)):
            pixel = pixel_data[j + bytes_per_line * curr_line]
            fout.write("{:<3d}, ".format(pixel))

        curr_line+=1
        fout.write("\n")

    fout.write("];\n\n")

def clamp(val, min, max):
    if val > max:
        return max

    if val < min:
        return min

    return val

if __name__ == "__main__":
    main(sys.argv[1:])