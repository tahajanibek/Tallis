#  Tallis  ![Rust](https://img.shields.io/badge/Rust-orange?logo=rust&logoColor=white) ![License](https://img.shields.io/badge/License-BSD_3--Clause-red?logo=freebsd&logoColor=red)

## 1. VLM Destekli E-Kitap ve Görsel Metin Çıkarma Motoru

**Tallis**, dijital kitapları (PDF, DjVu, EPUB vb.) ve yüksek çözünürlüklü belge görsellerini işlemek için tamamen yerel donanım üzerinde çalışan, performans ve hız odaklı hibrit bir metin çıkarma ve görsel tarama motorudur. Sıfır maliyetli soyutlama ve donanım hızlandırma desteği ile öne çıkar.

---

#### 🚀 Özellikler

- 🧠 **VLM Destekli Motor:** Yerel görsel-dil modelleriyle akıllı metin çıkarma
- 📖 **Çift Sayfa Ayrıştırma (`Storme`):** Görselleri sol ve sağ sayfa olarak bölme
- ⚡ **Optimize Boru Hattı:** `safetensors` ile güvenli ve hızlı model çalıştırma
- 🧱 **Rust:** Yüksek performans ve bellek güvenliği
- 🔄 **Modüler Yapı:** Tek komutla akademik korpus (veri seti) üretimi

---

### 🖥️ Kurulum

```bash
git clone [https://github.com/tahajanibek/Tallis.git](https://github.com/tahajanibek/Tallis.git)
cd Tallis
cargo build --release
```

⚙️ Kullanım

Derleme işlemi bittikten sonra projeyi target/release/ dizini üzerinden çalıştırabilirsiniz:
```Bash

./target/release/tallis [KOMUT] [SEÇENEKLER] [HEDEF_DIZIN]
```
Başlangıçta model kontrolü yapılır. Eğer core/models/ dizininde bir model bulunamazsa, terminal üzerinden doğrudan Hugging Face Hub aracılığıyla önerilen modeli indirmeniz istenecektir.

📦 Gereksinimler 
```
    Rust araçları (rustup, cargo)

    Uçbirim (Terminal/Konsol)

    Linux, Windows, macOS
```

### 🐧 Linux için gerekli kütüphaneler ve bağımlılıklar: ![Linux](https://img.shields.io/badge/Linux-yellow?logo=linux&logoColor=black)

  *Rust derleyicisi (rustc, cargo)*
  
  *Derleyici araçları (build-essential, gcc)*
  
  *Görsel ve sistem kütüphaneleri (pkg-config, libssl-dev)*
  
***Not: Rust projelerini derlemek için genel olarak bunlar gereklidir.***


### **Rust Kurulumu (Tüm Linux Dağıtımları İçin)**
```
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
```

**Debian / Ubuntu tabanlı sistemler**
```
sudo apt update
sudo apt install build-essential pkg-config libssl-dev
```

