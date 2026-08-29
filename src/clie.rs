use clap::ValueEnum;
use clap::{ArgAction, Args, Parser, Subcommand};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Default)]
pub enum Lang {
    #[default]
    Tr,
    En,
}

impl Lang {
    pub fn t(&self, key: &str) -> &'static str {
        match (key, self) {
            ("destiny_start", Lang::Tr) => "dosya bulundu. İşlem başlıyor...",
            ("destiny_start", Lang::En) => "files were found. Processing is starting...",
            ("destiny_warn", Lang::Tr) => "Uyarı: Dosya taşınamadı:",
            ("destiny_warn", Lang::En) => "Warning: Could not move file:",
            ("destiny_done", Lang::Tr) => "İşlem tamamlandı: Dosya yeniden adlandırıldı.",
            ("destiny_done", Lang::En) => "Operation completed: files were renamed.",

            ("forge_model_found", Lang::Tr) => "Bilgi: Model bulundu. İşleme alınılıyor...",
            ("forge_model_found", Lang::En) => "Info: Model found. Processing...",
            ("forge_model_missing", Lang::Tr) => {
                "Bilgi: Model bulunamadı. Standart metin ayıklama modu aktif."
            }
            ("forge_model_missing", Lang::En) => {
                "Info: Model not found. Standard text extraction mode active."
            }
            ("forge_scanning", Lang::Tr) => "Forge Korpus Motoru Çalışıyor...",
            ("forge_scanning", Lang::En) => "Forge Corpus Engine Running...",
            ("forge_done", Lang::Tr) => "İşlem Başarıyla Tamamlandı!",
            ("forge_done", Lang::En) => "Operation Successfully Completed!",

            ("quozart_scanning", Lang::Tr) => "Belgeler Taranıyor ve İşleniyor...",
            ("quozart_scanning", Lang::En) => "Scanning and Processing Documents...",
            ("quozart_done", Lang::Tr) => "İşlem Tamamlandı!",
            ("quozart_done", Lang::En) => "Operation Completed!",

            ("no_files", Lang::Tr) => "HATA: Belirtilen dizinde uygun formatta dosya bulunamadı.",
            ("no_files", Lang::En) => "ERROR: No supported files found in the specified directory.",

            ("report_file", Lang::Tr) => "Rapor Dosyası:",
            ("report_file", Lang::En) => "Report File:",

            ("model_not_found", Lang::Tr) => {
                "\n[!] Dikkat: 'core/models/' dizininde geçerli bir .safetensors modeli bulunamadı.\nİndirmek istediğiniz modeli seçin veya kendi özel modelinizi 'core/models/' içine atın:"
            }
            ("model_not_found", Lang::En) => {
                "\n[!] Notice: No valid .safetensors model found in 'core/models/'.\nSelect a model to download, or place your custom model into 'core/models/':"
            }

            ("prompt_select", Lang::Tr) => "\nSeçiminiz [1-2]: ",
            ("prompt_select", Lang::En) => "\nSelect option [1-2]: ",

            ("aborted", Lang::Tr) => "İşlem iptal edildi: Model seçilmedi veya bulunamadı.",
            ("aborted", Lang::En) => "Execution aborted: No model selected or found.",

            ("downloading", Lang::Tr) => "Hugging Face Hub üzerinden indiriliyor...",
            ("downloading", Lang::En) => "Downloading from Hugging Face Hub...",

            ("success_download", Lang::Tr) => "Model başarıyla indirildi ve kaydedildi:",
            ("success_download", Lang::En) => "Model successfully downloaded and saved to:",

            ("venexus_init_error", Lang::Tr) => "Venexus motoru başlatılamadı:",
            ("venexus_init_error", Lang::En) => "Venexus engine failed to start:",

            ("pdf_detected", Lang::Tr) => "PDF dosyası algılandı:",
            ("pdf_detected", Lang::En) => "PDF file detected:",

            ("left_page_error", Lang::Tr) => "Sol Sayfa Okuma Hatası",
            ("left_page_error", Lang::En) => "Left Page Read Error",

            ("right_page_error", Lang::Tr) => "Sağ Sayfa Okuma Hatası",
            ("right_page_error", Lang::En) => "Right Page Read Error",

            ("saving_markdown", Lang::Tr) => {
                "İşleme tamamlandı, veriler Markdown olarak kaydediliyor..."
            }
            ("saving_markdown", Lang::En) => "Processing complete, saving data as Markdown...",

            ("success_label", Lang::Tr) => "Başarılı",
            ("success_label", Lang::En) => "Success",

            ("fail_label", Lang::Tr) => "Başarısız",
            ("fail_label", Lang::En) => "Failed",

            ("md_data", Lang::Tr) => "Veri",
            ("md_data", Lang::En) => "Data",

            ("md_source", Lang::Tr) => "Kaynak",
            ("md_source", Lang::En) => "Source",

            ("md_mode", Lang::Tr) => "Mod",
            ("md_mode", Lang::En) => "Mode",

            ("pdf_extract_error", Lang::Tr) => "PDF Metin Çıkarma Hatası:",
            ("pdf_extract_error", Lang::En) => "PDF Text Extraction Error:",

            ("image_file_pointer", Lang::Tr) => "Görsel / Dosya İşaretçisi:",
            ("image_file_pointer", Lang::En) => "Image / File Pointer:",

            ("forge_saving_corpus", Lang::Tr) => {
                "İşlem tamamlandı, akademik korpus diske yazılıyor..."
            }
            ("forge_saving_corpus", Lang::En) => {
                "Process completed, writing academic corpus to disk..."
            }

            ("forge_success_count", Lang::Tr) => "Başarılı Çıktı Dosyası Sayısı",
            ("forge_success_count", Lang::En) => "Successful Output Files",

            ("corpus_doc", Lang::Tr) => "Korpus Belgesi",
            ("corpus_doc", Lang::En) => "Corpus Document",

            ("download_failed", Lang::Tr) => "İndirme başarısız:",
            ("download_failed", Lang::En) => "Download failed:",

            ("error_tokenizer_load_failed", Lang::Tr) => "Tokenizer yüklenemedi",
            ("error_tokenizer_load_failed", Lang::En) => "Failed to load tokenizer",

            ("error_token_decode_failed", Lang::Tr) => "Token çözme hatası",
            ("error_token_decode_failed", Lang::En) => "Token decoding error",

            ("error_vision_encoder_forward_failed", Lang::Tr) => "Görsel kodlayıcı işleme hatası",
            ("error_vision_encoder_forward_failed", Lang::En) => "Vision encoder forward failed",

            ("error_no_safetensors_found", Lang::Tr) => {
                "Klasörde hiçbir .safetensors dosyası bulunamadı!"
            }
            ("error_no_safetensors_found", Lang::En) => {
                "No .safetensors files found in the directory!"
            }

            ("metal_active", Lang::Tr) => "[+] Apple Metal (MPS) Hızlandırması Aktif!",
            ("metal_active", Lang::En) => "[+] Apple Metal (MPS) Acceleration Active!",

            ("cuda_active", Lang::Tr) => "[+] NVIDIA CUDA Hızlandırması Aktif!",
            ("cuda_active", Lang::En) => "[+] NVIDIA CUDA Acceleration Active!",

            ("cpu_active", Lang::Tr) => "[-] GPU bulunamadı, CPU modunda çalışıyor.",
            ("cpu_active", Lang::En) => "[-] No GPU found, running in CPU mode.",

            _ => "",
        }
    }
}

