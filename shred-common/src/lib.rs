#![allow(non_camel_case_types)]
use std::ffi::CString;
use std::os::raw::{c_char, c_uint, c_ulong, c_void};

// ----- i18n -----

pub struct I18n {
  pub menu_label: CString,
  pub menu_tip: CString,
  pub dialog_title: String,
  pub dialog_msg: fn(usize) -> String,
  pub trash_menu_label: CString,
  pub trash_menu_tip: CString,
  pub trash_dialog_title: String,
  pub trash_dialog_msg: String,
}

impl I18n {
  pub fn current() -> Self {
    let lang = std::env::var("LANG").unwrap_or_else(|_| "en".to_string());
    if lang.starts_with("pt") {
      Self {
        menu_label: CString::new("Excluir com Segurança").unwrap(),
        menu_tip: CString::new("Apagar permanentemente usando shred").unwrap(),
        dialog_title: "Excluir com Segurança".to_string(),
        dialog_msg: |count| {
          format!(
            "Deseja realmente excluir permanentemente {} arquivo(s)?\n\nEssa ação é irreversível e o(s) arquivo(s) será(ão) sobrescrito(s) múltiplas vezes para impossibilitar a recuperação.",
            count
          )
        },
        trash_menu_label: CString::new("Esvaziar Lixeira com Segurança").unwrap(),
        trash_menu_tip: CString::new("Esvaziar lixeira permanentemente usando shred").unwrap(),
        trash_dialog_title: "Esvaziar Lixeira com Segurança".to_string(),
        trash_dialog_msg: "Deseja realmente esvaziar a lixeira com segurança?\n\nEssa ação é irreversível e todos os arquivos serão sobrescritos múltiplas vezes para impossibilitar a recuperação.".to_string(),
      }
    } else if lang.starts_with("es") {
      Self {
        menu_label: CString::new("Eliminar de forma segura").unwrap(),
        menu_tip: CString::new("Borrar de forma permanente usando shred").unwrap(),
        dialog_title: "Eliminación Segura".to_string(),
        dialog_msg: |count| {
          format!(
            "¿Realmente desea eliminar permanentemente {} archivo(s)?\n\nEsta acción es irreversible y el archivo se sobrescribirá varias veces para imposibilitar su recuperación.",
            count
          )
        },
        trash_menu_label: CString::new("Vaciar Papelera de Forma Segura").unwrap(),
        trash_menu_tip: CString::new("Vaciar papelera permanentemente usando shred").unwrap(),
        trash_dialog_title: "Vaciar Papelera de Forma Segura".to_string(),
        trash_dialog_msg: "¿Realmente desea vaciar la papelera de forma segura?\n\nEsta acción es irreversible y todos los archivos se sobrescribirán varias veces para imposibilitar su recuperación.".to_string(),
      }
    } else {
      Self {
        menu_label: CString::new("Secure Delete").unwrap(),
        menu_tip: CString::new("Permanently delete using shred").unwrap(),
        dialog_title: "Secure Delete".to_string(),
        dialog_msg: |count| {
          format!(
            "Do you really want to permanently delete {} file(s)?\n\nThis action is irreversible and the file will be overwritten multiple times to prevent recovery.",
            count
          )
        },
        trash_menu_label: CString::new("Secure Empty Trash").unwrap(),
        trash_menu_tip: CString::new("Permanently empty trash using shred").unwrap(),
        trash_dialog_title: "Secure Empty Trash".to_string(),
        trash_dialog_msg: "Do you really want to securely empty the trash?\n\nThis action is irreversible and all files will be overwritten multiple times to prevent recovery.".to_string(),
      }
    }
  }
}

// ----- Tipos GLib -----

pub type GType = c_ulong;
pub type gpointer = *mut c_void;
pub type gconstpointer = *const c_void;

#[repr(C)]
pub struct GList {
  pub data: gpointer,
  pub next: *mut GList,
  pub prev: *mut GList,
}
#[repr(C)]
pub struct GTypeInterface {
  pub g_iface: GTypeInterfaceStruct,
}
#[repr(C)]
pub struct GTypeInterfaceStruct {
  pub g_type: GType,
  pub g_instance_type: GType,
}
#[repr(C)]
pub struct GTypeQuery {
  pub type_: GType,
  pub type_name: *const c_char,
  pub class_size: c_uint,
  pub instance_size: c_uint,
}

