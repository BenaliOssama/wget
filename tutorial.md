### 1. **Basic Download**

Download a single file:

```bash
wget http://example.com/file.zip
```

* This saves `file.zip` in your current directory.

---

### 2. **Download with a Different Filename**

```bash
wget -O myfile.zip http://example.com/file.zip
```

* `-O` lets you rename the file while downloading.

---

### 3. **Download in Background**

```bash
wget -b http://example.com/file.zip
```

* `-b` downloads in the background.
* Logs are saved to `wget-log`.

---

### 4. **Resume an Interrupted Download**

```bash
wget -c http://example.com/file.zip
```

* `-c` continues where it left off if the download was interrupted.

---

### 5. **Download an Entire Website**

```bash
wget -r -np -k http://example.com
```

* `-r`: recursive (download linked pages)
* `-np`: no parent (don’t go to upper directories)
* `-k`: convert links for offline viewing

---

### 6. **Limit Download Speed**

```bash
wget --limit-rate=100k http://example.com/file.zip
```

* Limits download speed to 100 KB/s.

---

### 7. **Download with Username and Password**

```bash
wget --user=username --password=password http://example.com/securefile.zip
```

---

### 8. **Download Multiple Files**

Create a file `urls.txt` with one URL per line, then:

```bash
wget -i urls.txt
```

---

**Documentation:** [GNU Wget Manual](https://www.gnu.org/software/wget/manual/wget.html)


