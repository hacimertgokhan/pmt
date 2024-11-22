### Port Management Tool (PMT)

PMT, port ve işlem yönetimi için tasarlanmış, basit ve kullanışlı bir CLI (Komut Satırı Arayüzü) aracıdır. Bu araç, belirli işlemleri yönetmenize, açık portları listelemenize ve belirli bir portu kullanan işlemi sonlandırmanıza olanak tanır.

---

## **Kurulum**

Bu projeyi kullanmak için Rust ve Cargo'nun sisteminizde yüklü olması gerekmektedir. Yükleme adımları:

1. Projeyi klonlayın:
   ```bash
   git clone https://github.com/hacimertgokhan/pmt.git
   cd pmt
   ```

2. Projeyi derleyin:
   ```bash
   cargo build --release
   ```

3. Uygulamayı çalıştırmaya hazır hale getirin:
   ```bash
   cd target/release
   ```

---

## **Kullanım**

PMT, çeşitli komutlar ve alt komutlar içerir. Komutlar şu şekildedir:

### **1. Pid Yönetimi**
İşlem ID'lerini (PID) listelemek veya belirli bir işlem adının PID'lerini bulmak için kullanılır.

- Tüm işlemleri listele:
  ```bash
  ./pmt pids
  ```
- Belirli bir işlemin PID'lerini listele:
  ```bash
  ./pmt pids <process_name>
  ```
  Örnek:
  ```bash
  ./pmt pids firefox
  ```

---

### **2. Açık Portları Listeleme**
Sistemdeki tüm açık portları listelemek için kullanılır:
```bash
./pmt ports
```

---

### **3. Port Kullanan İşlemi Sonlandırma**
Belirli bir portu kullanan işlemi sonlandırmak için kullanılır:
```bash
./pmt kill --port <port_number>
```
Örnek:
```bash
./pmt kill --port 8080
```

---

## **Özellikler**
- **Açık Port Yönetimi**: Sistemdeki açık portları kolayca listeleme.
- **İşlem Yönetimi**: İşlem adlarına göre PID'leri listeleme.
- **Port Sonlandırma**: Belirli bir portu kullanan işlemi hızlıca sonlandırma.

---

## **Katkıda Bulunma**
Bu projeye katkıda bulunmak için şu adımları izleyebilirsiniz:
1. Bu projeyi forklayın.
2. Geliştirmek istediğiniz özelliği ekleyin.
3. Değişikliklerinizi test edin ve bir PR (Pull Request) oluşturun.

---

## **Lisans**
Bu proje MIT Lisansı ile lisanslanmıştır. Daha fazla bilgi için `LICENSE` dosyasını inceleyebilirsiniz.