**Red Hat / Fedora (RPM) tabanlı sistemler** ![Red Hat](https://img.shields.io/badge/Red%20Hat-black?logo=redhat&logoColor=EE0000)
```
sudo dnf update
sudo dnf install gcc pkgconf-pkg-config openssl-devel
```

**Arch tabanlı sistemler için**
```
sudo pacman -S base-devel rust cargo
```

** macOS için** ![macOS](https://img.shields.io/badge/macOS-darkgray?logo=apple&logoColor=white)

Apple sistemlerinde Xcode komut satırı araçları ve Homebrew üzerinden Rust kurulumu önerilir:
```
xcode-select --install
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
```

**🪟 Windows 10/11**
1. Microsoft C++ Build Tools
Rust kodunun Windows üzerinde derlenebilmesi için (özellikle MSVC ortamında) C++ derleyicisine ihtiyaç duyar.
Kurulum:

        Rust Resmi Sitesinden rustup-init.exe dosyasını indirin ve çalıştırın.
    
        Eğer sisteminizde Build Tools yoksa, kurulum aracı sizi Microsoft C++ Build Tools sayfasına yönlendirecektir.
    
        İlgili sayfadan "C++ build tools" ve "Windows 10/11 SDK" bileşenlerini seçip kurun.
    
        Bilgisayarı yeniden başlatın ve terminalde cargo build --release komutunu çalıştırın.

---

## Quozart7: VLM Sayfa ve Toplu E-Kitap İşleme Motoru (quozart7)

Bu modül, taranmış çift sayfa kitap görsellerini sol ve sağ sayfa olarak ayırarak VLM modeli (örneğin Qwen2-VL veya Baidu Unlimited-OCR) ile okur. 
Amaç, ham taranmış görsellerin hızla dijital metin formatına (***Markdown ve TXT***) dönüştürülmesini sağlamaktır. Çoklu çekirdek desteği ile çalışır.

*🚀 Çalıştırılması*
```Bash

./target/release/tallis quozart7 -omega prefix_adi /gorsellerin/oldugu/dizin /ciktidizini
```
**Örnek Kullanım:**
```./target/release/tallis quozart7 -o -t kitap_sayfasi /home/user/scans /home/user/output```

**Not:**

```-o / omega``` bayrağı (%90 CPU kullanımı) ile yüksek paralelleştirme modunu açar.

```-t, --top```	Sayfa başlarında tekrarlayan üst bilgileri (headers) temizler.

```-j, --jpg```	Sadece JPG/JPEG formatındaki görselleri hedef alır.



##  Forge: Akademik Korpus Oluşturucu (forge)

Bu modül, hedef dizindeki (PDF, TXT, EPUB, JPG) dosyaları tarayarak içlerindeki metni çıkarır. 
İsteğe bağlı olaral çıkarılan metindeki sayfa başlıklarını (headers), dipnotları ve gereksiz akademik kalıntıları temizleyerek makine öğrenimi veya okuma için optimize edilmiş temiz bir korpus üretir.

*🚀 Çalıştırılması:*
```Bash

./target/release/tallis forge -t --omega proje_adi /hedef/belge/dizini
```

***Örnek Kullanım:***

```./target/release/tallis forge --omega -t tarih_tezi /belgeler/tez_kaynaklari```

**Not:**

```-o, --omega```	İşlemci sınırlarını zorlayarak maksimum hızda okuma yapar.

```-t, --top```	Sayfa başlıklarını (headers) temizler.

```--strip-footers```	Dipnotları ve sayfa numaralarını otomatik temizler (Varsayılan olarak aktiftir).

```-m, --model```	Özel bir .safetensors model yolu belirtmenizi sağlar.


##  Destiny: Dosya Sıralama Aracı (destiny)

Bu modül, hedef dizindeki belirtilen uzantıya sahip (örn. jpg) dosyaları, oluşturulma zamanlarına göre kronolojik olarak sıralar ve belirlediğiniz bir ön eke (prefix) göre çakışma olmadan yeniden adlandırır. OCR veya tarama işlemleri öncesi dosyaları düzene sokmak için kullanılır.

*🚀 Çalıştırılması:*
```Bash

./target/release/tallis destiny jpg sayfa /siralanacak/gorsel/dizini
```

Örnek Kullanım: (Klasördeki tüm karmaşık isimli .jpg dosyalarını sayfa_1.jpg, sayfa_2.jpg şeklinde sıralar)

```./target/release/tallis destiny jpg sayfa /siralanacak/gorsel/dizini```

---

## 🧠 Geliştirici Notları

Tallis tarafından geliştirilen bu yazılım, açık kaynaklı ve geliştirilmeye açıktır.

Görsel veri akışlarını, yapay zeka modelleriyle yerel donanımda tamamen izole bir şekilde harmanlamak hedeflenmiştir.

----

## 📜 Lisans 

             BSD 3-CLAUSE LİSANSI (Üç Maddeli BSD Lisansı)
             
    Copyright (C) 2026 Taha Janibek
    Tüm hakları saklıdır.
    
    Bu yazılım ("Tallis" ve ilgili tüm kaynak kodları/bileşenleri) aşağıdaki şartların 
    yerine getirilmesi koşuluyla, kaynak ve ikili biçimde yeniden dağıtılabilir ve 
    kullanılabilir:
    
    1. Kaynak kodun yeniden dağıtımı yukarıdaki telif hakkı bildirimini, bu şartlar 
       listesini ve aşağıdaki sorumluluk reddini içermelidir.
    2. İkili biçimde yeniden dağıtım, belgelendirmeyle birlikte sunulan materyallerde 
       yukarıdaki telif hakkı bildirimini, bu şartlar listesini ve aşağıdaki sorumluluk 
       reddini içermelidir.
    3. Telif hakkı sahiplerinin özel önceden yazılı izni olmaksızın, bu yazılımdan türetilen 
       ürünleri onaylamak veya teşvik etmek için Taha Janibek adı veya katkıda 
       bulunanların adları kullanılamaz.
    
    BU YAZILIM, TELİF HAKKI SAHİPLERİ VE KATKIDA BULUNANLAR TARAFINDAN "OLDUĞU GİBİ" 
    SUNULMUŞTUR VE TİCARETE ELVERİŞLİLİK VEYA BELİRLİ BİR AMACA UYGUNLUK İÇİN ZIMNİ 
    GARANTİLER DAHİL OLMAK ÜZERE, ANCAK BUNUNLA SINIRLI OLMAMAK ÜZERE HER TÜRLÜ 
    AÇIK VEYA ZIMNİ GARANTİ REDDEDİLİR. HİÇBİR DURUMDA TELİF HAKKI SAHİBİ VEYA 
    KATKIDA BULUNANLAR; DOĞRUDAN, DOLAYLI, ARIZİ, ÖZEL, ÖRNEK TEŞKİL EDEN VEYA 
    NETİCEDE OLUŞAN ZARARLARDAN (HİZMET VEYA VERİ KAYBI, KÂR KAYBI VEYA İŞ 
    KESİNTİSİ DAHİL OLMAK ÜZERE) BUNUN KÖKENİ NE OLURSA OLSUN VE HANGİ SORUMLULUK 
    TEORİSİNDE OLURSA OLSUN, SÖZLEŞMEDE, KUSURSUZ SORUMLULUKTA VEYA HAKSIZ FİİLDE 
    (İHMAL VEYA BAŞKA BİR ŞEKİLDE) BU YAZILIMIN KULLANIMINDAN HİÇBİR ŞEKİLDE 
    DOĞMUŞ OLSUN, BU TÜR ZARARLARIN OLASILIĞI BİLDİRİLMİŞ OLSA BİLE SORUMLU TUTULAMAZ.
    
    Bu lisansın aslı `LICENSE` dosyası olarak projede yer almaktadır. Alternatif olarak 
    resmi metne <https://opensource.org/licenses/BSD-3-Clause> adresinden ulaşabilirsiniz.
    
    NOT: Bu Türkçe çeviri, bilgilendirme amaçlıdır. Yasal bağlayıcılığı olan sürüm,
    İngilizce orijinal LICENSE dosyasıdır.
    
    Bu proje, BSD 3-Clause Lisansı ile lisanslanmıştır.


# ⚠️ Sorumluluk Reddi Beyanı

Tallis, yerel donanımınız üzerinde tamamen izole bir şekilde çalışır ve internet üzerinden herhangi bir dış sunucuya veri aktarımı veya analizi yapmaz. Modellerin indirilmesi haricinde süreç tamamen yereldir.
Kullanıcı, bu yazılımı kullanarak işlediği telif hakkına tabi e-kitaplar, taranmış akademik materyaller veya belgeler üzerinde yapılan işlemlerden ve bunların dağıtımından tamamen kendisi sorumludur.
**Geliştirici Taha Janibek, bu yazılımın herhangi bir kötüye kullanımından (telif hakkı ihlali, yetkisiz belge işleme vb.) dolayı sorumluluk kabul etmez.**
***Büyük hacimli VLM (safetensors vb.) dosyalarının indirilmesi, sistem belleğinin (RAM/VRAM) yüksek oranda kullanılması ve işlemci sınırlarının zorlanması kullanıcının kontrolündedir. Sistem kaynaklarının aşırı kullanımından doğabilecek stabilite sorunlarında sorumluluk kullanıcıya aittir.***
Araç, hem kişisel arşiv yönetimi hem de akademik araştırmalarda yapılandırılmış veri setleri oluşturmak amacıyla tasarlanmıştır.


## 👤 Geliştirici

**Taha Janibek**

<div style="margin-bottom: 8px;">
  <a href="#">
    <span style="display: inline-flex; align-items: stretch; border-radius: 3px; overflow: hidden; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif; font-size: 11px; font-weight: 700; line-height: 1.3; text-shadow: 0 1px 0 rgba(0,0,0,0.2);">
      <span style="background-color: #24292e; display: flex; align-items: center; justify-content: center; padding: 4px 6px;">
        <img src="https://raw.githubusercontent.com/tahajanibek/tatnet-ethash/refs/heads/main/assets/eklipse_red_24.svg" height="13" style="width: auto; display: block;" />
      </span>
      <span style="background-color: #444444; color: #ffffff; display: flex; align-items: center; padding: 4px 8px;">
        Eklipse
      </span>
    </span>
  </a>
</div>    

**📧 tahajanibek@mail.ru**    
🌐 [tahajanibek.asia](tahajanibek.asia)
