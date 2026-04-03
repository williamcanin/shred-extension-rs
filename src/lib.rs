#![allow(non_camel_case_types)]

use gio::prelude::FileExt;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uint, c_ulong, c_void};
use std::ptr;

// ----- i18n -----

struct I18n {
  menu_label: CString,
  menu_tip: CString,
  dialog_title: String,
  dialog_msg: fn(usize) -> String,
}

impl I18n {
  fn current() -> Self {
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
      }
    }
  }
}

// ----- Tipos GLib -----

type GType = c_ulong;
type gpointer = *mut c_void;
type gconstpointer = *const c_void;

#[repr(C)]
struct GList {
  data: gpointer,
  next: *mut GList,
  prev: *mut GList,
}
#[repr(C)]
struct GTypeInterface {
  g_type: GType,
  g_instance_type: GType,
}
#[repr(C)]
struct GTypeQuery {
  type_: GType,
  type_name: *const c_char,
  class_size: c_uint,
  instance_size: c_uint,
}

#[repr(C)]
struct GTypeInfo {
  class_size: u16,
  base_init: Option<unsafe extern "C" fn(gpointer)>,
  base_finalize: Option<unsafe extern "C" fn(gpointer)>,
  class_init: Option<unsafe extern "C" fn(gpointer, gpointer)>,
  class_finalize: Option<unsafe extern "C" fn(gpointer, gpointer)>,
  class_data: gconstpointer,
  instance_size: u16,
  n_preallocs: u16,
  instance_init: Option<unsafe extern "C" fn(gpointer, gpointer)>,
  value_table: gconstpointer,
}

#[repr(C)]
struct GInterfaceInfo {
  interface_init: Option<unsafe extern "C" fn(gpointer, gpointer)>,
  interface_finalize: Option<unsafe extern "C" fn(gpointer, gpointer)>,
  interface_data: gpointer,
}

// ----- Tipos opacos -----

enum FileInfo {}
enum MenuItem {}
enum MenuProvider {}
enum GtkWidget {}

// ----- VTable — layout diferente por feature -----

#[cfg(feature = "nautilus")]
#[repr(C)]
struct MenuProviderIface {
  g_iface: GTypeInterface,
  get_file_items: Option<unsafe extern "C" fn(*mut MenuProvider, *mut GList) -> *mut GList>,
  get_background_items:
    Option<unsafe extern "C" fn(*mut MenuProvider, *mut FileInfo) -> *mut GList>,
}

#[cfg(feature = "thunar")]
#[repr(C)]
struct MenuProviderIface {
  g_iface: GTypeInterface,
  get_file_items:
    Option<unsafe extern "C" fn(*mut MenuProvider, *mut GtkWidget, *mut GList) -> *mut GList>,
  get_background_items:
    Option<unsafe extern "C" fn(*mut MenuProvider, *mut GtkWidget, *mut FileInfo) -> *mut GList>,
}

// ----- Funções C externas -----

unsafe extern "C" {
  fn g_object_get_type() -> GType;
  fn g_type_query(type_: GType, query: *mut GTypeQuery);
  fn g_type_module_register_type(
    module: gpointer,
    parent_type: GType,
    type_name: *const c_char,
    info: *const GTypeInfo,
    flags: u32,
  ) -> GType;
  fn g_type_module_add_interface(
    module: gpointer,
    instance_type: GType,
    interface_type: GType,
    info: *const GInterfaceInfo,
  );
  fn g_list_append(list: *mut GList, data: gpointer) -> *mut GList;
  fn g_free(mem: gpointer);
  fn g_signal_connect_data(
    instance: gpointer,
    detailed_signal: *const c_char,
    c_handler: Option<unsafe extern "C" fn()>,
    data: gpointer,
    destroy_data: Option<unsafe extern "C" fn(gpointer, gpointer)>,
    connect_flags: u32,
  ) -> c_ulong;

  #[cfg(feature = "nautilus")]
  fn nautilus_menu_provider_get_type() -> GType;
  #[cfg(feature = "thunar")]
  fn thunarx_menu_provider_get_type() -> GType;

  #[cfg(feature = "nautilus")]
  fn nautilus_menu_item_new(
    name: *const c_char,
    label: *const c_char,
    tip: *const c_char,
    icon: *const c_char,
  ) -> *mut MenuItem;
  #[cfg(feature = "thunar")]
  fn thunarx_menu_item_new(
    name: *const c_char,
    label: *const c_char,
    tip: *const c_char,
    icon: *const c_char,
  ) -> *mut MenuItem;

  #[cfg(feature = "nautilus")]
  fn nautilus_file_info_get_uri(file: *mut FileInfo) -> *mut c_char;
  #[cfg(feature = "thunar")]
  fn thunarx_file_info_get_uri(file: *mut FileInfo) -> *mut c_char;
}

