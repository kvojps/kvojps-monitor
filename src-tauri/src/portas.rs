//! Portas de E/S do núcleo. Os coletores só falam com o mundo por elas; os testes as alimentam
//! com saídas reais gravadas em `fixtures/` e com um relógio falso.

use std::io;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

/// Espaço de um sistema de arquivos, em bytes, como o `statvfs` informa.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Espaco {
    pub total: u64,
    /// Livre para qualquer processo, inclusive os blocos reservados ao root.
    pub livre: u64,
    /// Livre para o usuário: o que o `df` mostra como disponível.
    pub disponivel: u64,
}

/// Sistema de arquivos visto a partir de uma raiz. Caminhos são absolutos (`/proc/stat`).
pub trait Arquivos: Send {
    fn ler(&self, caminho: &str) -> io::Result<String>;
    /// Nomes das entradas de um diretório.
    fn listar(&self, dir: &str) -> io::Result<Vec<String>>;
    fn espaco(&self, ponto_de_montagem: &str) -> io::Result<Espaco>;
}

/// O sistema de arquivos real sob `dir` (`/` em produção).
pub struct Raiz {
    dir: PathBuf,
}

impl Raiz {
    pub fn sistema() -> Self {
        Raiz::em("/")
    }

    pub fn em(dir: impl Into<PathBuf>) -> Self {
        Raiz { dir: dir.into() }
    }

    fn caminho(&self, caminho: &str) -> PathBuf {
        self.dir.join(caminho.trim_start_matches('/'))
    }
}

impl Arquivos for Raiz {
    fn ler(&self, caminho: &str) -> io::Result<String> {
        std::fs::read_to_string(self.caminho(caminho))
    }

    fn listar(&self, dir: &str) -> io::Result<Vec<String>> {
        std::fs::read_dir(self.caminho(dir))?
            .map(|e| Ok(e?.file_name().to_string_lossy().into_owned()))
            .collect()
    }

    fn espaco(&self, ponto_de_montagem: &str) -> io::Result<Espaco> {
        let s = rustix::fs::statvfs(self.caminho(ponto_de_montagem))?;
        Ok(Espaco {
            total: s.f_blocks * s.f_frsize,
            livre: s.f_bfree * s.f_frsize,
            disponivel: s.f_bavail * s.f_frsize,
        })
    }
}

/// Hora atual em epoch ms.
pub trait Relogio: Send {
    fn agora(&self) -> u64;
}

pub struct RelogioDoSistema;

impl Relogio for RelogioDoSistema {
    fn agora(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |d| d.as_millis() as u64)
    }
}

#[cfg(test)]
pub mod falsos {
    use std::collections::HashMap;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::{Arc, Mutex};

    use std::path::Path;

    use super::*;

    pub fn fixtures() -> &'static Path {
        Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures"))
    }

    /// Raiz falsa: cópia de uma fixture num diretório temporário, que o teste pode alterar, e
    /// `statvfs` respondido de uma tabela.
    #[derive(Clone)]
    pub struct RaizFalsa {
        pub dir: Arc<tempfile::TempDir>,
        espacos: Arc<Mutex<HashMap<String, Espaco>>>,
    }

    impl RaizFalsa {
        /// Copia `fixtures/<cenario>` para uma raiz nova.
        pub fn de(cenario: &str) -> Self {
            let dir = tempfile::tempdir().unwrap();
            copiar(&fixtures().join(cenario), dir.path());
            RaizFalsa {
                dir: Arc::new(dir),
                espacos: Default::default(),
            }
        }

        pub fn escrever(&self, caminho: &str, conteudo: &str) {
            let destino = self.dir.path().join(caminho.trim_start_matches('/'));
            std::fs::create_dir_all(destino.parent().unwrap()).unwrap();
            std::fs::write(destino, conteudo).unwrap();
        }

        pub fn apagar(&self, caminho: &str) {
            let _ = std::fs::remove_dir_all(self.dir.path().join(caminho.trim_start_matches('/')));
        }

        pub fn definir_espaco(&self, ponto: &str, espaco: Espaco) {
            self.espacos.lock().unwrap().insert(ponto.into(), espaco);
        }
    }

    impl Arquivos for RaizFalsa {
        fn ler(&self, caminho: &str) -> io::Result<String> {
            Raiz::em(self.dir.path()).ler(caminho)
        }

        fn listar(&self, dir: &str) -> io::Result<Vec<String>> {
            Raiz::em(self.dir.path()).listar(dir)
        }

        fn espaco(&self, ponto: &str) -> io::Result<Espaco> {
            self.espacos
                .lock()
                .unwrap()
                .get(ponto)
                .copied()
                .ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))
        }
    }

    fn copiar(origem: &Path, destino: &Path) {
        std::fs::create_dir_all(destino).unwrap();
        for entrada in std::fs::read_dir(origem).unwrap() {
            let entrada = entrada.unwrap();
            let alvo = destino.join(entrada.file_name());
            if entrada.file_type().unwrap().is_dir() {
                copiar(&entrada.path(), &alvo);
            } else {
                std::fs::copy(entrada.path(), alvo).unwrap();
            }
        }
    }

    #[derive(Clone, Default)]
    pub struct RelogioFalso(Arc<AtomicU64>);

    impl RelogioFalso {
        pub fn em(ms: u64) -> Self {
            RelogioFalso(Arc::new(AtomicU64::new(ms)))
        }

        pub fn avancar(&self, ms: u64) {
            self.0.fetch_add(ms, Ordering::SeqCst);
        }
    }

    impl Relogio for RelogioFalso {
        fn agora(&self) -> u64 {
            self.0.load(Ordering::SeqCst)
        }
    }
}
