use shred_common::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::ptr;
use gio::prelude::FileExt;

#[repr(C)]
struct NautilusMenuProviderIface {
  g_iface: GTypeInterfaceStruct,
  get_file_items: Option<unsafe extern "C" fn(*mut MenuProvider, *mut GList) -> *mut GList>,
  get_background_items: Option<unsafe extern "C" fn(*mut MenuProvider, *mut FileInfo) -> *mut GList>,
}

unsafe extern "C" {
  fn nautilus_menu_provider_get_type() -> GType;
  fn nautilus_menu_item_new(
    name: *const c_char,
    label: *const c_char,
    tip: *const c_char,
    icon: *const c_char,
  ) -> *mut MenuItem;
  fn nautilus_file_info_get_uri(file: *mut FileInfo) -> *mut c_char;
}

static mut MODULE_TYPES: [GType; 1] = [0];

unsafe extern "C" fn nautilus_get_file_items(
  _provider: *mut MenuProvider,
  files: *mut GList,
) -> *mut GList {
  unsafe {
    if files.is_null() {
      return ptr::null_mut();
    }

    let mut uris: Vec<String> = Vec::new();
    let mut node = files;
    while !node.is_null() {
      let fi = (*node).data as *mut FileInfo;
      if !fi.is_null() {
        let raw = nautilus_file_info_get_uri(fi);
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

    let is_trash = uris.len() == 1 && uris[0] == "trash:///";
    let i18n = I18n::current();
    let name = CString::new(if is_trash { "ShredRs::SecureEmptyTrash" } else { "ShredRs::Shred" }).unwrap();
    let icon = CString::new(if is_trash { "user-trash" } else { "edit-delete" }).unwrap();
    let label = if is_trash { i18n.trash_menu_label.as_ptr() } else { i18n.menu_label.as_ptr() };
    let tip = if is_trash { i18n.trash_menu_tip.as_ptr() } else { i18n.menu_tip.as_ptr() };

    let item = nautilus_menu_item_new(name.as_ptr(), label, tip, icon.as_ptr());
    let uris_ptr = Box::into_raw(Box::new(uris));
    let activate = CString::new("activate").unwrap();

    g_signal_connect_data(
      item as gpointer,
      activate.as_ptr(),
      Some(std::mem::transmute(on_activate as unsafe extern "C" fn(gpointer, gpointer))),
      uris_ptr as gpointer,
      Some(free_uris),
      0,
    );

    g_list_append(ptr::null_mut(), item as gpointer)
  }
}

unsafe extern "C" fn nautilus_get_background_items(
  _provider: *mut MenuProvider,
  file: *mut FileInfo,
) -> *mut GList {
  unsafe {
    if file.is_null() {
      return ptr::null_mut();
    }
    let raw = nautilus_file_info_get_uri(file);
    if raw.is_null() {
      return ptr::null_mut();
    }
    let uri = CStr::from_ptr(raw).to_string_lossy().into_owned();
    g_free(raw as gpointer);

    if uri != "trash:///" {
      return ptr::null_mut();
    }

    let i18n = I18n::current();
    let name = CString::new("ShredRs::SecureEmptyTrash").unwrap();
    let icon = CString::new("user-trash").unwrap();
    let item = nautilus_menu_item_new(
      name.as_ptr(),
      i18n.trash_menu_label.as_ptr(),
      i18n.trash_menu_tip.as_ptr(),
      icon.as_ptr(),
    );

    let uris: Vec<String> = vec!["trash:///".to_string()];
    let uris_ptr = Box::into_raw(Box::new(uris));
    let activate = CString::new("activate").unwrap();

    g_signal_connect_data(
      item as gpointer,
      activate.as_ptr(),
      Some(std::mem::transmute(on_activate as unsafe extern "C" fn(gpointer, gpointer))),
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
  let is_trash = uris.len() == 1 && uris[0] == "trash:///";

  std::thread::spawn(move || {
    let msg = if is_trash { i18n.trash_dialog_msg.clone() } else { (i18n.dialog_msg)(uris.len()) };
    let title = if is_trash { &i18n.trash_dialog_title } else { &i18n.dialog_title };

    let ok = std::process::Command::new("zenity")
      .args([
        "--question",
        "--width=450",
        &format!("--title={}", title),
        &format!("--text={}", msg),
        "--icon-name=edit-delete",
      ])
      .status()
      .map_or(false, |s| s.success());

    if ok {
      if is_trash {
        let trash = gio::File::for_uri("trash:///");
        if let Ok(enumerator) = trash.enumerate_children("*", gio::FileQueryInfoFlags::NONE, gio::Cancellable::NONE) {
          for info in enumerator.flatten() {
            let child = trash.child(info.name());
            if let Some(target_uri) = info.attribute_as_string("standard::target-uri") {
              if let Some(target_file) = gio::File::for_uri(&target_uri).path() {
                secure_erase_path(&target_file);
              }
            }
            let _ = child.delete(gio::Cancellable::NONE);
          }
        }
      } else {
        for uri in &uris {
          let file = gio::File::for_uri(uri);
          if let Some(path) = file.path() {
            secure_erase_path(&path);
          }
        }
      }
    }
  });
}

unsafe extern "C" fn nautilus_menu_provider_iface_init(iface: gpointer, _: gpointer) {
  unsafe {
    let iface = &mut *(iface as *mut NautilusMenuProviderIface);
    iface.get_file_items = Some(nautilus_get_file_items);
    iface.get_background_items = Some(nautilus_get_background_items);
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn nautilus_module_initialize(module: gpointer) {
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

    let type_name = CString::new("ShredProvider").unwrap();
    let shred_type = g_type_module_register_type(module, gobject_type, type_name.as_ptr(), &type_info, 0);

    let iface_info = GInterfaceInfo {
      interface_init: Some(nautilus_menu_provider_iface_init),
      interface_finalize: None,
      interface_data: ptr::null_mut(),
    };
    g_type_module_add_interface(module, shred_type, nautilus_menu_provider_get_type(), &iface_info);
    MODULE_TYPES[0] = shred_type;
  }
}

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

#[unsafe(no_mangle)]
pub extern "C" fn nautilus_module_shutdown() {}