// ----- Wrappers para unificar os nomes -----

unsafe fn menu_provider_get_type() -> GType {
  unsafe {
    #[cfg(feature = "nautilus")]
    {
      nautilus_menu_provider_get_type()
    }
    #[cfg(feature = "thunar")]
    {
      thunarx_menu_provider_get_type()
    }
  }
}

unsafe fn menu_item_new(
  name: *const c_char,
  label: *const c_char,
  tip: *const c_char,
  icon: *const c_char,
) -> *mut MenuItem {
  unsafe {
    #[cfg(feature = "nautilus")]
    {
      nautilus_menu_item_new(name, label, tip, icon)
    }
    #[cfg(feature = "thunar")]
    {
      thunarx_menu_item_new(name, label, tip, icon)
    }
  }
}

unsafe fn file_info_get_uri(file: *mut FileInfo) -> *mut c_char {
  unsafe {
    #[cfg(feature = "nautilus")]
    {
      nautilus_file_info_get_uri(file)
    }
    #[cfg(feature = "thunar")]
    {
      thunarx_file_info_get_uri(file)
    }
  }
}

// ----- Estado do módulo -----

static mut MODULE_TYPES: [GType; 1] = [0];

// ----- get_file_items — assinatura diferente por feature -----

#[cfg(feature = "nautilus")]
unsafe extern "C" fn get_file_items(_provider: *mut MenuProvider, files: *mut GList) -> *mut GList {
  unsafe { get_file_items_impl(files) }
}

#[cfg(feature = "thunar")]
unsafe extern "C" fn get_file_items(
  _provider: *mut MenuProvider,
  _window: *mut GtkWidget,
  files: *mut GList,
) -> *mut GList {
  unsafe { get_file_items_impl(files) }
}

// ----- Lógica compartilhada -----

unsafe fn get_file_items_impl(files: *mut GList) -> *mut GList {
  unsafe {
    if files.is_null() {
      return ptr::null_mut();
    }

    let mut uris: Vec<String> = Vec::new();
    let mut node = files;
    while !node.is_null() {
      let fi = (*node).data as *mut FileInfo;
      if !fi.is_null() {
        let raw = file_info_get_uri(fi);
        if !raw.is_null() {
          uris.push(CStr::from_ptr(raw).to_string_lossy().into_owned());
          g_free(raw as gpointer);
        }
      }
      node = (*node).next;
    }

    if uris.is_empty() {
      return ptr::null_mut();
    }

    let name = CString::new("ShredRs::Shred").unwrap();
    let i18n = I18n::current();
    let icon = CString::new("edit-delete").unwrap();

    let item = menu_item_new(
      name.as_ptr(),
      i18n.menu_label.as_ptr(),
      i18n.menu_tip.as_ptr(),
      icon.as_ptr(),
    );

    let uris_ptr = Box::into_raw(Box::new(uris));
    let activate = CString::new("activate").unwrap();

    // Rust 2024: cast explícito em vez de transmute para fn pointers
    let handler: unsafe extern "C" fn(gpointer, gpointer) = on_activate;
    let handler_erased: unsafe extern "C" fn() = std::mem::transmute(handler);

    g_signal_connect_data(
      item as gpointer,
      activate.as_ptr(),
      Some(handler_erased),
      uris_ptr as gpointer,
      Some(free_uris),
      0,
    );

    g_list_append(ptr::null_mut(), item as gpointer)
  }
}