#[repr(C)]
pub struct GTypeInfo {
  pub class_size: u16,
  pub base_init: Option<unsafe extern "C" fn(gpointer)>,
  pub base_finalize: Option<unsafe extern "C" fn(gpointer)>,
  pub class_init: Option<unsafe extern "C" fn(gpointer, gpointer)>,
  pub class_finalize: Option<unsafe extern "C" fn(gpointer, gpointer)>,
  pub class_data: gconstpointer,
  pub instance_size: u16,
  pub n_preallocs: u16,
  pub instance_init: Option<unsafe extern "C" fn(gpointer, gpointer)>,
  pub value_table: gconstpointer,
}

#[repr(C)]
pub struct GInterfaceInfo {
  pub interface_init: Option<unsafe extern "C" fn(gpointer, gpointer)>,
  pub interface_finalize: Option<unsafe extern "C" fn(gpointer, gpointer)>,
  pub interface_data: gpointer,
}

// ----- Tipos opacos -----

pub enum FileInfo {}
pub enum MenuItem {}
pub enum MenuProvider {}
pub enum GtkWidget {}

// ----- Funções C externas -----

unsafe extern "C" {
  pub fn g_object_get_type() -> GType;
  pub fn g_type_query(type_: GType, query: *mut GTypeQuery);
  pub fn g_type_module_register_type(
    module: gpointer,
    parent_type: GType,
    type_name: *const c_char,
    info: *const GTypeInfo,
    flags: u32,
  ) -> GType;
  pub fn g_type_module_add_interface(
    module: gpointer,
    instance_type: GType,
    interface_type: GType,
    info: *const GInterfaceInfo,
  );
  pub fn g_list_append(list: *mut GList, data: gpointer) -> *mut GList;
  pub fn g_free(mem: gpointer);
  pub fn g_signal_connect_data(
    instance: gpointer,
    detailed_signal: *const c_char,
    c_handler: Option<unsafe extern "C" fn()>,
    data: gpointer,
    destroy_data: Option<unsafe extern "C" fn(gpointer, gpointer)>,
    connect_flags: u32,
  ) -> c_ulong;
}

// ----- Lógica de Destruição (Shredding) -----

pub fn secure_erase_path(path: &std::path::Path) {
  if !path.exists() {
    return;
  }

  let (Some(dir), Some(name)) = (path.parent(), path.file_name()) else {
    if path.is_file() {
      let _ = std::process::Command::new("shred")
        .args(["-u", "-n", "3", "-z"])
        .arg(path)
        .status();
    }
    return;
  };

  let nanos = std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .unwrap_or_default()
    .as_nanos();

  if path.is_dir() {
    let hidden_dir = dir.join(format!(".~shred_dir_{}_{}", name.to_string_lossy(), nanos));
    let working_path = if std::fs::rename(path, &hidden_dir).is_ok() {
      hidden_dir
    } else {
      path.to_path_buf()
    };

    secure_erase_contents_recursive(&working_path);
    let _ = std::fs::remove_dir_all(&working_path);
  } else {
    let hidden_dir = dir.join(format!(".~shred_{}_{}", name.to_string_lossy(), nanos));
    if std::fs::create_dir(&hidden_dir).is_ok() {
      let dest = hidden_dir.join(name);
      if std::fs::rename(path, &dest).is_ok() {
        let _ = std::process::Command::new("shred")
          .args(["-u", "-n", "3", "-z"])
          .arg(&dest)
          .status();
        let _ = std::fs::remove_dir(&hidden_dir);
      } else {
        let _ = std::fs::remove_dir(&hidden_dir);
        let _ = std::process::Command::new("shred")
          .args(["-u", "-n", "3", "-z"])
          .arg(path)
          .status();
      }
    } else {
      let hidden_file = dir.join(format!(".~shred_{}", name.to_string_lossy()));
      if std::fs::rename(path, &hidden_file).is_ok() {
        let _ = std::process::Command::new("shred")
          .args(["-u", "-n", "3", "-z"])
          .arg(&hidden_file)
          .status();
      } else {
        let _ = std::process::Command::new("shred")
          .args(["-u", "-n", "3", "-z"])
          .arg(path)
          .status();
      }
    }
  }
}

fn secure_erase_contents_recursive(path: &std::path::Path) {
  if let Ok(entries) = std::fs::read_dir(path) {
    for entry in entries.flatten() {
      let p = entry.path();
      if p.is_dir() {
        secure_erase_contents_recursive(&p);
        let _ = std::fs::remove_dir(&p);
      } else {
        let _ = std::process::Command::new("shred")
          .args(["-u", "-n", "3", "-z"])
          .arg(&p)
          .status();
      }
    }
  }
}

pub unsafe extern "C" fn free_uris(data: gpointer, _: gpointer) {
  if !data.is_null() {
    unsafe {
      drop(Box::from_raw(data as *mut Vec<String>));
    }
  }
}
