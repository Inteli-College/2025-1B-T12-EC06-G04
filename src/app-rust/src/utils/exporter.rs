use std::{fs::File, io::{BufWriter, Write}, path::PathBuf, process::Command};

use tempfile::NamedTempFile;

pub fn export(md_content: &str, file_type: &str) {
    let md_content = md_content.to_string();
    let file_type = file_type.to_string();

    std::thread::spawn(move || {
        let file_type_lower = file_type.to_lowercase();
        let file_type_upper = file_type.to_uppercase();

        if let Some(path) = rfd::FileDialog::new()
            .set_title(&format!("Salvar arquivo {} como...", file_type_lower))
            .add_filter(&file_type_upper, &[&file_type_lower])
            .set_file_name(&format!("Relatorio.{}", &file_type_lower))
            .save_file()
        {
            let path = PathBuf::from(path);
            if &file_type_lower == "md" {
                let new_file = File::create(&path).unwrap();
                let mut writer = BufWriter::new(new_file);
                writer.write_all(md_content.as_bytes()).unwrap();
            } else {
                let mut temp_md = NamedTempFile::new().unwrap();
                write!(temp_md, "{}", md_content).unwrap();
                temp_md.flush().unwrap();

                let status = Command::new("pandoc")
                    .arg(temp_md.path())
                    .arg("-o")
                    .arg(&path)
                    .status()
                    .expect(&format!("Falha ao gerar {} com pandoc", &file_type_upper));

                if !status.success() {
                    eprintln!("Erro ao converter Markdown para {} com pandoc", &file_type_upper);
                }
            }
        }
    });
}