unsafe extern "C" fn on_activate(_item: gpointer, user_data: gpointer) {
  let uris = unsafe { &*(user_data as *const Vec<String>) }.clone();
  let i18n = I18n::current();

  std::thread::spawn(move || {
    let msg = (i18n.dialog_msg)(uris.len());
    let ok = std::process::Command::new("zenity")
      .args([
        "--question",
        "--width=450",
        &format!("--title={}", i18n.dialog_title),
        &format!("--text={}", msg),
        "--icon-name=edit-delete",
      ])
      .status()
      .map_or(false, |s| s.success());

    if ok {
      for uri in &uris {
        let file = gio::File::for_uri(uri);
        let Some(path) = file.path() else { continue };
        if let (Some(dir), Some(name)) = (path.parent(), path.file_name()) {
          let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
          let hidden_dir = dir.join(format!(".~shred_{}_{}", name.to_string_lossy(), nanos));
          if std::fs::create_dir(&hidden_dir).is_ok() {
            let dest = hidden_dir.join(name);
            if std::fs::rename(&path, &dest).is_ok() {
              let _ = std::process::Command::new("shred")
                .args(["-u", "-n", "3", "-z"])
                .arg(&dest)
                .status();
              let _ = std::fs::remove_dir(&hidden_dir);
            } else {
              let _ = std::fs::remove_dir(&hidden_dir);
            }
          } else {
            let hidden_file = dir.join(format!(".~shred_{}", name.to_string_lossy()));
            if std::fs::rename(&path, &hidden_file).is_ok() {
              let _ = std::process::Command::new("shred")
                .args(["-n", "3", "-z"])
                .arg(&hidden_file)
                .status();
              let _ = std::fs::remove_file(&hidden_file);
            }
          }
        }
      }
    }
  });
}

unsafe extern "C" fn free_uris(data: gpointer, _: gpointer) {
  if !data.is_null() {
    unsafe {
      drop(Box::from_raw(data as *mut Vec<String>));
    }
  }
}

unsafe extern "C" fn menu_provider_iface_init(iface: gpointer, _: gpointer) {
  unsafe {
    let iface = &mut *(iface as *mut MenuProviderIface);
    iface.get_file_items = Some(get_file_items);
    iface.get_background_items = None;
  }
}

unsafe fn initialize_impl(module: gpointer) {
  unsafe {
    let gobject_type = g_object_get_type();
    let mut query = GTypeQuery {
      type_: 0,
      type_name: ptr::null(),
      class_size: 0,
      instance_size: 0,
    };
    g_type_query(gobject_type, &mut query);
    let type_info = GTypeInfo {
      class_size: query.class_size as u16,
      base_init: None,
      base_finalize: None,
      class_init: None,
      class_finalize: None,
      class_data: ptr::null(),
      instance_size: query.instance_size as u16,
      n_preallocs: 0,
      instance_init: None,
      value_table: ptr::null(),
    };
    let type_name = c"ShredProvider";
    let shred_type =
      g_type_module_register_type(module, gobject_type, type_name.as_ptr(), &type_info, 0);
    let iface_info = GInterfaceInfo {
      interface_init: Some(menu_provider_iface_init),
      interface_finalize: None,
      interface_data: ptr::null_mut(),
    };
    g_type_module_add_interface(module, shred_type, menu_provider_get_type(), &iface_info);
    *std::ptr::addr_of_mut!(MODULE_TYPES[0]) = shred_type;
  }
}

// ----- Pontos de entrada Nautilus -----

#[cfg(feature = "nautilus")]
#[unsafe(no_mangle)]
pub extern "C" fn nautilus_module_initialize(module: gpointer) {
  unsafe {
    initialize_impl(module);
  }
}

#[cfg(feature = "nautilus")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nautilus_module_list_types(
  types: *mut *const GType,
  num_types: *mut c_int,
) {
  unsafe {
    *types = std::ptr::addr_of!(MODULE_TYPES) as *const GType;
    *num_types = 1;
  }
}

#[cfg(feature = "nautilus")]
#[unsafe(no_mangle)]
pub extern "C" fn nautilus_module_shutdown() {}

// ----- Pontos de entrada Thunar -----

#[cfg(feature = "thunar")]
#[unsafe(no_mangle)]
pub extern "C" fn thunar_extension_initialize(module: gpointer) {
  unsafe {
    initialize_impl(module);
  }
}

#[cfg(feature = "thunar")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thunar_extension_list_types(
  types: *mut *const GType,
  num_types: *mut c_int,
) {
  unsafe {
    *types = std::ptr::addr_of!(MODULE_TYPES) as *const GType;
    *num_types = 1;
  }
}

#[cfg(feature = "thunar")]
#[unsafe(no_mangle)]
pub extern "C" fn thunar_extension_shutdown() {}