#[derive(Parser)]
#[command(name = "tallis")]
#[command(version = "mk-1.0")]
#[command(author = "tahajanibek")]
#[command(
    about = "2026 🄯 Tallis - Local AI-Powered Dual-Page OCR, Academic Corpus Generator & Batch File Program."
)]
pub struct Cli {
    #[arg(short, long, value_enum, default_value_t = Lang::Tr, global = true, help = "Dil seçimi (tr / en)")]
    pub dil: Lang,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Forge(ForgeArgs),
    Destiny(DestinyArgs),
    Quozart7(Quozart7Args),
}

#[derive(Args)]
pub struct ForgeArgs {
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub jpg: bool,

    #[arg(
        short,
        long,
        default_value_t = false,
        help = "Sayfa başındaki tekrarlayan başlıkları (headers) temizle"
    )]
    pub top: bool,

    #[arg(
        long,
        default_value_t = true,
        help = "Dipnotları ve sayfa numaralarını temizle (varsayılan: true)"
    )]
    pub strip_footers: bool,

    #[arg(
        short,
        long,
        help = "Özel model dosyası yolu (Belirtilmezse core/models/ içi taranır)"
    )]
    pub model: Option<String>,

    pub prefix: String,
    pub directory: String,

    #[arg(short, long, default_value_t = false, help = "%90 CPU usage")]
    pub omega: bool,
}

#[derive(Args)]
pub struct DestinyArgs {
    pub extension: String,

    pub prefix: String,

    pub directory: String,
}

#[derive(Args)]
pub struct Quozart7Args {
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub jpg: bool,

    #[arg(short, long, default_value_t = false)]
    pub top: bool,

    pub prefix: String,
    pub directory: String,

    pub output_dir: Option<String>,

    #[arg(short, long, default_value_t = false, help = "%90 CPU usage")]
    pub omega: bool,
}
