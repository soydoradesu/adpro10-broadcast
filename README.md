## 2.1 Original code, and how it run

### Server
![alt text](image.png)

### Clients
![alt text](image-1.png)
![alt text](image-2.png)
![alt text](image-3.png)

Ketika seorang klien mengirim pesan, server akan menjadi pihak pertama yang menerima pesan tersebut. Setelah itu, server akan mendistribusikan pesan tersebut ke semua klien yang sedang terhubung, termasuk si pengirim. Hal ini dimungkinkan karena server menyimpan daftar seluruh koneksi aktif dan terus memantau aktivitas mereka, sehingga saat ada pesan masuk, server dapat langsung menyebarkannya ke semua klien.