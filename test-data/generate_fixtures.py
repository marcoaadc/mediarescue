"""Generate corrupted test fixtures for MediaRescue.

Creates intentionally corrupted media files and a small disk image
for testing the recovery engine. Uses only Python stdlib.

Run: python test-data/generate_fixtures.py
"""
import os
import struct
import zlib

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
CORRUPTED_DIR = os.path.join(SCRIPT_DIR, "corrupted")
IMAGES_DIR = os.path.join(SCRIPT_DIR, "disk-images")


def ensure_dirs():
    os.makedirs(CORRUPTED_DIR, exist_ok=True)
    os.makedirs(IMAGES_DIR, exist_ok=True)


def write_file(path: str, data: bytes):
    with open(path, "wb") as f:
        f.write(data)
    print(f"  Created {path} ({len(data)} bytes)")


def make_truncated_jpeg():
    """JPEG with SOI + JFIF APP0 header but no EOI — truncated mid-stream."""
    buf = bytearray()
    buf += b"\xff\xd8"  # SOI
    # APP0 JFIF marker
    app0_data = b"JFIF\x00\x01\x01\x00\x00\x01\x00\x01\x00\x00"
    buf += b"\xff\xe0"
    buf += struct.pack(">H", len(app0_data) + 2)
    buf += app0_data
    # Fake DQT
    buf += b"\xff\xdb"
    dqt = bytes(range(64))
    buf += struct.pack(">H", len(dqt) + 3)
    buf += b"\x00" + dqt
    # Fake SOF0 (8x8, 3 components)
    buf += b"\xff\xc0"
    sof_data = b"\x08\x00\x08\x00\x08\x03\x01\x11\x00\x02\x11\x01\x03\x11\x01"
    buf += struct.pack(">H", len(sof_data) + 2)
    buf += sof_data
    # SOS marker start, then abrupt truncation (no EOI)
    buf += b"\xff\xda"
    buf += struct.pack(">H", 12)
    buf += b"\x03\x01\x00\x02\x11\x03\x11\x00\x3f\x00"
    buf += os.urandom(200)  # random "scan data" — truncated
    write_file(os.path.join(CORRUPTED_DIR, "truncated.jpg"), bytes(buf))


def make_broken_png():
    """PNG with valid header/IHDR but corrupted IDAT CRC, and valid IEND."""
    buf = bytearray()
    buf += b"\x89PNG\r\n\x1a\n"  # PNG signature

    # IHDR: 4x4 RGBA
    ihdr_data = struct.pack(">IIBBBBB", 4, 4, 8, 6, 0, 0, 0)
    ihdr_crc = zlib.crc32(b"IHDR" + ihdr_data) & 0xFFFFFFFF
    buf += struct.pack(">I", len(ihdr_data))
    buf += b"IHDR" + ihdr_data
    buf += struct.pack(">I", ihdr_crc)

    # IDAT with WRONG CRC
    raw_row = b"\x00" + b"\xff\x00\x00\xff" * 4  # filter=None, red pixels
    raw_data = raw_row * 4
    compressed = zlib.compress(raw_data)
    buf += struct.pack(">I", len(compressed))
    buf += b"IDAT" + compressed
    buf += struct.pack(">I", 0xDEADBEEF)  # intentionally wrong CRC

    # IEND
    iend_crc = zlib.crc32(b"IEND") & 0xFFFFFFFF
    buf += struct.pack(">I", 0)
    buf += b"IEND"
    buf += struct.pack(">I", iend_crc)

    write_file(os.path.join(CORRUPTED_DIR, "broken_idat.png"), bytes(buf))


def make_no_moov_mp4():
    """MP4 with valid ftyp box but missing moov — only has mdat with junk."""
    buf = bytearray()

    # ftyp box
    ftyp_payload = b"isom\x00\x00\x02\x00isomiso2mp41"
    ftyp_size = 8 + len(ftyp_payload)
    buf += struct.pack(">I", ftyp_size)
    buf += b"ftyp" + ftyp_payload

    # mdat box with random data (no moov!)
    mdat_data = os.urandom(1024)
    mdat_size = 8 + len(mdat_data)
    buf += struct.pack(">I", mdat_size)
    buf += b"mdat" + mdat_data

    write_file(os.path.join(CORRUPTED_DIR, "no_moov.mp4"), bytes(buf))


