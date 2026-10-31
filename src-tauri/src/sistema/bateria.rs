//! Bateria via UPower `DisplayDevice` (D-Bus de sistema).

use std::io;
use std::thread;

use serde::Serialize;
use zbus::blocking::fdo::PropertiesProxy;
use zbus::blocking::Connection;
use zbus::names::InterfaceName;

/// Propriedades de `org.freedesktop.UPower.Device` que o Módulo usa.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DisplayDevice {
    pub tipo: u32,
    pub presente: bool,
    pub percentual: f64,
    pub estado: u32,
    /// Segundos; 0 quando o UPower não sabe.
    pub ate_esvaziar: i64,
    pub ate_encher: i64,
}

/// Porta D-Bus para o UPower.
pub trait UPower: Send {
    fn display_device(&self) -> io::Result<DisplayDevice>;
}

/// `Type` do UPower para bateria.
const TIPO_BATERIA: u32 = 2;
const PERCENTUAL_DE_ATENCAO: f64 = 15.0;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Bateria {
    pub percentual: f64,
    pub estado: EstadoBateria,
    /// Até esvaziar ou encher, quando o UPower informa.
    pub segundos_restantes: Option<i64>,
    /// Abaixo de 15%.
    pub atencao: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum EstadoBateria {
    Carregando,
    Descarregando,
    Cheia,
    Vazia,
    /// Na tomada sem carregar (ex.: limite de carga).
    NaTomada,
    Desconhecido,
}

/// Lê o `DisplayDevice`; None sem bateria ou sem UPower.
pub fn ler(upower: &impl UPower) -> Option<Bateria> {
    let d = upower.display_device().ok()?;
    if d.tipo != TIPO_BATERIA || !d.presente {
        return None;
    }
    let estado = match d.estado {
        1 => EstadoBateria::Carregando,
        2 | 6 => EstadoBateria::Descarregando,
        3 => EstadoBateria::Vazia,
        4 => EstadoBateria::Cheia,
        5 => EstadoBateria::NaTomada,
        _ => EstadoBateria::Desconhecido,
    };
    let restantes = match estado {
        EstadoBateria::Carregando => d.ate_encher,
        EstadoBateria::Descarregando => d.ate_esvaziar,
        _ => 0,
    };
    Some(Bateria {
        percentual: d.percentual.round(),
        estado,
        segundos_restantes: (restantes > 0).then_some(restantes),
        atencao: d.percentual < PERCENTUAL_DE_ATENCAO,
    })
}

const DESTINO: &str = "org.freedesktop.UPower";
const CAMINHO: &str = "/org/freedesktop/UPower/devices/DisplayDevice";
const INTERFACE: &str = "org.freedesktop.UPower.Device";

/// O UPower real, no D-Bus de sistema. Sem barramento, toda leitura falha e o bloco some.
pub struct UPowerDbus(Option<Connection>);

impl UPowerDbus {
    pub fn conectar() -> Self {
        UPowerDbus(Connection::system().ok())
    }
}

fn propriedades(conexao: &Connection) -> zbus::Result<PropertiesProxy<'static>> {
    PropertiesProxy::builder(conexao)
        .destination(DESTINO)?
        .path(CAMINHO)?
        .build()
}

impl UPower for UPowerDbus {
    fn display_device(&self) -> io::Result<DisplayDevice> {
        let conexao = self
            .0
            .as_ref()
            .ok_or_else(|| io::Error::from(io::ErrorKind::NotConnected))?;
        let todas = propriedades(conexao)
            .map_err(io::Error::other)?
            .get_all(InterfaceName::from_static_str_unchecked(INTERFACE))
            .map_err(io::Error::other)?;
        let campo = |nome: &str| {
            todas
                .get(nome)
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, nome.to_owned()))
        };
        let converter = |e: zbus::zvariant::Error| io::Error::new(io::ErrorKind::InvalidData, e);
        Ok(DisplayDevice {
            tipo: campo("Type")?.downcast_ref().map_err(converter)?,
            presente: campo("IsPresent")?.downcast_ref().map_err(converter)?,
            percentual: campo("Percentage")?.downcast_ref().map_err(converter)?,
            estado: campo("State")?.downcast_ref().map_err(converter)?,
            ate_esvaziar: campo("TimeToEmpty")?.downcast_ref().map_err(converter)?,
            ate_encher: campo("TimeToFull")?.downcast_ref().map_err(converter)?,
        })
    }
}

/// Chama `aviso` a cada `PropertiesChanged` do `DisplayDevice`, numa thread própria.
pub fn ouvir_upower(aviso: impl Fn() + Send + 'static) {
    let ouvir = move || -> zbus::Result<()> {
        let conexao = Connection::system()?;
        for _ in propriedades(&conexao)?.receive_properties_changed()? {
            aviso();
        }
        Ok(())
    };
    let _ = thread::Builder::new().name("upower".into()).spawn(move || {
        if let Err(e) = ouvir() {
            eprintln!("sistema: sem sinais do UPower: {e}");
        }
    });
}

#[cfg(test)]
pub mod falso {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};

    use super::*;
    use crate::portas::falsos::fixtures;

    /// UPower falso que responde a gravação de `busctl -j ... GetAll` em `fixtures/sistema/upower/`.
    #[derive(Clone)]
    pub struct UPowerFalso {
        resposta: Arc<Mutex<io::Result<DisplayDevice>>>,
        pub leituras: Arc<AtomicUsize>,
    }

    impl UPowerFalso {
        pub fn de(gravacao: &str) -> Self {
            let falso = UPowerFalso {
                resposta: Arc::new(Mutex::new(Ok(Default::default()))),
                leituras: Default::default(),
            };
            falso.responder(gravacao);
            falso
        }

        pub fn responder(&self, gravacao: &str) {
            *self.resposta.lock().unwrap() = Ok(ler_gravacao(gravacao));
        }

        pub fn fora_do_ar(&self) {
            *self.resposta.lock().unwrap() = Err(io::Error::from(io::ErrorKind::ConnectionRefused));
        }
    }

    impl UPower for UPowerFalso {
        fn display_device(&self) -> io::Result<DisplayDevice> {
            self.leituras.fetch_add(1, Ordering::SeqCst);
            match &*self.resposta.lock().unwrap() {
                Ok(d) => Ok(d.clone()),
                Err(e) => Err(io::Error::from(e.kind())),
            }
        }
    }

    fn ler_gravacao(nome: &str) -> DisplayDevice {
        let caminho = fixtures().join(format!("sistema/upower/{nome}.json"));
        let json: serde_json::Value =
            serde_json::from_slice(&std::fs::read(caminho).unwrap()).unwrap();
        let props = &json["data"][0];
        let v = |chave: &str| &props[chave]["data"];
        DisplayDevice {
            tipo: v("Type").as_u64().unwrap() as u32,
            presente: v("IsPresent").as_bool().unwrap(),
            percentual: v("Percentage").as_f64().unwrap(),
            estado: v("State").as_u64().unwrap() as u32,
            ate_esvaziar: v("TimeToEmpty").as_i64().unwrap(),
            ate_encher: v("TimeToFull").as_i64().unwrap(),
        }
    }
}
