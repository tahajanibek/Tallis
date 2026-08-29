use image::{DynamicImage, GenericImageView};

pub struct Storme;

impl Storme {
    pub fn sayfayi_bol(img: &DynamicImage) -> (DynamicImage, DynamicImage) {
        let (w, h) = img.dimensions();

        let sol_sayfa = img.crop_imm(0, 0, w / 2, h);
        let sag_sayfa = img.crop_imm(w / 2, 0, w / 2, h);

        (sol_sayfa, sag_sayfa)
    }

    pub fn akademik_temizle(ham_veri: &str, strip_headers: bool, strip_footers: bool) -> String {
        let mut temiz_paragraflar = Vec::new();
        let mut mevcut_paragraf = Vec::new();
        let satirlar: Vec<&str> = ham_veri.lines().collect();
        let toplam_satir = satirlar.len();

        for (i, satir) in satirlar.iter().enumerate() {
            let s = satir.trim();

            if s.is_empty() {
                if !mevcut_paragraf.is_empty() {
                    temiz_paragraflar.push(mevcut_paragraf.join(" "));
                    mevcut_paragraf.clear();
                }
                continue;
            }

            if (s
                .chars()
                .all(|c| c.is_ascii_digit() || c.is_whitespace() || c == '-' || c == '—'))
                && s.len() < 8
            {
                continue;
            }

            let sayfa_kenarinda_mi = i < 3 || i > toplam_satir - 4;
            if strip_headers
                && sayfa_kenarinda_mi
                && s.split_whitespace().count() < 6
                && (s.chars().all(|c| c.is_uppercase())
                    || s.contains("Chapter")
                    || s.contains("Bölüm")
                    || s.contains("Section"))
            {
                continue;
            }

            if strip_footers
                && ((s.starts_with('[') && s.contains(']') && s.len() < 30)
                    || (s.starts_with(|c: char| c.is_ascii_digit()) && s.contains('.')))
                && s.len() < 40
            {
                continue;
            }

            mevcut_paragraf.push(s);
        }

        if !mevcut_paragraf.is_empty() {
            temiz_paragraflar.push(mevcut_paragraf.join(" "));
        }

        temiz_paragraflar.join("\n\n")
    }
}
