import struct, json, numpy as np

f = open(r"C:\SarahCore\Genlex_Map.json", "r")
d = json.load(f)
arr = [x for x in d["Engine_Sectors"]["Gemma_4B"]["Arrays"] if x["Name"]=="blk.0.attn_k.weight"][0]
print(f"Type: {arr['Type']}, Dims: {arr['Dims']}, Offset: {arr['Offset']}")

fv = open(r"C:\SarahCore\Sovereign_Hybrid_13B.genlex", "rb")
fv.seek(arr["Offset"])
block = fv.read(176)
d_scale = struct.unpack("<e", block[0:2])[0]
d_min = struct.unpack("<e", block[2:4])[0]
print(f"d={d_scale}, dmin={d_min}")
print(f"block[0:16] = {list(block[0:16])}")
print(f"Is d NaN: {np.isnan(d_scale)}, Is dmin NaN: {np.isnan(d_min)}")

# Check multiple blocks
nan_count = 0
for i in range(100):
    fv.seek(arr["Offset"] + i * 176)
    b = fv.read(4)
    ds = struct.unpack("<e", b[0:2])[0]
    dm = struct.unpack("<e", b[2:4])[0]
    if np.isnan(ds) or np.isnan(dm):
        nan_count += 1
        if nan_count <= 3:
            print(f"  Block {i}: d={ds}, dmin={dm}, raw={list(b)}")

print(f"\nNaN blocks in first 100: {nan_count}")
