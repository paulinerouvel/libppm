from ctypes import cdll

lib = cdll.LoadLibrary("../target/release/libppm.dylib")

# res = lib.dummy()

# print(str(res) + " done!")

#lib.image(23, 30)

# res2.show()

print(lib.dummy())
lib.Image.new(255,255,255)
print("Fini !")