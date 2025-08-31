**MNIST 檔案格式 (IDX format)**，我來完整拆解：

---

## 1. 檔案種類

MNIST 資料集共有四個主要檔案，都是 **big-endian binary (高位在前)**，遵循 **IDX file format**：

1. **train-images-idx3-ubyte**

   * 訓練影像，60,000 張手寫數字 (28x28)。

2. **train-labels-idx1-ubyte**

   * 訓練影像對應的標籤，60,000 筆 (0–9)。

3. **t10k-images-idx3-ubyte**

   * 測試影像，10,000 張。

4. **t10k-labels-idx1-ubyte**

   * 測試標籤，10,000 筆。

---

## 2. 影像檔案格式 (idx3-ubyte)

檔頭 (header) 長度 **16 bytes**：

| 位元組範圍  | 說明                                                                      |
| ------ | ----------------------------------------------------------------------- |
| 0–3    | **Magic number** = 0x00000803 (十進位 2051)。0x08=unsigned byte, 0x03=三維資料。 |
| 4–7    | **圖像數量** (number of images)。                                            |
| 8–11   | **行數** (rows, 高度)，MNIST 為 28。                                           |
| 12–15  | **列數** (cols, 寬度)，MNIST 為 28。                                           |
| 16–end | **影像資料**，每個像素一個 byte (0–255 灰階)，先 row 再 col。                            |

舉例：
`t10k-images-idx3-ubyte` →

* Magic = 2051
* 圖像數 = 10000
* Rows = 28
* Cols = 28
* 每張圖 784 bytes，共 7840000 bytes，加 header 16 bytes = 7840016 bytes。

---

## 3. 標籤檔案格式 (idx1-ubyte)

檔頭 (header) 長度 **8 bytes**：

| 位元組範圍 | 說明                                                                      |
| ----- | ----------------------------------------------------------------------- |
| 0–3   | **Magic number** = 0x00000801 (十進位 2049)。0x08=unsigned byte, 0x01=一維資料。 |
| 4–7   | **標籤數量** (number of items)。                                             |
| 8–end | **標籤資料**，每個標籤佔 1 byte，數字範圍 0–9。                                         |

舉例：
`t10k-labels-idx1-ubyte` →

* Magic = 2049
* 標籤數 = 10000
* 資料大小 = 10000 bytes，加 header 8 bytes = 10008 bytes。

---

## 4. 公式化格式 (IDX 定義)

IDX 格式由 **magic number** 定義資料型別與維度：

* **高兩個 byte** → 保留 (通常是 0x0000)。
* **第三個 byte** → 資料型別 (data type)。
* **第四個 byte** → 維度數 (dimension)。

### 常見 data type 編碼

| 值    | 型別                    |
| ---- | --------------------- |
| 0x08 | unsigned byte (uint8) |
| 0x09 | signed byte (int8)    |
| 0x0B | short (int16)         |
| 0x0C | int (int32)           |
| 0x0D | float (float32)       |
| 0x0E | double (float64)      |

### 範例

* 0x00000803 → unsigned byte, 維度=3 (images: \[num, rows, cols])。
* 0x00000801 → unsigned byte, 維度=1 (labels: \[num])。

---

## 5. 簡單驗證

Linux 檢查檔案大小是否正確：

```bash
ls -lh t10k-images-idx3-ubyte
# 預期: 7.5M (7840016 bytes)

ls -lh t10k-labels-idx1-ubyte
# 預期: 10K (10008 bytes)
```

---

## Usage

This tool provides two subcommands to handle MNIST data files: `unpack` and `pack`.

### Unpack

Unpacks MNIST `.ubyte` files into PNG images and text files containing labels.

**Command:**
```bash
cargo run -- unpack [OPTIONS]
```

**Options:**
*   `--input-dir <PATH>`: Specifies the directory containing the MNIST `.ubyte` files. Defaults to `./data`.
*   `--output-dir <PATH>`: Specifies the directory to save the unpacked PNG images and label files. Defaults to `./output`.

**Example:**
```bash
# Unpack using default directories
cargo run -- unpack

# Unpack using custom directories
cargo run -- unpack --input-dir ./my_mnist_data --output-dir ./unpacked_images
```

### Pack

Packs PNG images and label files back into MNIST `.ubyte` format.

**Command:**
```bash
cargo run -- pack [OPTIONS]
```

**Options:**
*   `--input-dir <PATH>`: Specifies the directory containing the PNG images and label files. Defaults to `./output`.
*   `--output-dir <PATH>`: Specifies the directory to save the packed `.ubyte` files. Defaults to `./output`.

**Example:**
```bash
# Pack using default directories
cargo run -- pack

# Pack using custom directories
cargo run -- pack --input-dir ./unpacked_images --output-dir ./packed_files
```