def make_good_jpeg():
    """Minimal valid 8x8 red JPEG for comparison testing."""
    buf = bytearray()
    buf += b"\xff\xd8"  # SOI

    # APP0 JFIF
    app0 = b"JFIF\x00\x01\x01\x00\x00\x01\x00\x01\x00\x00"
    buf += b"\xff\xe0" + struct.pack(">H", len(app0) + 2) + app0

    # DQT — all-ones quantization table
    dqt = b"\x00" + bytes([1] * 64)
    buf += b"\xff\xdb" + struct.pack(">H", len(dqt) + 2) + dqt

    # SOF0 — 8x8 grayscale (1 component, simpler)
    sof = b"\x08\x00\x08\x00\x08\x01\x01\x11\x00"
    buf += b"\xff\xc0" + struct.pack(">H", len(sof) + 2) + sof

    # DHT — minimal DC Huffman table
    dht = b"\x00"  # class 0, id 0
    dht += bytes([0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])  # counts
    dht += b"\x00"  # symbol
    buf += b"\xff\xc4" + struct.pack(">H", len(dht) + 2) + dht

    # SOS
    sos_header = b"\x01\x01\x00\x00\x3f\x00"
    buf += b"\xff\xda" + struct.pack(">H", len(sos_header) + 2) + sos_header
    # Minimal scan data (DC=0 for all blocks means mid-gray)
    buf += b"\x7f\xff\xd9"  # data byte + EOI

    write_file(os.path.join(CORRUPTED_DIR, "good_reference.jpg"), bytes(buf))


def make_disk_image():
    """64KB disk image with embedded media signatures at known sector offsets."""
    sector_size = 512
    total_sectors = 128  # 128 * 512 = 64KB
    image = bytearray(sector_size * total_sectors)

    # Sector 10: JPEG SOI + JFIF APP0 marker
    offset_jpeg = 10 * sector_size
    image[offset_jpeg:offset_jpeg + 4] = b"\xff\xd8\xff\xe0"
    image[offset_jpeg + 4:offset_jpeg + 6] = struct.pack(">H", 16)
    image[offset_jpeg + 6:offset_jpeg + 11] = b"JFIF\x00"
    # Fill rest of sector with plausible JPEG data
    for i in range(11, sector_size):
        image[offset_jpeg + i] = (i * 7) & 0xFF

    # Sector 30: PNG signature
    offset_png = 30 * sector_size
    image[offset_png:offset_png + 8] = b"\x89PNG\r\n\x1a\n"
    # Fake IHDR chunk
    ihdr = struct.pack(">IIBBBBB", 4, 4, 8, 2, 0, 0, 0)
    image[offset_png + 8:offset_png + 12] = struct.pack(">I", len(ihdr))
    image[offset_png + 12:offset_png + 16] = b"IHDR"
    image[offset_png + 16:offset_png + 16 + len(ihdr)] = ihdr

    # Sector 50: MP4 ftyp box
    offset_mp4 = 50 * sector_size
    ftyp = b"isom\x00\x00\x02\x00"
    ftyp_size = 8 + len(ftyp)
    image[offset_mp4:offset_mp4 + 4] = struct.pack(">I", ftyp_size)
    image[offset_mp4 + 4:offset_mp4 + 8] = b"ftyp"
    image[offset_mp4 + 8:offset_mp4 + 8 + len(ftyp)] = ftyp

    path = os.path.join(IMAGES_DIR, "test_small.dd")
    write_file(path, bytes(image))


if __name__ == "__main__":
    print("Generating MediaRescue test fixtures...\n")
    ensure_dirs()
    make_truncated_jpeg()
    make_broken_png()
    make_no_moov_mp4()
    make_good_jpeg()
    make_disk_image()
    print("\nDone! All fixtures generated.")
