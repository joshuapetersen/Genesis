import os
import mmap
import time
import struct
from AERIS_Chat import AERISAlpha

def start_mmap_kernel():
    bridge_path = r"C:\Genlex_Linear\lattice_bridge.bin"
    bridge_size = 1024 * 1024 * 5  # 5 MB to hold massive context prompts
    
    # Initialize the binary file if it doesn't exist
    if not os.path.exists(bridge_path):
        import numpy as np
        arr = np.memmap(bridge_path, dtype='uint8', mode='w+', shape=(bridge_size,))
        arr[0] = 0 # IDLE
        arr.flush()
        del arr

    print("[MMAP KERNEL] Binding to lattice_bridge.bin...")
    
    # Open the memory-mapped file
    with open(bridge_path, "r+b") as f:
        mm = mmap.mmap(f.fileno(), bridge_size, access=mmap.ACCESS_WRITE)
    
    # Initialize Aeris Core (Layer 12)
    print("[MMAP KERNEL] Initializing Native Genlex Cortex...")
    core = AERISAlpha()
    print("[MMAP KERNEL] Sovereign Membrane Sealed. Spinning for Neural Input...")
    
    # STATES:
    # 0 = IDLE
    # 1 = VSCODE_WROTE_PROMPT
    # 2 = PYTHON_WROTE_TOKEN
    # 3 = PYTHON_DONE
    # 4 = VSCODE_ACK_TOKEN

    try:
        while True:
            # Check state byte
            state = mm[0]
            
            if state == 1:
                # VSCODE_WROTE_PROMPT
                # Read length
                payload_len = struct.unpack('<I', mm[1:5])[0]
                prompt_bytes = mm[5:5 + payload_len]
                prompt = prompt_bytes.decode('utf-8', errors='ignore')
                
                print(f"[MMAP KERNEL] Signal Received: {len(prompt)} bytes.")
                
                # Execute Neural Generation
                try:
                    response_block = core.generate_response(prompt)
                    # Fake streaming by yielding words to maintain the UI illusion without breaking the bridge
                    words = response_block.split(' ')
                    for i, word in enumerate(words):
                        token = word + (" " if i < len(words) - 1 else "")
                        token_bytes = token.encode('utf-8')
                        t_len = len(token_bytes)
                        
                        # Write Token payload length
                        mm[1:5] = struct.pack('<I', t_len)
                        # Write Token bytes
                        mm[5:5 + t_len] = token_bytes
                        # Set state to 2 (PYTHON_WROTE_TOKEN)
                        mm[0] = 2
                        
                        # Spinwait for VSCODE to ACK (state == 4)
                        timeout = time.time() + 5.0
                        while mm[0] == 2:
                            if time.time() > timeout:
                                # Timeout waiting for VSCode, abort stream
                                break
                            time.sleep(0.0005) # Hyper-fast 0.5ms spinloop
                            
                        # If node didn't ack in time or disconnected, abort stream
                        if mm[0] != 4:
                            break
                            
                except Exception as e:
                    error_msg = f"\n[KERNEL PANIC] {str(e)}".encode('utf-8')
                    mm[1:5] = struct.pack('<I', len(error_msg))
                    mm[5:5 + len(error_msg)] = error_msg
                    mm[0] = 2 # Write error as token
                    
                    # Spinwait for ack
                    while mm[0] == 2:
                        time.sleep(0.001)
                
                # Stream finished
                mm[0] = 3 # PYTHON_DONE
                
                # Wait for VSCode to read the DONE state and reset it
                while mm[0] == 3:
                    time.sleep(0.005)
                
                print("[MMAP KERNEL] Stream completed. Idling.")
                
            # Idle spin
            time.sleep(0.005)
            
    except KeyboardInterrupt:
        print("\n[MMAP KERNEL] Shutting down...")
    finally:
        mm.close()

if __name__ == "__main__":
    start_mmap_kernel()
