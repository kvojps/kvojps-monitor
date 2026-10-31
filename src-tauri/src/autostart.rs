use std::io;
use std::path::{Path, PathBuf};

/// Como este processo foi lançado.
pub struct Execucao {
    /// Binário em execução (`/usr/bin/kvojps-monitor` no `.deb`).
    pub executavel: PathBuf,
    /// Caminho do arquivo `.AppImage` (variável `APPIMAGE`), quando roda como AppImage.
    pub appimage: Option<PathBuf>,
}

const ARQUIVO: &str = "kvojps-monitor.desktop";

/// Liga o início com o login na primeira execução e, depois, só mantém a entrada que já existe
/// apontando para o executável atual (o AppImage pode ter sido movido). Se o usuário removeu a
/// entrada, ela não volta.
pub fn sincronizar(
    dir_autostart: &Path,
    execucao: &Execucao,
    primeira_execucao: bool,
) -> io::Result<()> {
    let arquivo = dir_autostart.join(ARQUIVO);
    if !primeira_execucao && !arquivo.exists() {
        return Ok(());
    }
    let alvo = execucao.appimage.as_ref().unwrap_or(&execucao.executavel);
    std::fs::create_dir_all(dir_autostart)?;
    std::fs::write(arquivo, entrada(alvo))
}

fn entrada(alvo: &Path) -> String {
    format!(
        "[Desktop Entry]\n\
         Type=Application\n\
         Name=kvojps-monitor\n\
         Exec={}\n\
         Icon=kvojps-monitor\n\
         Terminal=false\n\
         X-GNOME-Autostart-enabled=true\n",
        argumento_exec(alvo)
    )
}

/// Aspas e escapes da chave `Exec` (Desktop Entry Specification): primeiro a regra de aspas,
/// depois o escape de string do valor, que dobra as barras invertidas.
fn argumento_exec(caminho: &Path) -> String {
    let mut entre_aspas = String::from("\"");
    for c in caminho.to_string_lossy().chars() {
        match c {
            '"' | '`' | '$' | '\\' => {
                entre_aspas.push('\\');
                entre_aspas.push(c);
            }
            '%' => entre_aspas.push_str("%%"),
            _ => entre_aspas.push(c),
        }
    }
    entre_aspas.push('"');
    entre_aspas.replace('\\', "\\\\")
}

#[cfg(test)]
mod testes {
    use super::*;
    use std::fs;

    fn deb() -> Execucao {
        Execucao {
            executavel: "/usr/bin/kvojps-monitor".into(),
            appimage: None,
        }
    }

    fn appimage(caminho: &str) -> Execucao {
        Execucao {
            executavel: "/tmp/.mount_kvojpsXYZ/usr/bin/kvojps-monitor".into(),
            appimage: Some(caminho.into()),
        }
    }

    fn linha_exec(dir: &Path) -> String {
        let conteudo = fs::read_to_string(dir.join("kvojps-monitor.desktop")).unwrap();
        conteudo
            .lines()
            .find(|l| l.starts_with("Exec="))
            .unwrap()
            .to_string()
    }

    #[test]
    fn primeira_execucao_liga_o_inicio_com_o_login() {
        let d = tempfile::tempdir().unwrap();
        let dir = d.path().join("autostart");

        sincronizar(&dir, &deb(), true).unwrap();

        let conteudo = fs::read_to_string(dir.join("kvojps-monitor.desktop")).unwrap();
        assert!(conteudo.starts_with("[Desktop Entry]\n"));
        assert!(conteudo.contains("\nType=Application\n"));
        assert_eq!(linha_exec(&dir), "Exec=\"/usr/bin/kvojps-monitor\"");
    }

    #[test]
    fn no_appimage_aponta_para_o_arquivo_appimage_e_nao_para_o_ponto_de_montagem() {
        let d = tempfile::tempdir().unwrap();

        sincronizar(
            d.path(),
            &appimage("/home/example/Apps/kvojps-monitor.AppImage"),
            true,
        )
        .unwrap();

        assert_eq!(
            linha_exec(d.path()),
            "Exec=\"/home/example/Apps/kvojps-monitor.AppImage\""
        );
    }

    #[test]
    fn appimage_movido_reescreve_a_entrada_com_o_caminho_atual() {
        let d = tempfile::tempdir().unwrap();
        sincronizar(
            d.path(),
            &appimage("/home/example/Downloads/kvojps-monitor.AppImage"),
            true,
        )
        .unwrap();

        sincronizar(
            d.path(),
            &appimage("/home/example/Apps/kvojps-monitor.AppImage"),
            false,
        )
        .unwrap();

        assert_eq!(
            linha_exec(d.path()),
            "Exec=\"/home/example/Apps/kvojps-monitor.AppImage\""
        );
    }

    #[test]
    fn entrada_removida_pelo_usuario_nao_volta_depois_da_primeira_execucao() {
        let d = tempfile::tempdir().unwrap();
        sincronizar(d.path(), &appimage("/home/example/a.AppImage"), true).unwrap();
        fs::remove_file(d.path().join("kvojps-monitor.desktop")).unwrap();

        sincronizar(d.path(), &appimage("/home/example/a.AppImage"), false).unwrap();
        sincronizar(d.path(), &deb(), false).unwrap();

        assert!(!d.path().join("kvojps-monitor.desktop").exists());
    }

    #[test]
    fn caminho_com_espacos_e_caracteres_reservados_fica_entre_aspas_escapado() {
        let d = tempfile::tempdir().unwrap();

        sincronizar(
            d.path(),
            &appimage("/home/example/Meus Apps/$x 100%.AppImage"),
            true,
        )
        .unwrap();

        assert_eq!(
            linha_exec(d.path()),
            "Exec=\"/home/example/Meus Apps/\\\\$x 100%%.AppImage\""
        );
    }
}
