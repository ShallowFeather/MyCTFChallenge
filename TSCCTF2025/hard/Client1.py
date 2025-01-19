import socket
import struct

# 自行實現的 ChaCha20 加密器
class Chacha20:
    def __init__(self, key, nonce):
        self.state = self.initialize_state(key, nonce)
        self.keystream = b""
        self.position = 64  # 初始化位置，因為每次生成 64 字節

    @staticmethod
    def initialize_state(key, nonce):
        def pack4(data):
            return struct.unpack("<I", data)[0]

        magic_constant = b"Expand 32-byte k"
        return [
            pack4(magic_constant[0:4]),
            pack4(magic_constant[4:8]),
            pack4(magic_constant[8:12]),
            pack4(magic_constant[12:16]),
            pack4(key[0:4]),
            pack4(key[4:8]),
            pack4(key[8:12]),
            pack4(key[12:16]),
            pack4(key[16:20]),
            pack4(key[20:24]),
            pack4(key[24:28]),
            pack4(key[28:32]),
            0,  # Counter
            0,  # Counter high bits
            pack4(nonce[0:4]),
            pack4(nonce[4:8]),
        ]

    @staticmethod
    def quarter_round(state, a, b, c, d):
        state[a] = (state[a] + state[b]) & 0xFFFFFFFF
        state[d] ^= state[a]
        state[d] = ((state[d] << 16) | (state[d] >> 16)) & 0xFFFFFFFF

        state[c] = (state[c] + state[d]) & 0xFFFFFFFF
        state[b] ^= state[c]
        state[b] = ((state[b] << 12) | (state[b] >> 20)) & 0xFFFFFFFF

        state[a] = (state[a] + state[b]) & 0xFFFFFFFF
        state[d] ^= state[a]
        state[d] = ((state[d] << 8) | (state[d] >> 24)) & 0xFFFFFFFF

        state[c] = (state[c] + state[d]) & 0xFFFFFFFF
        state[b] ^= state[c]
        state[b] = ((state[b] << 7) | (state[b] >> 25)) & 0xFFFFFFFF

    def chacha20_block(self):
        working_state = self.state[:]
        for _ in range(10):
            self.quarter_round(working_state, 0, 4, 8, 12)
            self.quarter_round(working_state, 1, 5, 9, 13)
            self.quarter_round(working_state, 2, 6, 10, 14)
            self.quarter_round(working_state, 3, 7, 11, 15)
            self.quarter_round(working_state, 0, 5, 10, 15)
            self.quarter_round(working_state, 1, 6, 11, 12)
            self.quarter_round(working_state, 2, 7, 8, 13)
            self.quarter_round(working_state, 3, 4, 9, 14)

        return b"".join(
            struct.pack("<I", (working_state[i] + self.state[i]) & 0xFFFFFFFF)
            for i in range(16)
        )

    def generate_keystream(self):
        self.keystream = self.chacha20_block()
        self.state[12] = (self.state[12] + 1) & 0xFFFFFFFF
        if self.state[12] == 0:
            self.state[13] = (self.state[13] + 1) & 0xFFFFFFFF
        self.position = 0

    def crypt(self, data):
        output = bytearray()
        for byte in data:
            if self.position >= len(self.keystream):
                self.generate_keystream()
            output.append(byte ^ self.keystream[self.position])
            self.position += 1
        return bytes(output)


# 初始化密鑰和 nonce
key = bytes([0x73, 0x68, 0x61, 0x6c, 0x6c, 0x6f, 0x77, 0x66, 0x65, 0x61, 0x74, 0x68, 0x65, 0x72, 0xc8, 0x76,
             0x3, 0x73, 0x74, 0x61, 0x72, 0x62, 0x75, 0x72, 0x73, 0x74, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d])



try:
    recv1 = s.recv(14) # pcap 的檢查 bytestring
    rand1, rand2, rand3 = 0, 0, 0

    for i in range(0xFF):
        for j in range(0xFF):
            for k in range(0xFF):
                nonce = bytes([0x68, 0x61, 0x68, 0x61, 0x68, i, j, k])
                print(i, j, k)
                cipher = Chacha20(key, nonce)
                ciphertext = cipher.crypt(recv1)

                if ciphertext == b"shallowfeather":
                    print("密文:", ciphertext, i, j, k)
                    rand1, rand2, rand3 = i, j, k

                    with open("flag", "rb") as rb:
                        out = rb.read()

                    cipher = Chacha20(key, nonce)
                    decrypted_flag = cipher.crypt(out)
                    with open("flagDec", "wb") as wb:
                        wb.write(decrypted_flag)

                    exit(0)
except Exception as e:
    print(f"Error: {e}")
finally:
    s.close()
